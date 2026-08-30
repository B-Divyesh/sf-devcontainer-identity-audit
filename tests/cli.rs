use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

fn binary() -> &'static str {
    env!("CARGO_BIN_EXE_mount-identity-audit")
}

#[test]
fn demo_runs_the_bundled_sample_in_a_temporary_project() {
    let shipped = fs::read("examples/mismatch/.devcontainer/devcontainer.json").unwrap();
    let output = Command::new(binary()).arg("--demo").output().unwrap();

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("DEMO — bundled sample data"));
    assert!(stdout.contains("Sample copy:"));
    assert!(stdout.contains("MOUNT IDENTITY AUDIT"));
    assert!(stdout.contains("FAIL"));
    assert_eq!(
        fs::read("examples/mismatch/.devcontainer/devcontainer.json").unwrap(),
        shipped
    );
}

#[test]
fn documented_no_runtime_example_passes_for_owner() {
    let temp = tempfile::tempdir().unwrap();
    fs::create_dir(temp.path().join(".devcontainer")).unwrap();
    fs::write(
        temp.path().join(".devcontainer/devcontainer.json"),
        "{ remoteUser: \"0:0\" }",
    )
    .unwrap();
    let output = Command::new(binary())
        .arg(temp.path())
        .args(["--runtime", "docker", "--no-runtime", "--json"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["schema_version"], 1);
    assert_eq!(json["verdict"], "pass");
}

#[test]
fn reports_definite_permission_failure() {
    let temp = tempfile::tempdir().unwrap();
    fs::create_dir(temp.path().join(".devcontainer")).unwrap();
    fs::write(
        temp.path().join(".devcontainer/devcontainer.json"),
        "{ remoteUser: \"424242:424242\" }",
    )
    .unwrap();
    fs::set_permissions(temp.path(), fs::Permissions::from_mode(0o755)).unwrap();
    let output = Command::new(binary())
        .arg(temp.path())
        .args(["--runtime", "docker", "--no-runtime", "--quiet"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stdout).starts_with("FAIL:"));
}

#[test]
fn reserved_linux_identity_is_unknown_in_direct_docker_mode() {
    let temp = tempfile::tempdir().unwrap();
    fs::create_dir(temp.path().join(".devcontainer")).unwrap();
    fs::write(
        temp.path().join(".devcontainer/devcontainer.json"),
        "{ remoteUser: \"4294967295:4294967295\" }",
    )
    .unwrap();
    fs::set_permissions(temp.path(), fs::Permissions::from_mode(0o777)).unwrap();

    let output = Command::new(binary())
        .arg(temp.path())
        .args(["--runtime", "docker", "--no-runtime", "--json"])
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(2));
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["verdict"], "unknown");
    assert_eq!(json["identity"]["container_uid"], serde_json::Value::Null);
    assert!(json["summary"].as_str().unwrap().contains("reserved"));
    assert!(
        json["remediations"]
            .to_string()
            .contains("below 4294967295")
    );
}

#[test]
fn reserved_linux_identity_is_unknown_in_podman_host_mode() {
    let temp = tempfile::tempdir().unwrap();
    fs::create_dir(temp.path().join(".devcontainer")).unwrap();
    fs::write(
        temp.path().join(".devcontainer/devcontainer.json"),
        r#"{ remoteUser: "4294967295:4294967295", runArgs: ["--userns", "host"] }"#,
    )
    .unwrap();
    fs::set_permissions(temp.path(), fs::Permissions::from_mode(0o777)).unwrap();

    let output = Command::new(binary())
        .arg(temp.path())
        .args(["--runtime", "podman", "--no-runtime", "--json"])
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(2));
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["verdict"], "unknown");
    assert!(json["summary"].as_str().unwrap().contains("reserved"));
}

fn run_compose_identity_precedence_case(
    remote_user: &str,
    compose_user: &str,
) -> std::process::Output {
    let temp = tempfile::tempdir().unwrap();
    fs::create_dir(temp.path().join(".devcontainer")).unwrap();
    fs::write(
        temp.path().join(".devcontainer/devcontainer.json"),
        format!(
            r#"{{
                dockerComposeFile: "compose.yml",
                service: "app",
                remoteUser: "{remote_user}",
                workspaceFolder: "/work"
            }}"#
        ),
    )
    .unwrap();
    fs::write(
        temp.path().join(".devcontainer/compose.yml"),
        format!(
            "services:\n  app:\n    image: local/example:latest\n    user: \"{compose_user}\"\n    volumes:\n      - ../:/work\n"
        ),
    )
    .unwrap();
    fs::set_permissions(temp.path(), fs::Permissions::from_mode(0o755)).unwrap();

    Command::new(binary())
        .arg(temp.path())
        .args(["--runtime", "docker", "--no-runtime", "--json"])
        .output()
        .unwrap()
}

#[test]
fn explicit_remote_user_wins_over_root_compose_user() {
    let output = run_compose_identity_precedence_case("424242:424242", "0:0");

    assert_eq!(output.status.code(), Some(1));
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["verdict"], "fail");
    assert_eq!(json["identity"]["container_uid"], 424242);
    assert_eq!(json["identity"]["container_gid"], 424242);
    assert_eq!(json["identity"]["source"], "devcontainer remoteUser");
    assert_eq!(json["workspace"]["readable"], true);
    assert_eq!(json["workspace"]["writable"], false);
}

#[test]
fn explicit_root_remote_user_wins_over_non_root_compose_user() {
    let output = run_compose_identity_precedence_case("0:0", "424242:424242");

    assert_eq!(output.status.code(), Some(0));
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["verdict"], "pass");
    assert_eq!(json["identity"]["container_uid"], 0);
    assert_eq!(json["identity"]["container_gid"], 0);
    assert_eq!(json["identity"]["source"], "devcontainer remoteUser");
    assert_eq!(json["workspace"]["readable"], true);
    assert_eq!(json["workspace"]["writable"], true);
}

fn assert_uid_only_is_unknown(config: &str, extra_args: &[&str], runtime_user: Option<&str>) {
    let temp = tempfile::tempdir().unwrap();
    fs::create_dir(temp.path().join(".devcontainer")).unwrap();
    fs::write(temp.path().join(".devcontainer/devcontainer.json"), config).unwrap();
    if config.contains("dockerComposeFile") {
        fs::write(
            temp.path().join(".devcontainer/compose.yml"),
            "services:\n  app:\n    user: \"1000\"\n",
        )
        .unwrap();
    }
    fs::set_permissions(temp.path(), fs::Permissions::from_mode(0o770)).unwrap();

    let current_uid = Command::new("id").arg("-u").output().unwrap().stdout;
    if String::from_utf8(current_uid).unwrap().trim() == "0" {
        let chown = Command::new("chown")
            .arg("0:1000")
            .arg(temp.path())
            .status()
            .unwrap();
        assert!(chown.success());
    }

    let runtime = temp.path().join("docker-fixture");
    fs::write(
        &runtime,
        format!(
            "#!/bin/sh\ncase \"$1\" in\n  info) printf '%s\\n' '{{\"ServerVersion\":\"27.3.1\",\"SecurityOptions\":[]}}' ;;\n  image) printf '%s\\n' '{:?}' ;;\n  *) exit 8 ;;\nesac\n",
            runtime_user.unwrap_or("")
        ),
    )
    .unwrap();
    fs::set_permissions(&runtime, fs::Permissions::from_mode(0o755)).unwrap();

    let output = Command::new(binary())
        .arg(temp.path())
        .args(["--runtime", "docker", "--runtime-bin"])
        .arg(&runtime)
        .args(extra_args)
        .arg("--json")
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(2));
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["verdict"], "unknown");
    assert_eq!(json["identity"]["container_uid"], serde_json::Value::Null);
    assert!(json["summary"].as_str().unwrap().contains("identity"));
    assert!(json["remediations"].to_string().contains("UID:GID"));
    assert!(json["remediations"].to_string().contains("invent"));
}

#[test]
fn uid_only_devcontainer_user_never_uses_an_invented_gid() {
    assert_uid_only_is_unknown(r#"{ remoteUser: "1000" }"#, &[], None);
}

#[test]
fn uid_only_cli_override_never_uses_an_invented_gid() {
    assert_uid_only_is_unknown(
        r#"{ remoteUser: "2000:2000" }"#,
        &["--remote-user", "1000"],
        None,
    );
}

#[test]
fn uid_only_compose_user_never_uses_an_invented_gid() {
    assert_uid_only_is_unknown(
        r#"{ dockerComposeFile: "compose.yml", service: "app" }"#,
        &[],
        None,
    );
}

#[test]
fn uid_only_image_user_never_uses_an_invented_gid() {
    assert_uid_only_is_unknown(r#"{ image: "local/uid-only:latest" }"#, &[], Some("1000"));
}

#[test]
fn share_json_redacts_paths() {
    let temp = tempfile::tempdir().unwrap();
    fs::create_dir(temp.path().join(".devcontainer")).unwrap();
    fs::write(
        temp.path().join(".devcontainer/devcontainer.json"),
        "{ remoteUser: \"0:0\" }",
    )
    .unwrap();
    let output = Command::new(binary())
        .arg(temp.path())
        .args(["--runtime", "docker", "--no-runtime", "--json", "--share"])
        .output()
        .unwrap();
    let body = String::from_utf8(output.stdout).unwrap();
    assert!(!body.contains(temp.path().to_str().unwrap()));
    assert!(body.contains("<workspace>"));
    assert!(body.contains("<devcontainer-config>"));
}

#[test]
fn build_backed_config_without_a_resolved_user_is_unknown() {
    let temp = tempfile::tempdir().unwrap();
    fs::create_dir(temp.path().join(".devcontainer")).unwrap();
    fs::write(
        temp.path().join(".devcontainer/devcontainer.json"),
        r#"{
            name: "Valid Dockerfile-backed Dev Container",
            build: { dockerfile: "Dockerfile" }
        }"#,
    )
    .unwrap();
    fs::write(
        temp.path().join(".devcontainer/Dockerfile"),
        "FROM ubuntu:24.04\nUSER 424242:424242\n",
    )
    .unwrap();
    fs::set_permissions(temp.path(), fs::Permissions::from_mode(0o755)).unwrap();

    let runtime = temp.path().join("docker-fixture");
    fs::write(
        &runtime,
        r#"#!/bin/sh
case "$1" in
  info) printf '%s\n' '{"ServerVersion":"27.3.1","SecurityOptions":[]}' ;;
  *) exit 8 ;;
esac
"#,
    )
    .unwrap();
    fs::set_permissions(&runtime, fs::Permissions::from_mode(0o755)).unwrap();

    let output = Command::new(binary())
        .arg(temp.path())
        .args(["--runtime", "docker", "--runtime-bin"])
        .arg(&runtime)
        .arg("--json")
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(2));
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["verdict"], "unknown");
    assert_eq!(json["identity"]["container_uid"], serde_json::Value::Null);
    assert!(json["summary"].as_str().unwrap().contains("build"));
}

#[test]
fn compose_build_with_image_does_not_trust_a_stale_local_tag() {
    let temp = tempfile::tempdir().unwrap();
    fs::create_dir(temp.path().join(".devcontainer")).unwrap();
    fs::write(
        temp.path().join(".devcontainer/devcontainer.json"),
        r#"{ dockerComposeFile: "compose.yml", service: "app", workspaceFolder: "/work" }"#,
    )
    .unwrap();
    fs::write(
        temp.path().join(".devcontainer/compose.yml"),
        "services:\n  app:\n    build: .\n    image: local/audit-stale:latest\n    volumes:\n      - ../:/work\n",
    )
    .unwrap();
    fs::write(
        temp.path().join(".devcontainer/Dockerfile"),
        "FROM ubuntu:24.04\nUSER 424242:424242\n",
    )
    .unwrap();
    fs::set_permissions(temp.path(), fs::Permissions::from_mode(0o755)).unwrap();

    let runtime = temp.path().join("docker-fixture");
    fs::write(
        &runtime,
        r#"#!/bin/sh
case "$1" in
  info) printf '%s\n' '{"ServerVersion":"27.3.1","SecurityOptions":[]}' ;;
  image) printf '%s\n' '""' ;;
  *) exit 8 ;;
esac
"#,
    )
    .unwrap();
    fs::set_permissions(&runtime, fs::Permissions::from_mode(0o755)).unwrap();

    let output = Command::new(binary())
        .arg(temp.path())
        .args(["--runtime", "docker", "--runtime-bin"])
        .arg(&runtime)
        .arg("--json")
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(2));
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["verdict"], "unknown");
    assert_eq!(json["identity"]["container_uid"], serde_json::Value::Null);
    assert!(
        json["summary"]
            .as_str()
            .unwrap()
            .contains("Compose service app build")
    );
}

#[test]
fn share_json_redacts_runtime_errors_in_every_field() {
    let temp = tempfile::tempdir().unwrap();
    fs::create_dir(temp.path().join(".devcontainer")).unwrap();
    fs::write(
        temp.path().join(".devcontainer/devcontainer.json"),
        "{ remoteUser: \"0:0\" }",
    )
    .unwrap();
    let runtime = temp.path().join("private/acme/team/docker-wrapper");

    let output = Command::new(binary())
        .arg(temp.path())
        .args(["--runtime", "docker", "--runtime-bin"])
        .arg(&runtime)
        .args(["--share", "--json"])
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(2));
    let body = String::from_utf8(output.stdout).unwrap();
    assert!(!body.contains(temp.path().to_str().unwrap()));
    assert!(!body.contains(runtime.to_str().unwrap()));
    assert!(body.contains("<runtime-bin>"));
}

#[test]
fn share_json_redacts_an_explicit_relative_malformed_config_path() {
    let temp = tempfile::tempdir().unwrap();
    let relative = "target/qa/cases/malformed/.devcontainer/devcontainer.json";
    let config = temp.path().join(relative);
    fs::create_dir_all(config.parent().unwrap()).unwrap();
    fs::write(&config, "{").unwrap();

    let output = Command::new(binary())
        .current_dir(temp.path())
        .arg(".")
        .args(["--config", relative, "--share", "--json"])
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(2));
    let body = String::from_utf8(output.stdout).unwrap();
    assert!(!body.contains(relative));
    assert!(body.contains("<devcontainer-config>"));
}

#[test]
fn invalid_config_is_unknown() {
    let temp = tempfile::tempdir().unwrap();
    fs::create_dir(temp.path().join(".devcontainer")).unwrap();
    fs::write(temp.path().join(".devcontainer/devcontainer.json"), "{").unwrap();
    let output = Command::new(binary())
        .arg(temp.path())
        .arg("--quiet")
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stdout).starts_with("UNKNOWN:"));
}

#[test]
fn rootless_podman_uses_live_identity_maps() {
    let temp = tempfile::tempdir().unwrap();
    fs::create_dir(temp.path().join(".devcontainer")).unwrap();
    fs::write(
        temp.path().join(".devcontainer/devcontainer.json"),
        "{ remoteUser: \"0:0\" }",
    )
    .unwrap();
    let runtime = temp.path().join("podman-fixture");
    fs::write(
        &runtime,
        r#"#!/bin/sh
case "$1" in
  info) printf '%s\n' '{"version":{"Version":"5.2.2"},"host":{"security":{"rootless":true}}}' ;;
  unshare)
    case "$3" in
      /proc/self/uid_map|/proc/self/gid_map) printf '%s\n' '0 0 1' '1 100000 65536' ;;
      *) exit 9 ;;
    esac ;;
  *) exit 8 ;;
esac
"#,
    )
    .unwrap();
    fs::set_permissions(&runtime, fs::Permissions::from_mode(0o755)).unwrap();

    let output = Command::new(binary())
        .arg(temp.path())
        .args(["--runtime", "podman", "--runtime-bin"])
        .arg(&runtime)
        .arg("--json")
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["runtime"]["rootless"], true);
    assert_eq!(json["identity"]["host_uid"], 0);
    assert_eq!(json["verdict"], "pass");
}

#[test]
fn rootless_podman_split_keep_id_preserves_the_host_identity() {
    let temp = tempfile::tempdir().unwrap();
    let current_uid = Command::new("id").arg("-u").output().unwrap().stdout;
    let current_uid = String::from_utf8(current_uid)
        .unwrap()
        .trim()
        .parse::<u32>()
        .unwrap();
    let current_gid = Command::new("id").arg("-g").output().unwrap().stdout;
    let current_gid = String::from_utf8(current_gid)
        .unwrap()
        .trim()
        .parse::<u32>()
        .unwrap();

    // The verifier reproduced this as nobody. Keep that exact non-zero
    // identity when tests run as root; otherwise use the test user's identity.
    let (uid, gid) = if current_uid == 0 {
        (65_534, 65_534)
    } else {
        (current_uid, current_gid)
    };
    if current_uid == 0 {
        let status = Command::new("chown")
            .arg(format!("{uid}:{gid}"))
            .arg(temp.path())
            .status()
            .unwrap();
        assert!(status.success());
    }

    fs::create_dir(temp.path().join(".devcontainer")).unwrap();
    fs::write(
        temp.path().join(".devcontainer/devcontainer.json"),
        format!("{{ remoteUser: \"{uid}:{gid}\", runArgs: [\"--userns\", \"keep-id\"] }}"),
    )
    .unwrap();
    let runtime = temp.path().join("podman-fixture");
    fs::write(
        &runtime,
        format!(
            "#!/bin/sh\ncase \"$1\" in\n  info) printf '%s\\n' '{{\"version\":{{\"Version\":\"5.2.2\"}},\"host\":{{\"security\":{{\"rootless\":true}}}}}}' ;;\n  unshare)\n    case \"$3\" in\n      /proc/self/uid_map|/proc/self/gid_map) printf '%s\\n' '0 {uid} 1' '1 100000 65536' ;;\n      *) exit 9 ;;\n    esac ;;\n  *) exit 8 ;;\nesac\n"
        ),
    )
    .unwrap();
    fs::set_permissions(&runtime, fs::Permissions::from_mode(0o755)).unwrap();

    let mut command = Command::new(binary());
    command
        .arg(temp.path())
        .args(["--runtime", "podman", "--runtime-bin"])
        .arg(&runtime)
        .arg("--json");
    if current_uid == 0 {
        let bin = temp.path().join("bin");
        fs::create_dir(&bin).unwrap();
        let fake_id = bin.join("id");
        fs::write(
            &fake_id,
            format!(
                "#!/bin/sh\ncase \"$1\" in\n  -u) echo {uid} ;;\n  -g) echo {gid} ;;\n  *) exit 2 ;;\nesac\n"
            ),
        )
        .unwrap();
        fs::set_permissions(&fake_id, fs::Permissions::from_mode(0o755)).unwrap();
        command.env(
            "PATH",
            format!("{}:{}", bin.display(), std::env::var("PATH").unwrap()),
        );
    }
    let output = command.output().unwrap();

    assert_eq!(output.status.code(), Some(0));
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["identity"]["host_uid"], uid);
    assert_eq!(json["identity"]["host_gid"], gid);
    assert_eq!(json["verdict"], "pass");
}

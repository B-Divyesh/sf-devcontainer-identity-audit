use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

fn binary() -> &'static str {
    env!("CARGO_BIN_EXE_mount-identity-audit")
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

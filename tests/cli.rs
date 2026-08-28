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

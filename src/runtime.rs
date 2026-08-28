use clap::ValueEnum;
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum RuntimeChoice {
    Auto,
    Docker,
    Podman,
}

#[derive(Debug, Clone)]
pub struct RuntimeInfo {
    pub kind: String,
    pub version: Option<String>,
    pub rootless: Option<bool>,
    pub inspected: bool,
    pub executable: PathBuf,
}

pub fn inspect(
    choice: RuntimeChoice,
    override_bin: Option<&Path>,
    no_runtime: bool,
) -> Result<RuntimeInfo, String> {
    if no_runtime {
        let kind = match choice {
            RuntimeChoice::Docker => "docker",
            RuntimeChoice::Podman => "podman",
            RuntimeChoice::Auto => {
                return Err("--no-runtime requires --runtime docker or --runtime podman".into());
            }
        };
        return Ok(RuntimeInfo {
            kind: kind.into(),
            version: None,
            rootless: if kind == "docker" { Some(false) } else { None },
            inspected: false,
            executable: override_bin.unwrap_or(Path::new(kind)).to_path_buf(),
        });
    }

    if let Some(path) = override_bin {
        let selected = match choice {
            RuntimeChoice::Podman => RuntimeChoice::Podman,
            _ => RuntimeChoice::Docker,
        };
        return inspect_one(selected, path);
    }

    match choice {
        RuntimeChoice::Docker => inspect_one(choice, Path::new("docker")),
        RuntimeChoice::Podman => inspect_one(choice, Path::new("podman")),
        RuntimeChoice::Auto => {
            let podman_hint = std::env::var("DOCKER_HOST")
                .is_ok_and(|value| value.to_lowercase().contains("podman"));
            let candidates = if podman_hint {
                [
                    (RuntimeChoice::Podman, "podman"),
                    (RuntimeChoice::Docker, "docker"),
                ]
            } else {
                [
                    (RuntimeChoice::Docker, "docker"),
                    (RuntimeChoice::Podman, "podman"),
                ]
            };
            let mut errors = Vec::new();
            for (runtime, binary) in candidates {
                match inspect_one(runtime, Path::new(binary)) {
                    Ok(info) => return Ok(info),
                    Err(error) => errors.push(error),
                }
            }
            Err(format!(
                "neither Docker nor Podman responded ({})",
                errors.join("; ")
            ))
        }
    }
}

fn inspect_one(choice: RuntimeChoice, executable: &Path) -> Result<RuntimeInfo, String> {
    let kind = match choice {
        RuntimeChoice::Docker => "docker",
        RuntimeChoice::Podman => "podman",
        RuntimeChoice::Auto => unreachable!(),
    };
    let args: &[&str] = if kind == "docker" {
        &["info", "--format", "{{json .}}"]
    } else {
        &["info", "--format", "json"]
    };
    let output = run(executable, args)?;
    let value: Value = serde_json::from_str(&output)
        .map_err(|error| format!("{kind} info returned invalid JSON: {error}"))?;
    let version = if kind == "docker" {
        lookup_string(&value, &["ServerVersion"])
            .or_else(|| lookup_string(&value, &["serverVersion"]))
    } else {
        lookup_string(&value, &["version", "Version"])
            .or_else(|| lookup_string(&value, &["version", "version"]))
    };
    let rootless = if kind == "podman" {
        lookup_bool(&value, &["host", "security", "rootless"])
            .or_else(|| lookup_bool(&value, &["Host", "Security", "Rootless"]))
    } else {
        let serialized = value
            .get("SecurityOptions")
            .map(Value::to_string)
            .unwrap_or_default();
        Some(serialized.to_lowercase().contains("rootless"))
    };
    Ok(RuntimeInfo {
        kind: kind.into(),
        version,
        rootless,
        inspected: true,
        executable: executable.to_path_buf(),
    })
}

pub fn inspect_image_user(
    info: &RuntimeInfo,
    override_bin: Option<&Path>,
    image: &str,
) -> Result<String, String> {
    if !info.inspected {
        return Err("runtime inspection disabled".into());
    }
    let executable = override_bin.unwrap_or(&info.executable);
    let output = run(
        executable,
        &[
            "image",
            "inspect",
            image,
            "--format",
            "{{json .Config.User}}",
        ],
    )?;
    serde_json::from_str::<String>(output.trim())
        .or_else(|_| Ok(output.trim().trim_matches('"').to_string()))
}

pub fn map_identity(
    info: &RuntimeInfo,
    override_bin: Option<&Path>,
    uid: u32,
    gid: u32,
    run_args: &[String],
) -> Result<(u32, u32), String> {
    if info.kind == "docker" {
        if info.rootless == Some(true) {
            return Err("rootless Docker UID maps are not supported in v1".into());
        }
        return Ok((uid, gid));
    }
    if info.rootless == Some(false) {
        return Ok((uid, gid));
    }

    let userns = userns_value(run_args);
    if userns.is_some_and(|value| value == "host") {
        return Ok((uid, gid));
    }
    if userns.is_some_and(|value| value.starts_with("keep-id")) {
        let host_uid = id_value("-u")?;
        let host_gid = id_value("-g")?;
        let configured_uid = userns
            .and_then(|value| option_number(value, "uid"))
            .unwrap_or(host_uid);
        let configured_gid = userns
            .and_then(|value| option_number(value, "gid"))
            .unwrap_or(host_gid);
        if uid == configured_uid && gid == configured_gid {
            return Ok((host_uid, host_gid));
        }
    }
    if !info.inspected {
        return Err(
            "rootless Podman mapping requires runtime inspection or --userns=host/keep-id".into(),
        );
    }
    let executable = override_bin.unwrap_or(&info.executable);
    let uid_map = read_map(executable, "/proc/self/uid_map")?;
    let gid_map = read_map(executable, "/proc/self/gid_map")?;
    Ok((map_id(uid, &uid_map)?, map_id(gid, &gid_map)?))
}

fn read_map(executable: &Path, path: &str) -> Result<Vec<(u32, u32, u32)>, String> {
    let output = run(executable, &["unshare", "cat", path])?;
    let mut rows = Vec::new();
    for line in output.lines() {
        let numbers: Vec<u32> = line
            .split_whitespace()
            .filter_map(|part| part.parse().ok())
            .collect();
        if numbers.len() == 3 {
            rows.push((numbers[0], numbers[1], numbers[2]));
        }
    }
    if rows.is_empty() {
        Err(format!("empty identity map from {path}"))
    } else {
        Ok(rows)
    }
}

fn map_id(id: u32, rows: &[(u32, u32, u32)]) -> Result<u32, String> {
    for (container_start, host_start, count) in rows {
        if id >= *container_start && id < container_start.saturating_add(*count) {
            return Ok(host_start + (id - container_start));
        }
    }
    Err(format!("container ID {id} is outside the runtime map"))
}

fn id_value(flag: &str) -> Result<u32, String> {
    run(Path::new("id"), &[flag])?
        .trim()
        .parse()
        .map_err(|_| format!("`id {flag}` returned a non-numeric value"))
}

fn option_number(value: &str, key: &str) -> Option<u32> {
    value.split([':', ',']).find_map(|item| {
        item.strip_prefix(&format!("{key}="))
            .and_then(|raw| raw.parse().ok())
    })
}

/// Return the value supplied to Podman's `--userns` option in Dev Container
/// `runArgs`. Both `--userns=value` and the normal option/value spelling are
/// valid there.
fn userns_value(run_args: &[String]) -> Option<&str> {
    run_args.iter().enumerate().find_map(|(index, argument)| {
        argument
            .strip_prefix("--userns=")
            .or_else(|| argument.strip_prefix("--userns:"))
            .or_else(|| {
                (argument == "--userns")
                    .then(|| run_args.get(index + 1))
                    .flatten()
                    .map(String::as_str)
            })
    })
}

fn run(executable: &Path, args: &[&str]) -> Result<String, String> {
    let output = Command::new(executable)
        .args(args)
        .output()
        .map_err(|error| format!("cannot run {}: {error}", executable.display()))?;
    if !output.status.success() {
        let detail = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(format!(
            "{} {} failed{}",
            executable.display(),
            args.join(" "),
            if detail.is_empty() {
                String::new()
            } else {
                format!(": {detail}")
            }
        ));
    }
    String::from_utf8(output.stdout)
        .map_err(|_| format!("{} returned non-UTF-8 output", executable.display()))
}

fn lookup_string(value: &Value, path: &[&str]) -> Option<String> {
    let mut cursor = value;
    for segment in path {
        cursor = cursor.get(*segment)?;
    }
    cursor.as_str().map(str::to_string)
}

fn lookup_bool(value: &Value, path: &[&str]) -> Option<bool> {
    let mut cursor = value;
    for segment in path {
        cursor = cursor.get(*segment)?;
    }
    cursor.as_bool()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_subuid_ranges() {
        let rows = [(0, 1000, 1), (1, 100000, 65536)];
        assert_eq!(map_id(0, &rows).unwrap(), 1000);
        assert_eq!(map_id(1000, &rows).unwrap(), 100999);
        assert!(map_id(70000, &rows).is_err());
    }

    #[test]
    fn reads_keep_id_options() {
        assert_eq!(
            option_number("keep-id:uid=1200,gid=1300", "uid"),
            Some(1200)
        );
        assert_eq!(
            option_number("keep-id:uid=1200,gid=1300", "gid"),
            Some(1300)
        );
    }

    #[test]
    fn reads_split_userns_options() {
        let keep_id = vec!["--userns".to_string(), "keep-id".to_string()];
        let host = vec!["--userns".to_string(), "host".to_string()];

        assert_eq!(userns_value(&keep_id), Some("keep-id"));
        assert_eq!(userns_value(&host), Some("host"));
    }
}

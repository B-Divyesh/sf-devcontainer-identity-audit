use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ProjectConfig {
    pub path: PathBuf,
    pub remote_user: Option<String>,
    pub remote_user_source: String,
    pub image: Option<String>,
    pub workspace: Option<PathBuf>,
    pub read_only: bool,
    pub run_args: Vec<String>,
}

pub fn load(project: &Path, explicit: Option<&Path>) -> Result<ProjectConfig, String> {
    let path = match explicit {
        Some(path) => path.to_path_buf(),
        None => [
            project.join(".devcontainer/devcontainer.json"),
            project.join(".devcontainer.json"),
            project.join("devcontainer.json"),
        ]
        .into_iter()
        .find(|path| path.is_file())
        .ok_or_else(|| {
            format!(
                "No Dev Container configuration found below {}. Expected .devcontainer/devcontainer.json, .devcontainer.json, or devcontainer.json.",
                project.display()
            )
        })?,
    };
    let source = fs::read_to_string(&path)
        .map_err(|error| format!("Cannot read {}: {error}", path.display()))?;
    let json: JsonValue = json5::from_str(&source)
        .map_err(|error| format!("Invalid JSONC in {}: {error}", path.display()))?;

    let base = project_root_for(&path);
    let remote_user =
        string_value(json.get("remoteUser")).or_else(|| string_value(json.get("containerUser")));
    let remote_user_source = if json.get("remoteUser").is_some() {
        "devcontainer remoteUser"
    } else if json.get("containerUser").is_some() {
        "devcontainer containerUser"
    } else {
        "image default"
    }
    .to_string();
    let mut result = ProjectConfig {
        path: path.clone(),
        remote_user,
        remote_user_source,
        image: string_value(json.get("image")),
        workspace: None,
        read_only: false,
        run_args: string_array(json.get("runArgs")),
    };

    if let Some(mount) = json.get("workspaceMount").and_then(|value| value.as_str()) {
        let parsed = parse_mount(mount, &base);
        result.workspace = parsed.0;
        result.read_only = parsed.1;
    }

    let compose_files = json
        .get("dockerComposeFile")
        .map(string_or_array)
        .unwrap_or_default();
    if !compose_files.is_empty() {
        let service = json
            .get("service")
            .and_then(|value| value.as_str())
            .ok_or_else(|| "dockerComposeFile is set but `service` is missing.".to_string())?;
        for compose in compose_files {
            let compose_path = resolve_relative(path.parent().unwrap_or(&base), &compose);
            merge_compose(
                &mut result,
                &compose_path,
                service,
                json.get("workspaceFolder").and_then(|v| v.as_str()),
            )?;
        }
    }

    Ok(result)
}

fn merge_compose(
    result: &mut ProjectConfig,
    path: &Path,
    service_name: &str,
    workspace_folder: Option<&str>,
) -> Result<(), String> {
    let source = fs::read_to_string(path)
        .map_err(|error| format!("Cannot read Compose file {}: {error}", path.display()))?;
    let yaml: YamlValue = serde_yaml::from_str(&source)
        .map_err(|error| format!("Invalid Compose YAML in {}: {error}", path.display()))?;
    let service = yaml
        .get("services")
        .and_then(|value| value.get(service_name))
        .ok_or_else(|| {
            format!(
                "Compose service {service_name:?} not found in {}",
                path.display()
            )
        })?;

    if let Some(user) = service.get("user").and_then(yaml_scalar) {
        result.remote_user = Some(user);
        result.remote_user_source = format!("Compose service {service_name} user");
    }
    if let Some(image) = service.get("image").and_then(yaml_scalar) {
        result.image = Some(image);
    }
    if result.workspace.is_none()
        && let Some(volumes) = service.get("volumes").and_then(|value| value.as_sequence())
    {
        let mut fallback = None;
        for volume in volumes {
            if let Some((source, target, read_only)) =
                parse_compose_volume(volume, path.parent().unwrap_or(Path::new(".")))
            {
                if fallback.is_none() {
                    fallback = Some((source.clone(), read_only));
                }
                if workspace_folder.is_some_and(|folder| folder == target) {
                    result.workspace = Some(source);
                    result.read_only = read_only;
                    break;
                }
            }
        }
        if result.workspace.is_none()
            && let Some((source, read_only)) = fallback
        {
            result.workspace = Some(source);
            result.read_only = read_only;
        }
    }
    Ok(())
}

fn parse_compose_volume(value: &YamlValue, base: &Path) -> Option<(PathBuf, String, bool)> {
    if let Some(raw) = value.as_str() {
        let parts: Vec<&str> = raw.split(':').collect();
        if parts.len() < 2 || is_named_volume(parts[0]) {
            return None;
        }
        let read_only = parts
            .get(2)
            .is_some_and(|mode| mode.split(',').any(|item| item == "ro"));
        return Some((
            resolve_relative(base, parts[0]),
            parts[1].to_string(),
            read_only,
        ));
    }
    let kind = value
        .get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("volume");
    if kind != "bind" {
        return None;
    }
    let source = value.get("source").and_then(|v| v.as_str())?;
    let target = value.get("target").and_then(|v| v.as_str())?;
    let read_only = value
        .get("read_only")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    Some((
        resolve_relative(base, source),
        target.to_string(),
        read_only,
    ))
}

fn parse_mount(raw: &str, base: &Path) -> (Option<PathBuf>, bool) {
    let mut source = None;
    let mut read_only = false;
    for part in raw.split(',') {
        let (key, value) = part.split_once('=').unwrap_or((part, ""));
        match key.trim() {
            "source" | "src" => {
                let expanded = value
                    .replace("${localWorkspaceFolder}", &base.display().to_string())
                    .replace(
                        "${localWorkspaceFolderBasename}",
                        base.file_name()
                            .and_then(|v| v.to_str())
                            .unwrap_or("workspace"),
                    );
                source = Some(resolve_relative(base, &expanded));
            }
            "readonly" | "read_only" | "ro" => read_only = value.is_empty() || value == "true",
            _ => {}
        }
    }
    (source, read_only)
}

fn string_value(value: Option<&JsonValue>) -> Option<String> {
    value.and_then(|value| match value {
        JsonValue::String(value) => Some(value.clone()),
        JsonValue::Number(value) => Some(value.to_string()),
        _ => None,
    })
}

fn string_array(value: Option<&JsonValue>) -> Vec<String> {
    value
        .and_then(|value| value.as_array())
        .map(|items| {
            items
                .iter()
                .filter_map(|item| item.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

fn string_or_array(value: &JsonValue) -> Vec<String> {
    match value {
        JsonValue::String(value) => vec![value.clone()],
        JsonValue::Array(values) => values
            .iter()
            .filter_map(|value| value.as_str().map(str::to_string))
            .collect(),
        _ => Vec::new(),
    }
}

fn yaml_scalar(value: &YamlValue) -> Option<String> {
    match value {
        YamlValue::String(value) => Some(value.clone()),
        YamlValue::Number(value) => Some(value.to_string()),
        _ => None,
    }
}

fn project_root_for(config: &Path) -> PathBuf {
    let parent = config.parent().unwrap_or(Path::new("."));
    if parent.file_name().and_then(|v| v.to_str()) == Some(".devcontainer") {
        parent.parent().unwrap_or(parent).to_path_buf()
    } else {
        parent.to_path_buf()
    }
}

fn resolve_relative(base: &Path, value: &str) -> PathBuf {
    let path = Path::new(value);
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        base.join(path)
    }
}

fn is_named_volume(source: &str) -> bool {
    !source.starts_with('.')
        && !source.starts_with('/')
        && !source.contains('/')
        && !source.contains('\\')
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn reads_jsonc_and_workspace_mount() {
        let temp = tempfile::tempdir().unwrap();
        fs::create_dir(temp.path().join(".devcontainer")).unwrap();
        fs::write(
            temp.path().join(".devcontainer/devcontainer.json"),
            r#"{ // identity
                remoteUser: "1000:1001",
                workspaceMount: "source=${localWorkspaceFolder},target=/work,type=bind,readonly=false",
                runArgs: ["--userns=keep-id",],
            }"#,
        )
        .unwrap();
        let loaded = load(temp.path(), None).unwrap();
        assert_eq!(loaded.remote_user.as_deref(), Some("1000:1001"));
        assert_eq!(
            loaded.workspace.unwrap().canonicalize().unwrap(),
            temp.path().canonicalize().unwrap()
        );
        assert!(!loaded.read_only);
    }

    #[test]
    fn reads_compose_bind_and_user() {
        let temp = tempfile::tempdir().unwrap();
        fs::create_dir(temp.path().join(".devcontainer")).unwrap();
        fs::write(
            temp.path().join(".devcontainer/devcontainer.json"),
            r#"{
          dockerComposeFile: "compose.yml", service: "app", workspaceFolder: "/work"
        }"#,
        )
        .unwrap();
        fs::write(
            temp.path().join(".devcontainer/compose.yml"),
            r#"
services:
  app:
    image: example:local
    user: "1000:1000"
    volumes:
      - "../:/work:ro"
"#,
        )
        .unwrap();
        let loaded = load(temp.path(), None).unwrap();
        assert_eq!(loaded.remote_user.as_deref(), Some("1000:1000"));
        assert!(loaded.read_only);
        assert_eq!(
            loaded.workspace.unwrap().canonicalize().unwrap(),
            temp.path().canonicalize().unwrap()
        );
    }
}

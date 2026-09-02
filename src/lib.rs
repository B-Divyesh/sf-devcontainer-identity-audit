//! Mount Identity Audit core library.
//!
//! The library is intentionally small: [`audit`] accepts explicit options and
//! returns a serializable [`AuditReport`] without mutating the project or host.

mod config;
mod runtime;

use serde::Serialize;
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

pub use runtime::RuntimeChoice;

#[derive(Debug, Clone)]
pub struct AuditOptions {
    pub project: PathBuf,
    pub config: Option<PathBuf>,
    pub workspace: Option<PathBuf>,
    pub remote_user: Option<String>,
    pub runtime: RuntimeChoice,
    pub runtime_bin: Option<PathBuf>,
    pub no_runtime: bool,
    pub share: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Verdict {
    Pass,
    Fail,
    Unknown,
}

impl Verdict {
    pub fn exit_code(self) -> i32 {
        match self {
            Self::Pass => 0,
            Self::Fail => 1,
            Self::Unknown => 2,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Pass => "PASS",
            Self::Fail => "FAIL",
            Self::Unknown => "UNKNOWN",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Check {
    pub name: String,
    pub expected: String,
    pub observed: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct IdentityReport {
    pub container_uid: Option<u32>,
    pub container_gid: Option<u32>,
    pub host_uid: Option<u32>,
    pub host_gid: Option<u32>,
    pub source: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct WorkspaceReport {
    pub path: String,
    pub owner_uid: u32,
    pub owner_gid: u32,
    pub mode: String,
    pub declared_read_only: bool,
    pub readable: Option<bool>,
    pub writable: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RuntimeReport {
    pub kind: String,
    pub version: Option<String>,
    pub rootless: Option<bool>,
    pub inspected: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuditReport {
    pub schema_version: u8,
    pub verdict: Verdict,
    pub summary: String,
    pub config_path: String,
    pub runtime: RuntimeReport,
    pub identity: IdentityReport,
    pub workspace: Option<WorkspaceReport>,
    pub checks: Vec<Check>,
    pub remediations: Vec<String>,
    pub caveats: Vec<String>,
    pub guarantees: Vec<String>,
}

/// Audit one Dev Container project without creating or starting a container.
pub fn audit(options: AuditOptions) -> AuditReport {
    let share = options.share;
    let redactions = if share {
        report_path_redactions(&options)
    } else {
        Vec::new()
    };
    let mut report = audit_unredacted(options);
    if share {
        redact_report(&mut report, &redactions);
    }
    report
}

fn audit_unredacted(options: AuditOptions) -> AuditReport {
    let mut checks = Vec::new();
    let mut remediations = Vec::new();
    let caveats = vec![
        "POSIX ACLs, security labels, and remote filesystem policy are not evaluated.".to_string(),
        "Runtime changes made while creating a container can alter the final identity.".to_string(),
    ];
    let guarantees = vec![
        "No files were changed.".to_string(),
        "No image was pulled and no container was created or started.".to_string(),
    ];

    let project = canonical_or_original(&options.project);
    let loaded = match config::load(&project, options.config.as_deref()) {
        Ok(value) => value,
        Err(error) => {
            return unknown_report(
                options.share,
                error,
                "<not loaded>".into(),
                checks,
                remediations,
                caveats,
                guarantees,
            );
        }
    };
    let config_label = redact_path(&loaded.path, options.share, "<devcontainer-config>");

    let workspace_path = options
        .workspace
        .clone()
        .or_else(|| loaded.workspace.clone())
        .unwrap_or_else(|| project.clone());
    let workspace_path = canonical_or_original(&workspace_path);
    let metadata = match fs::metadata(&workspace_path) {
        Ok(value) if value.is_dir() => value,
        Ok(_) => {
            return unknown_report(
                options.share,
                format!(
                    "Workspace path is not a directory: {}",
                    workspace_path.display()
                ),
                config_label,
                checks,
                vec!["Pass --workspace with the host directory mounted as the workspace.".into()],
                caveats,
                guarantees,
            );
        }
        Err(error) => {
            return unknown_report(
                options.share,
                format!(
                    "Cannot inspect workspace {}: {error}",
                    workspace_path.display()
                ),
                config_label,
                checks,
                vec!["Pass --workspace with an existing, accessible host directory.".into()],
                caveats,
                guarantees,
            );
        }
    };

    let runtime_info = match runtime::inspect(
        options.runtime,
        options.runtime_bin.as_deref(),
        options.no_runtime,
    ) {
        Ok(info) => info,
        Err(error) => {
            checks.push(Check {
                name: "runtime".into(),
                expected: "reachable read-only adapter".into(),
                observed: error.clone(),
                status: "unknown".into(),
            });
            return unknown_report(
                options.share,
                format!("Runtime identity mapping is unknown: {error}"),
                config_label,
                checks,
                vec![
                    "Start the selected runtime, choose --runtime docker|podman, or use --no-runtime with an explicit runtime.".into(),
                    "Provide --remote-user UID:GID when the configured user is named.".into(),
                ],
                caveats,
                guarantees,
            );
        }
    };

    let runtime_observed = match (
        &runtime_info.version,
        runtime_info.rootless,
        runtime_info.userns_remap,
    ) {
        (Some(version), _, true) => {
            format!("{} {version} (userns-remap)", runtime_info.kind)
        }
        (Some(version), Some(true), false) => {
            format!("{} {version} (rootless)", runtime_info.kind)
        }
        (Some(version), _, false) => format!("{} {version}", runtime_info.kind),
        (None, _, _) => format!("{} (not inspected)", runtime_info.kind),
    };
    checks.push(Check {
        name: "runtime".into(),
        expected: runtime_info.kind.clone(),
        observed: runtime_observed,
        status: if runtime_info.userns_remap {
            "warning"
        } else if runtime_info.inspected {
            "pass"
        } else {
            "warning"
        }
        .into(),
    });

    let raw_user = options
        .remote_user
        .clone()
        .or_else(|| loaded.remote_user.clone());
    let identity_source = if options.remote_user.is_some() {
        "--remote-user".to_string()
    } else {
        loaded.remote_user_source.clone()
    };

    let resolved_user = match raw_user {
        Some(raw) => match parse_numeric_user(&raw) {
            Ok(pair) => pair,
            Err(error) => {
                let (observed, summary, remediation) = match error {
                    NumericUserError::MissingGid => (
                        format!("UID-only user {raw:?}; primary GID is unproven"),
                        "The intended remote user does not include a GID, so its numeric identity cannot be proven.",
                        format!(
                            "Resolve UID {raw}'s primary group inside the image and rerun with --remote-user UID:GID. The audit will not invent a same-number GID."
                        ),
                    ),
                    NumericUserError::Reserved => (
                        format!("reserved Linux identity {raw:?}"),
                        "The intended remote user contains Linux's reserved 4294967295 identity value.",
                        "Choose a usable UID:GID below 4294967295 and rerun the audit.".to_string(),
                    ),
                    NumericUserError::NotNumeric => (
                        format!("named or invalid user {raw:?}"),
                        "The intended remote user has no safely resolvable numeric identity.",
                        format!(
                            "Resolve {raw:?} inside the image and rerun with --remote-user UID:GID. The audit will not start a container to guess it."
                        ),
                    ),
                };
                checks.push(Check {
                    name: "remote user".into(),
                    expected: "numeric UID:GID".into(),
                    observed,
                    status: "unknown".into(),
                });
                remediations.push(remediation);
                return unknown_report_with_runtime(
                    options.share,
                    summary.into(),
                    config_label,
                    runtime_info,
                    checks,
                    remediations,
                    caveats,
                    guarantees,
                );
            }
        },
        None => {
            if let Some(build_detail) = loaded.build_source.as_deref() {
                checks.push(Check {
                    name: "remote user".into(),
                    expected: "numeric UID:GID tied to current build inputs".into(),
                    observed: format!("unresolved {build_detail}"),
                    status: "unknown".into(),
                });
                remediations.push(
                    "Resolve the current build's effective user and rerun with --remote-user UID:GID. A local image tag may be stale, so the audit will not inspect it as build evidence."
                        .into(),
                );
                return unknown_report_with_runtime(
                    options.share,
                    format!(
                        "The {build_detail} does not provide a safely resolvable numeric identity."
                    ),
                    config_label,
                    runtime_info,
                    checks,
                    remediations,
                    caveats,
                    guarantees,
                );
            }
            let Some(image) = loaded.image.as_deref() else {
                checks.push(Check {
                    name: "remote user".into(),
                    expected: "numeric UID:GID".into(),
                    observed: "unresolved configuration without an image".into(),
                    status: "unknown".into(),
                });
                remediations.push(
                    "Provide a configured image or rerun with --remote-user UID:GID. The audit will not assume root."
                        .into(),
                );
                return unknown_report_with_runtime(
                    options.share,
                    "The configuration does not provide a safely resolvable numeric identity."
                        .into(),
                    config_label,
                    runtime_info,
                    checks,
                    remediations,
                    caveats,
                    guarantees,
                );
            };
            let image_user =
                runtime::inspect_image_user(&runtime_info, options.runtime_bin.as_deref(), image)
                    .ok();
            match image_user.as_deref() {
                Some("") => (0, 0),
                None => {
                    checks.push(Check {
                        name: "remote user".into(),
                        expected: "numeric UID:GID".into(),
                        observed: "image metadata unavailable".into(),
                        status: "unknown".into(),
                    });
                    remediations.push(
                        "Make the configured image available locally or rerun with --remote-user UID:GID. Images are never pulled automatically."
                            .into(),
                    );
                    return unknown_report_with_runtime(
                        options.share,
                        "The configured image is unavailable or its user metadata could not be read."
                            .into(),
                        config_label,
                        runtime_info,
                        checks,
                        remediations,
                        caveats,
                        guarantees,
                    );
                }
                Some(raw) => match parse_numeric_user(raw) {
                    Ok(pair) => pair,
                    Err(error) => {
                        let (observed, summary, remediation) = match error {
                            NumericUserError::MissingGid => (
                                format!("image UID {raw:?}; primary GID is unproven"),
                                "The image declares a UID without a primary GID, so its numeric identity cannot be proven.",
                                format!(
                                    "Resolve image UID {raw}'s primary group and rerun with --remote-user UID:GID. The audit will not invent a same-number GID."
                                ),
                            ),
                            NumericUserError::Reserved => (
                                format!("reserved Linux image identity {raw:?}"),
                                "The image declares Linux's reserved 4294967295 identity value.",
                                "Choose a usable image UID:GID below 4294967295 or pass --remote-user UID:GID."
                                    .to_string(),
                            ),
                            NumericUserError::NotNumeric => (
                                format!("image user {raw:?}"),
                                "The image declares a named user that metadata cannot map to a UID and GID.",
                                format!(
                                    "Resolve image user {raw:?} and rerun with --remote-user UID:GID."
                                ),
                            ),
                        };
                        checks.push(Check {
                            name: "remote user".into(),
                            expected: "numeric UID:GID".into(),
                            observed,
                            status: "unknown".into(),
                        });
                        remediations.push(remediation);
                        return unknown_report_with_runtime(
                            options.share,
                            summary.into(),
                            config_label,
                            runtime_info,
                            checks,
                            remediations,
                            caveats,
                            guarantees,
                        );
                    }
                },
            }
        }
    };

    let (container_uid, container_gid) = resolved_user;
    let mapped = match runtime::map_identity(
        &runtime_info,
        options.runtime_bin.as_deref(),
        container_uid,
        container_gid,
        &loaded.run_args,
    ) {
        Ok(pair) => pair,
        Err(error) => {
            checks.push(Check {
                name: "identity map".into(),
                expected: format!("container {container_uid}:{container_gid} mapped to host"),
                observed: error.clone(),
                status: "unknown".into(),
            });
            if runtime_info.kind == "docker" && runtime_info.userns_remap {
                remediations.push(
                    "Inspect Docker's userns-remap configuration and subordinate-ID allocation. This version will not assume direct host IDs."
                        .into(),
                );
            } else if runtime_info.kind == "docker" {
                remediations.push(
                    "Rootless Docker identity maps are not supported in this version. Verify the mapped host IDs before relying on workspace access."
                        .into(),
                );
            } else {
                remediations.push("For rootless Podman, use --userns=keep-id or ensure `podman unshare` can read the live UID/GID maps.".into());
            }
            return unknown_report_with_runtime(
                options.share,
                format!("The runtime identity map could not be proven: {error}"),
                config_label,
                runtime_info,
                checks,
                remediations,
                caveats,
                guarantees,
            );
        }
    };
    let (host_uid, host_gid) = mapped;
    let mode = metadata.mode() & 0o7777;
    let owner_uid = metadata.uid();
    let owner_gid = metadata.gid();
    let (readable, writable) = access_for(mode, owner_uid, owner_gid, host_uid, host_gid);
    let writable = writable && !loaded.read_only;

    checks.push(Check {
        name: "remote user".into(),
        expected: format!("container {container_uid}:{container_gid}"),
        observed: format!("host {host_uid}:{host_gid} via {identity_source}"),
        status: "pass".into(),
    });
    checks.push(Check {
        name: "workspace".into(),
        expected: "read + write + traverse".into(),
        observed: format!(
            "{} {:04o} {owner_uid}:{owner_gid}{}",
            file_type_prefix(mode),
            mode,
            if loaded.read_only {
                " (declared read-only)"
            } else {
                ""
            }
        ),
        status: if readable && writable { "pass" } else { "fail" }.into(),
    });

    let verdict = if readable && writable {
        Verdict::Pass
    } else {
        Verdict::Fail
    };
    let summary = if verdict == Verdict::Pass {
        "The intended remote user can read and write this bind mount.".to_string()
    } else if loaded.read_only {
        "The workspace bind mount is declared read-only.".to_string()
    } else if !readable {
        "The mapped remote identity cannot read and traverse the workspace.".to_string()
    } else {
        "The mapped remote identity can read but cannot write the workspace.".to_string()
    };

    if verdict == Verdict::Fail {
        if loaded.read_only {
            remediations.push(
                "Review the workspace mount and remove `readonly`, `read_only`, or `ro` only if workspace edits are intended."
                    .into(),
            );
        } else {
            if runtime_info.kind == "podman" && runtime_info.rootless == Some(true) {
                remediations.push("Prefer `\"runArgs\": [\"--userns=keep-id\"]` so the calling developer keeps the same identity in rootless Podman.".into());
            }
            remediations.push(format!(
                "Choose a remote UID:GID that maps to workspace owner {owner_uid}:{owner_gid}, then verify with --remote-user {owner_uid}:{owner_gid}."
            ));
            remediations.push("If team write access is intentional, change the project group/mode explicitly on the host; this tool never changes ownership or permissions.".into());
        }
    }

    AuditReport {
        schema_version: 1,
        verdict,
        summary,
        config_path: config_label,
        runtime: RuntimeReport::from(&runtime_info),
        identity: IdentityReport {
            container_uid: Some(container_uid),
            container_gid: Some(container_gid),
            host_uid: Some(host_uid),
            host_gid: Some(host_gid),
            source: identity_source,
        },
        workspace: Some(WorkspaceReport {
            path: redact_path(&workspace_path, options.share, "<workspace>"),
            owner_uid,
            owner_gid,
            mode: format!("{:04o}", mode),
            declared_read_only: loaded.read_only,
            readable: Some(readable),
            writable: Some(writable),
        }),
        checks,
        remediations,
        caveats,
        guarantees,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NumericUserError {
    MissingGid,
    Reserved,
    NotNumeric,
}

fn parse_numeric_user(raw: &str) -> Result<(u32, u32), NumericUserError> {
    if raw == "root" {
        return Ok((0, 0));
    }
    let Some((uid, gid)) = raw.split_once(':') else {
        return Err(if raw.parse::<u32>().is_ok() {
            NumericUserError::MissingGid
        } else {
            NumericUserError::NotNumeric
        });
    };
    let pair = (
        uid.parse().map_err(|_| NumericUserError::NotNumeric)?,
        gid.parse().map_err(|_| NumericUserError::NotNumeric)?,
    );
    if pair.0 == u32::MAX || pair.1 == u32::MAX {
        return Err(NumericUserError::Reserved);
    }
    Ok(pair)
}

fn access_for(mode: u32, owner_uid: u32, owner_gid: u32, uid: u32, gid: u32) -> (bool, bool) {
    if uid == 0 {
        return (true, true);
    }
    let bits = if uid == owner_uid {
        (mode >> 6) & 0o7
    } else if gid == owner_gid {
        (mode >> 3) & 0o7
    } else {
        mode & 0o7
    };
    (bits & 0o5 == 0o5, bits & 0o3 == 0o3)
}

fn file_type_prefix(_mode: u32) -> &'static str {
    "directory mode"
}

fn canonical_or_original(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

fn redact_path(path: &Path, share: bool, replacement: &str) -> String {
    if share {
        replacement.into()
    } else {
        path.display().to_string()
    }
}

fn report_path_redactions(options: &AuditOptions) -> Vec<(String, &'static str)> {
    let mut redactions = Vec::new();
    add_path_redactions(
        &mut redactions,
        options.config.as_deref(),
        "<devcontainer-config>",
    );
    add_path_redactions(
        &mut redactions,
        options.runtime_bin.as_deref(),
        "<runtime-bin>",
    );
    add_path_redactions(&mut redactions, options.workspace.as_deref(), "<workspace>");
    add_path_redactions(&mut redactions, Some(&options.project), "<project>");
    redactions.sort_by_key(|item| std::cmp::Reverse(item.0.len()));
    redactions.dedup_by(|left, right| left.0 == right.0);
    redactions
}

fn add_path_redactions(
    redactions: &mut Vec<(String, &'static str)>,
    path: Option<&Path>,
    replacement: &'static str,
) {
    let Some(path) = path else { return };
    let raw = path.display().to_string();
    if is_safe_redaction_candidate(&raw) {
        redactions.push((raw, replacement));
    }
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map(|current| current.join(path))
            .unwrap_or_else(|_| path.to_path_buf())
    };
    let absolute = canonical_or_original(&absolute).display().to_string();
    if is_safe_redaction_candidate(&absolute) {
        redactions.push((absolute, replacement));
    }
}

fn is_safe_redaction_candidate(value: &str) -> bool {
    !value.is_empty() && value != "." && value != "/"
}

fn redact_report(report: &mut AuditReport, redactions: &[(String, &'static str)]) {
    redact_string(&mut report.summary, redactions);
    redact_string(&mut report.config_path, redactions);
    redact_string(&mut report.runtime.kind, redactions);
    if let Some(version) = &mut report.runtime.version {
        redact_string(version, redactions);
    }
    redact_string(&mut report.identity.source, redactions);
    if let Some(workspace) = &mut report.workspace {
        redact_string(&mut workspace.path, redactions);
        redact_string(&mut workspace.mode, redactions);
    }
    for check in &mut report.checks {
        redact_string(&mut check.name, redactions);
        redact_string(&mut check.expected, redactions);
        redact_string(&mut check.observed, redactions);
        redact_string(&mut check.status, redactions);
    }
    for value in report
        .remediations
        .iter_mut()
        .chain(report.caveats.iter_mut())
        .chain(report.guarantees.iter_mut())
    {
        redact_string(value, redactions);
    }
}

fn redact_string(value: &mut String, redactions: &[(String, &'static str)]) {
    for (path, replacement) in redactions {
        if value.contains(path) {
            *value = value.replace(path, replacement);
        }
    }
}

fn unknown_report(
    share: bool,
    mut summary: String,
    config_path: String,
    checks: Vec<Check>,
    remediations: Vec<String>,
    caveats: Vec<String>,
    guarantees: Vec<String>,
) -> AuditReport {
    if share {
        summary = redact_text_paths(&summary);
    }
    AuditReport {
        schema_version: 1,
        verdict: Verdict::Unknown,
        summary,
        config_path,
        runtime: RuntimeReport {
            kind: "unknown".into(),
            version: None,
            rootless: None,
            inspected: false,
        },
        identity: IdentityReport {
            container_uid: None,
            container_gid: None,
            host_uid: None,
            host_gid: None,
            source: "unresolved".into(),
        },
        workspace: None,
        checks,
        remediations,
        caveats,
        guarantees,
    }
}

#[allow(clippy::too_many_arguments)]
fn unknown_report_with_runtime(
    share: bool,
    summary: String,
    config_path: String,
    runtime: runtime::RuntimeInfo,
    checks: Vec<Check>,
    remediations: Vec<String>,
    caveats: Vec<String>,
    guarantees: Vec<String>,
) -> AuditReport {
    let mut report = unknown_report(
        share,
        summary,
        config_path,
        checks,
        remediations,
        caveats,
        guarantees,
    );
    report.runtime = RuntimeReport::from(&runtime);
    report
}

fn redact_text_paths(value: &str) -> String {
    value
        .split_whitespace()
        .map(|word| {
            if word.starts_with('/') {
                "<local-path>"
            } else {
                word
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

impl From<&runtime::RuntimeInfo> for RuntimeReport {
    fn from(value: &runtime::RuntimeInfo) -> Self {
        Self {
            kind: value.kind.clone(),
            version: value.version.clone(),
            rootless: value.rootless,
            inspected: value.inspected,
        }
    }
}

pub fn render_text(report: &AuditReport, quiet: bool) -> String {
    if quiet {
        return format!("{}: {}\n", report.verdict.label(), report.summary);
    }
    let mut out = format!("MOUNT IDENTITY AUDIT{:>28}\n\n", report.verdict.label());
    out.push_str(&format!(
        "{:<15}{:<28}{:<34}{}\n",
        "CHECK", "EXPECTED", "OBSERVED", "STATUS"
    ));
    for check in &report.checks {
        out.push_str(&format!(
            "{:<15}{:<28}{:<34}{}\n",
            truncate(&check.name, 14),
            truncate(&check.expected, 27),
            truncate(&check.observed, 33),
            check.status.to_uppercase()
        ));
    }
    out.push_str(&format!("\n{}\n", report.summary));
    if !report.remediations.is_empty() {
        out.push_str("\nSAFE NEXT STEPS\n");
        for (index, item) in report.remediations.iter().enumerate() {
            out.push_str(&format!("{}. {item}\n", index + 1));
        }
    }
    out.push_str("\nLIMITS\n");
    for item in &report.caveats {
        out.push_str(&format!("- {item}\n"));
    }
    out.push('\n');
    for item in &report.guarantees {
        out.push_str(&format!("{item} "));
    }
    out.push('\n');
    out
}

fn truncate(value: &str, width: usize) -> String {
    if value.chars().count() <= width {
        return value.into();
    }
    value
        .chars()
        .take(width.saturating_sub(1))
        .collect::<String>()
        + "…"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_numeric_identity() {
        assert_eq!(parse_numeric_user("1000:1001"), Ok((1000, 1001)));
        assert_eq!(parse_numeric_user("42"), Err(NumericUserError::MissingGid));
        assert_eq!(parse_numeric_user("root"), Ok((0, 0)));
        assert_eq!(
            parse_numeric_user("vscode"),
            Err(NumericUserError::NotNumeric)
        );
        assert_eq!(
            parse_numeric_user("4294967295:1000"),
            Err(NumericUserError::Reserved)
        );
        assert_eq!(
            parse_numeric_user("1000:4294967295"),
            Err(NumericUserError::Reserved)
        );
    }

    #[test]
    fn computes_directory_permissions() {
        assert_eq!(access_for(0o775, 1000, 100, 2000, 100), (true, true));
        assert_eq!(access_for(0o755, 1000, 100, 2000, 200), (true, false));
        assert_eq!(access_for(0o700, 1000, 100, 2000, 100), (false, false));
    }
}

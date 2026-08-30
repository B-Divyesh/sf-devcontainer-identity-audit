use clap::Parser;
use mount_identity_audit::{AuditOptions, RuntimeChoice, audit, render_text};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Parser)]
#[command(
    name = "mount-identity-audit",
    version,
    about = "Predict Dev Container bind-mount access before startup",
    long_about = "Read Dev Container and Compose metadata, inspect Docker or rootless Podman without starting a container, and compare the mapped remote identity with workspace ownership."
)]
struct Cli {
    /// Project directory to inspect
    #[arg(default_value = ".")]
    project: PathBuf,

    /// Run the bundled sample in an isolated temporary project
    #[arg(long)]
    demo: bool,

    /// Dev Container configuration to read
    #[arg(long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Host workspace path when it cannot be inferred
    #[arg(long, value_name = "PATH")]
    workspace: Option<PathBuf>,

    /// Numeric intended container identity (UID:GID)
    #[arg(long, value_name = "UID:GID")]
    remote_user: Option<String>,

    /// Runtime adapter
    #[arg(long, value_enum, default_value_t = RuntimeChoice::Auto)]
    runtime: RuntimeChoice,

    /// Runtime executable (useful for wrappers and tests)
    #[arg(long, value_name = "PATH")]
    runtime_bin: Option<PathBuf>,

    /// Do not inspect an installed container runtime
    #[arg(long)]
    no_runtime: bool,

    /// Emit stable machine-readable JSON
    #[arg(long)]
    json: bool,

    /// Redact local path names in output intended for sharing
    #[arg(long)]
    share: bool,

    /// Print only the verdict and primary explanation
    #[arg(short, long)]
    quiet: bool,
}

fn main() {
    let cli = Cli::parse();
    let demo_project = if cli.demo {
        match create_demo_project() {
            Ok(path) => Some(path),
            Err(error) => {
                eprintln!("UNKNOWN: The bundled demo could not be prepared: {error}");
                std::process::exit(2);
            }
        }
    } else {
        None
    };
    let report = audit(AuditOptions {
        project: demo_project.clone().unwrap_or(cli.project),
        config: if cli.demo { None } else { cli.config },
        workspace: if cli.demo { None } else { cli.workspace },
        remote_user: if cli.demo { None } else { cli.remote_user },
        runtime: if cli.demo {
            RuntimeChoice::Docker
        } else {
            cli.runtime
        },
        runtime_bin: if cli.demo { None } else { cli.runtime_bin },
        no_runtime: cli.demo || cli.no_runtime,
        share: cli.share,
    });
    if cli.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&report).expect("report is serializable")
        );
    } else {
        if let Some(path) = demo_project {
            println!(
                "DEMO — bundled sample data; your project was not read or changed.\nSample copy: {}\n",
                path.display()
            );
        }
        print!("{}", render_text(&report, cli.quiet));
    }
    std::process::exit(report.verdict.exit_code());
}

fn create_demo_project() -> Result<PathBuf, String> {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| error.to_string())?
        .as_nanos();
    let project = std::env::temp_dir().join(format!(
        "mount-identity-audit-demo-{}-{nonce}",
        std::process::id()
    ));
    let config_dir = project.join(".devcontainer");
    fs::create_dir_all(&config_dir)
        .map_err(|error| format!("cannot create {}: {error}", config_dir.display()))?;
    fs::set_permissions(&project, fs::Permissions::from_mode(0o755))
        .map_err(|error| format!("cannot set sample permissions: {error}"))?;
    fs::write(
        config_dir.join("devcontainer.json"),
        include_str!("../examples/mismatch/.devcontainer/devcontainer.json"),
    )
    .map_err(|error| format!("cannot write the sample configuration: {error}"))?;
    fs::write(
        project.join("README.md"),
        include_str!("../examples/mismatch/README.md"),
    )
    .map_err(|error| format!("cannot write the sample description: {error}"))?;
    Ok(project)
}

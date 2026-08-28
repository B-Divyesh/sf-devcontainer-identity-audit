use clap::Parser;
use mount_identity_audit::{AuditOptions, RuntimeChoice, audit, render_text};
use std::path::PathBuf;

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

    /// Dev Container configuration to read
    #[arg(long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Host workspace path when it cannot be inferred
    #[arg(long, value_name = "PATH")]
    workspace: Option<PathBuf>,

    /// Numeric intended container identity (UID or UID:GID)
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
    let report = audit(AuditOptions {
        project: cli.project,
        config: cli.config,
        workspace: cli.workspace,
        remote_user: cli.remote_user,
        runtime: cli.runtime,
        runtime_bin: cli.runtime_bin,
        no_runtime: cli.no_runtime,
        share: cli.share,
    });
    if cli.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&report).expect("report is serializable")
        );
    } else {
        print!("{}", render_text(&report, cli.quiet));
    }
    std::process::exit(report.verdict.exit_code());
}

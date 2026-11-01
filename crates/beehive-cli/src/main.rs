//! Command-line interface for `beehive`.
//!
//! The CLI owns argument parsing, configuration loading, and output. It talks
//! to ZigBee hardware only through [`beehive_core::Coordinator`].

mod cli;
mod commands;
mod config;

use anyhow::Context;
use beehive_core::{Coordinator, PermitJoin};
use clap::Parser;
use cli::{Backend, Cli, Command};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> std::process::ExitCode {
    let cli = Cli::parse();
    init_tracing(cli.verbose);

    match run(cli).await {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(error) => {
            tracing::error!("{error:#}");
            std::process::ExitCode::FAILURE
        }
    }
}

async fn run(cli: Cli) -> anyhow::Result<()> {
    let config = config::load(&cli.config)
        .with_context(|| format!("failed to load config {}", cli.config.display()))?;

    let backend = cli.backend.or(config.backend).unwrap_or_default();
    tracing::debug!(?backend, "selected backend");

    let mut coordinator = build_coordinator(backend, config.ezsp);

    coordinator
        .start()
        .await
        .context("failed to start coordinator")?;

    let result = dispatch(&cli.command, coordinator.as_ref()).await;

    if let Err(error) = coordinator.stop().await {
        tracing::warn!("failed to stop coordinator cleanly: {error}");
    }

    result
}

fn build_coordinator(
    backend: Backend,
    ezsp: Option<beehive_ezsp::EzspConfig>,
) -> Box<dyn Coordinator> {
    match backend {
        Backend::Mock => Box::new(beehive_core::mock::MockCoordinator::new()),
        Backend::Ezsp => Box::new(beehive_ezsp::EzspCoordinator::new(ezsp.unwrap_or_default())),
    }
}

async fn dispatch(command: &Command, coordinator: &dyn Coordinator) -> anyhow::Result<()> {
    match command {
        Command::PermitJoin { seconds, forever } => {
            let duration = if *forever {
                PermitJoin::forever()
            } else {
                PermitJoin::seconds(*seconds)
            };
            commands::permit_join::run(coordinator, duration).await
        }
        Command::Scan { channels } => commands::scan::run(coordinator, channels).await,
        Command::Devices => commands::devices::run(coordinator).await,
        Command::Send {
            address,
            endpoint,
            cluster,
            payload,
        } => commands::send::run(coordinator, *address, *endpoint, *cluster, payload).await,
    }
}

fn init_tracing(verbose: bool) {
    let default = if verbose { "debug" } else { "info" };
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();
}

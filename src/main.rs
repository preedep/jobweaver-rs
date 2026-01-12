use clap::Parser;
use tracing_subscriber::{EnvFilter, fmt};
use anyhow::Result;

use jobweaver::presentation::cli::Cli;
use jobweaver::presentation::cli::commands::AnalyzeCommand;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let filter = if cli.verbose {
        EnvFilter::new("jobweaver=debug,info")
    } else {
        EnvFilter::new("jobweaver=info")
    };

    fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();

    AnalyzeCommand::execute(
        &cli.input,
        &cli.output,
        cli.should_generate_json(),
        cli.should_generate_csv(),
        cli.should_generate_html(),
        cli.should_generate_markdown(),
    )?;

    Ok(())
}

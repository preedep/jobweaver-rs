use clap::Parser;
use tracing_subscriber::{EnvFilter, fmt};
use anyhow::Result;

use jobweaver::presentation::cli::{Cli, Commands};
use jobweaver::presentation::cli::commands::{AnalyzeCommand, ExportSqliteCommand};
use jobweaver::web::{WebConfig, start_web_server};

#[tokio::main]
async fn main() -> Result<()> {
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

    match &cli.command {
        Commands::Analyze { input, output, format } => {
            AnalyzeCommand::execute(
                input,
                output,
                format.should_generate_json(),
                format.should_generate_csv(),
                format.should_generate_html(),
                format.should_generate_markdown(),
            )?;
        }
        Commands::ExportSqlite { input, output } => {
            ExportSqliteCommand::execute(input, output)?;
        }
        Commands::Serve { database, port, host } => {
            let config = WebConfig::new(database.to_string_lossy().to_string())
                .with_port(*port)
                .with_host(host.clone());
            
            start_web_server(config).await?;
        }
    }

    Ok(())
}

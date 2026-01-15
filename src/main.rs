use clap::Parser;
use tracing_subscriber::{EnvFilter, fmt};
use anyhow::Result;
use std::env;

use jobweaver::presentation::cli::{Cli, Commands};
use jobweaver::presentation::cli::commands::{AnalyzeCommand, ExportSqliteCommand};
use jobweaver::web::{WebConfig, start_web_server};

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file if it exists
    // This allows configuration of authentication, security settings, etc.
    dotenv::dotenv().ok();
    
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
            // Create base configuration from command-line arguments
            let mut config = WebConfig::new(database.to_string_lossy().to_string())
                .with_port(*port)
                .with_host(host.clone());
            
            // Override configuration with environment variables from .env file
            // This allows secure configuration without hardcoding credentials
            
            // JWT and session secrets
            if let Ok(jwt_secret) = env::var("JWT_SECRET") {
                config.jwt_secret = jwt_secret;
            }
            if let Ok(session_key) = env::var("SESSION_KEY") {
                config.session_key = session_key;
            }
            
            // Authentication credentials (from .env)
            if let Ok(username) = env::var("AUTH_USERNAME") {
                config.auth_username = username;
            }
            if let Ok(password) = env::var("AUTH_PASSWORD") {
                config.auth_password = password;
            }
            
            // Login security settings
            if let Ok(max_attempts) = env::var("MAX_LOGIN_ATTEMPTS") {
                if let Ok(val) = max_attempts.parse::<u32>() {
                    config.max_login_attempts = val;
                }
            }
            if let Ok(lockout_duration) = env::var("LOCKOUT_DURATION_MINUTES") {
                if let Ok(val) = lockout_duration.parse::<u64>() {
                    config.lockout_duration_minutes = val;
                }
            }
            
            start_web_server(config).await?;
        }
    }

    Ok(())
}

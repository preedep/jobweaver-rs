pub mod commands;

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "jobweaver")]
#[command(author = "JobWeaver Team")]
#[command(version = "0.1.0")]
#[command(about = "Control-M XML analyzer for Airflow migration planning", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Analyze Control-M XML and generate migration reports")]
    Analyze {
        #[arg(short, long, value_name = "FILE", help = "Input Control-M XML file")]
        input: PathBuf,

        #[arg(short, long, value_name = "DIR", default_value = "output", help = "Output directory for reports")]
        output: PathBuf,

        #[arg(short, long, value_enum, default_value = "all", help = "Output format")]
        format: OutputFormat,
    },

    #[command(about = "Export Control-M XML raw data to SQLite database")]
    ExportSqlite {
        #[arg(short, long, value_name = "FILE", help = "Input Control-M XML file")]
        input: PathBuf,

        #[arg(short, long, value_name = "FILE", default_value = "controlm.db", help = "Output SQLite database file")]
        output: PathBuf,
    },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    Json,
    Csv,
    Html,
    Markdown,
    All,
}

impl OutputFormat {
    pub fn should_generate_json(&self) -> bool {
        matches!(self, OutputFormat::Json | OutputFormat::All)
    }

    pub fn should_generate_csv(&self) -> bool {
        matches!(self, OutputFormat::Csv | OutputFormat::All)
    }

    pub fn should_generate_html(&self) -> bool {
        matches!(self, OutputFormat::Html | OutputFormat::All)
    }

    pub fn should_generate_markdown(&self) -> bool {
        matches!(self, OutputFormat::Markdown | OutputFormat::All)
    }
}

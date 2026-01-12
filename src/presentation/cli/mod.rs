pub mod commands;

use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "jobweaver")]
#[command(author = "JobWeaver Team")]
#[command(version = "0.1.0")]
#[command(about = "Control-M XML analyzer for Airflow migration planning", long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE")]
    pub input: PathBuf,

    #[arg(short, long, value_name = "DIR", default_value = "output")]
    pub output: PathBuf,

    #[arg(short, long, value_enum, default_value = "all")]
    pub format: OutputFormat,

    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    Json,
    Csv,
    Html,
    Markdown,
    All,
}

impl Cli {
    pub fn should_generate_json(&self) -> bool {
        matches!(self.format, OutputFormat::Json | OutputFormat::All)
    }

    pub fn should_generate_csv(&self) -> bool {
        matches!(self.format, OutputFormat::Csv | OutputFormat::All)
    }

    pub fn should_generate_html(&self) -> bool {
        matches!(self.format, OutputFormat::Html | OutputFormat::All)
    }

    pub fn should_generate_markdown(&self) -> bool {
        matches!(self.format, OutputFormat::Markdown | OutputFormat::All)
    }
}

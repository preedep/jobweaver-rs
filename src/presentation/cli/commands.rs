use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use tracing::{info, warn};

use crate::infrastructure::parsers::ControlMXmlParser;
use crate::infrastructure::output::{JsonGenerator, CsvGenerator, HtmlGenerator, MarkdownGenerator};
use crate::application::use_cases::AnalyzeJobs;
use crate::presentation::dto::AnalysisOutput;

pub struct AnalyzeCommand;

impl AnalyzeCommand {
    pub fn execute<P: AsRef<Path>>(
        input_path: P,
        output_dir: P,
        generate_json: bool,
        generate_csv: bool,
        generate_html: bool,
        generate_markdown: bool,
    ) -> Result<()> {
        info!("Starting Control-M analysis...");
        
        let parser = ControlMXmlParser::new();
        info!("Parsing XML file: {:?}", input_path.as_ref());
        let folders = parser.parse_file(&input_path)
            .context("Failed to parse Control-M XML file")?;
        
        info!("Found {} folders", folders.len());
        let total_jobs: usize = folders.iter().map(|f| f.total_jobs()).sum();
        info!("Total jobs: {}", total_jobs);

        if total_jobs == 0 {
            warn!("No jobs found in the XML file");
            return Ok(());
        }

        info!("Analyzing jobs...");
        let analyze_use_case = AnalyzeJobs::new();
        let analysis_result = analyze_use_case.execute(&folders)
            .context("Failed to analyze jobs")?;

        let output = AnalysisOutput::from_analysis_result(analysis_result);

        fs::create_dir_all(&output_dir)
            .context("Failed to create output directory")?;

        if generate_json {
            let json_path = output_dir.as_ref().join("analysis.json");
            info!("Generating JSON report: {:?}", json_path);
            let json_gen = JsonGenerator::new();
            json_gen.generate(&output, &json_path)
                .context("Failed to generate JSON report")?;
        }

        if generate_csv {
            let csv_path = output_dir.as_ref().join("analysis.csv");
            info!("Generating CSV report: {:?}", csv_path);
            let csv_gen = CsvGenerator::new();
            csv_gen.generate(&output, &csv_path)
                .context("Failed to generate CSV report")?;
        }

        if generate_html {
            let html_path = output_dir.as_ref().join("analysis.html");
            info!("Generating HTML report: {:?}", html_path);
            let html_gen = HtmlGenerator::new();
            html_gen.generate(&output, &html_path)
                .context("Failed to generate HTML report")?;
        }

        if generate_markdown {
            let md_path = output_dir.as_ref().join("analysis.md");
            info!("Generating Markdown report: {:?}", md_path);
            let md_gen = MarkdownGenerator::new();
            md_gen.generate(&output, &md_path)
                .context("Failed to generate Markdown report")?;
        }

        info!("Analysis complete!");
        info!("Summary:");
        info!("  - Total Jobs: {}", output.summary.total_jobs);
        info!("  - Total Folders: {}", output.summary.total_folders);
        info!("  - Average Complexity: {:.2}", output.summary.average_complexity_score);
        info!("  - Migration Waves: {}", output.migration_waves.len());
        
        if output.summary.has_circular_dependencies {
            warn!("⚠️  Circular dependencies detected!");
        }

        Ok(())
    }
}

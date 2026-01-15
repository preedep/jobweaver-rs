//! CLI Commands module
//!
//! This module provides command implementations for the CLI interface,
//! including job analysis, report generation, and summary printing.

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use tracing::{info, warn};
use indicatif::{ProgressBar, ProgressStyle};

use crate::infrastructure::parsers::ControlMXmlParser;
use crate::infrastructure::output::{JsonGenerator, CsvGenerator, HtmlGenerator, MarkdownGenerator, SqliteExporter};
use crate::application::use_cases::AnalyzeJobs;
use crate::presentation::dto::AnalysisOutput;

/// Command for analyzing Control-M jobs and generating reports
///
/// Provides functionality to parse Control-M XML files, analyze jobs,
/// and generate various output formats (JSON, CSV, HTML, Markdown, SQLite).
pub struct AnalyzeCommand;

impl AnalyzeCommand {
    /// Prints a comprehensive analysis summary to the console
    ///
    /// Displays overall statistics, difficulty distribution, migration waves,
    /// top complex jobs, critical jobs, quick wins, and recommendations.
    ///
    /// # Arguments
    ///
    /// * `output` - Analysis output containing all job metrics
    fn print_summary(output: &AnalysisOutput) {
        println!("\n{}", "=".repeat(80));
        println!("📊 CONTROL-M MIGRATION ANALYSIS SUMMARY");
        println!("{}", "=".repeat(80));
        
        Self::print_overall_statistics(output);
        Self::print_difficulty_distribution(output);
        Self::print_migration_waves(output);
        Self::print_top_complex_jobs(output);
        
        let critical_jobs = Self::get_critical_jobs(output);
        Self::print_critical_jobs(&critical_jobs);
        
        let quick_wins = Self::get_quick_wins(output);
        Self::print_quick_wins(&quick_wins);
        
        Self::print_recommendations(output, &critical_jobs, &quick_wins);
        
        println!("\n{}", "=".repeat(80));
    }

    /// Prints overall statistics section
    ///
    /// Displays total jobs, folders, average complexity, and circular dependency warnings.
    ///
    /// # Arguments
    ///
    /// * `output` - Analysis output data
    fn print_overall_statistics(output: &AnalysisOutput) {
        println!("\n📈 Overall Statistics:");
        println!("  • Total Jobs:              {}", output.summary.total_jobs);
        println!("  • Total Folders:           {}", output.summary.total_folders);
        println!("  • Average Complexity:      {:.2}", output.summary.average_complexity_score);
        println!("  • Migration Waves:         {}", output.migration_waves.len());
        
        if output.summary.has_circular_dependencies {
            println!("  ⚠️  Circular Dependencies:  DETECTED");
        }
    }

    /// Calculates percentage for display purposes
    ///
    /// # Arguments
    ///
    /// * `count` - Numerator value
    /// * `total` - Denominator value
    ///
    /// # Returns
    ///
    /// Percentage as f64 (0.0 if total is 0)
    fn calculate_percentage(count: usize, total: usize) -> f64 {
        if total == 0 {
            0.0
        } else {
            (count as f64 / total as f64) * 100.0
        }
    }

    /// Prints migration difficulty distribution
    ///
    /// Shows breakdown of jobs by difficulty level (Easy, Medium, Hard)
    /// with counts and percentages.
    ///
    /// # Arguments
    ///
    /// * `output` - Analysis output data
    fn print_difficulty_distribution(output: &AnalysisOutput) {
        let easy_count = output.jobs.iter()
            .filter(|j| j.migration_difficulty == "Easy")
            .count();
        let medium_count = output.jobs.iter()
            .filter(|j| j.migration_difficulty == "Medium")
            .count();
        let hard_count = output.jobs.iter()
            .filter(|j| j.migration_difficulty == "Hard")
            .count();
        
        let total = output.summary.total_jobs;
        
        println!("\n🎯 Migration Difficulty Distribution:");
        println!("  • Easy (0-30):             {} jobs ({:.1}%)", 
            easy_count, Self::calculate_percentage(easy_count, total));
        println!("  • Medium (31-60):          {} jobs ({:.1}%)", 
            medium_count, Self::calculate_percentage(medium_count, total));
        println!("  • Hard (61+):              {} jobs ({:.1}%)", 
            hard_count, Self::calculate_percentage(hard_count, total));
    }

    /// Calculates average complexity for a specific migration wave
    ///
    /// # Arguments
    ///
    /// * `output` - Analysis output data
    /// * `wave_number` - Wave number to calculate average for
    ///
    /// # Returns
    ///
    /// Average complexity score for the wave (0.0 if no jobs)
    fn calculate_wave_average_complexity(output: &AnalysisOutput, wave_number: usize) -> f64 {
        let wave_jobs: Vec<_> = output.jobs.iter()
            .filter(|j| j.migration_wave == wave_number)
            .collect();
        
        if wave_jobs.is_empty() {
            0.0
        } else {
            wave_jobs.iter()
                .map(|j| j.complexity_score as f64)
                .sum::<f64>() / wave_jobs.len() as f64
        }
    }

    /// Prints migration waves breakdown
    ///
    /// Shows each wave with job count and average complexity.
    ///
    /// # Arguments
    ///
    /// * `output` - Analysis output data
    fn print_migration_waves(output: &AnalysisOutput) {
        println!("\n🌊 Migration Waves Breakdown:");
        for wave in &output.migration_waves {
            let avg_complexity = Self::calculate_wave_average_complexity(output, wave.wave);
            println!("  Wave {}: {} jobs (avg complexity: {:.1})", 
                wave.wave_number, wave.jobs.len(), avg_complexity);
        }
    }

    /// Prints top 10 most complex jobs
    ///
    /// Lists jobs sorted by complexity score in descending order.
    ///
    /// # Arguments
    ///
    /// * `output` - Analysis output data
    fn print_top_complex_jobs(output: &AnalysisOutput) {
        println!("\n🔥 Top 10 Most Complex Jobs:");
        let mut sorted_jobs = output.jobs.clone();
        sorted_jobs.sort_by(|a, b| b.complexity_score.cmp(&a.complexity_score));
        
        for (i, job) in sorted_jobs.iter().take(10).enumerate() {
            println!("  {}. {} (Complexity: {}, Wave: {})", 
                i + 1, job.job_name, job.complexity_score, job.migration_wave);
            println!("     Folder: {} | Difficulty: {}", 
                job.folder_name, job.migration_difficulty);
        }
    }

    /// Filters and returns critical jobs
    ///
    /// # Arguments
    ///
    /// * `output` - Analysis output data
    ///
    /// # Returns
    ///
    /// Vector of references to critical jobs
    fn get_critical_jobs(output: &AnalysisOutput) -> Vec<&crate::presentation::dto::JobOutput> {
        output.jobs.iter()
            .filter(|j| j.is_critical)
            .collect()
    }

    /// Prints critical jobs section
    ///
    /// Shows up to 5 critical jobs with their complexity and wave assignment.
    ///
    /// # Arguments
    ///
    /// * `critical_jobs` - Slice of critical job references
    fn print_critical_jobs(critical_jobs: &[&crate::presentation::dto::JobOutput]) {
        if critical_jobs.is_empty() {
            return;
        }
        
        println!("\n⚡ Critical Jobs ({} total):", critical_jobs.len());
        for (i, job) in critical_jobs.iter().take(5).enumerate() {
            println!("  {}. {} (Complexity: {}, Wave: {})", 
                i + 1, job.job_name, job.complexity_score, job.migration_wave);
        }
        
        if critical_jobs.len() > 5 {
            println!("  ... and {} more critical jobs", critical_jobs.len() - 5);
        }
    }

    /// Filters and returns quick win jobs
    ///
    /// Quick wins are easy jobs with no dependencies that can be migrated immediately.
    ///
    /// # Arguments
    ///
    /// * `output` - Analysis output data
    ///
    /// # Returns
    ///
    /// Vector of references to quick win jobs
    fn get_quick_wins(output: &AnalysisOutput) -> Vec<&crate::presentation::dto::JobOutput> {
        output.jobs.iter()
            .filter(|j| j.migration_difficulty == "Easy" && j.dependency_count == 0)
            .collect()
    }

    /// Prints quick wins section
    ///
    /// Shows up to 5 easy jobs with no dependencies.
    ///
    /// # Arguments
    ///
    /// * `quick_wins` - Slice of quick win job references
    fn print_quick_wins(quick_wins: &[&crate::presentation::dto::JobOutput]) {
        if quick_wins.is_empty() {
            return;
        }
        
        println!("\n✅ Quick Wins ({} jobs with no dependencies):", quick_wins.len());
        for (i, job) in quick_wins.iter().take(5).enumerate() {
            println!("  {}. {} (Complexity: {})", 
                i + 1, job.job_name, job.complexity_score);
        }
        
        if quick_wins.len() > 5 {
            println!("  ... and {} more quick wins", quick_wins.len() - 5);
        }
    }

    /// Prints migration recommendations
    ///
    /// Provides actionable recommendations based on analysis results.
    ///
    /// # Arguments
    ///
    /// * `output` - Analysis output data
    /// * `critical_jobs` - Slice of critical job references
    /// * `quick_wins` - Slice of quick win job references
    fn print_recommendations(
        output: &AnalysisOutput,
        critical_jobs: &[&crate::presentation::dto::JobOutput],
        quick_wins: &[&crate::presentation::dto::JobOutput],
    ) {
        println!("\n💡 Recommendations:");
        println!("  • Start with Wave 1 ({} jobs) - Low complexity, no dependencies", 
            output.migration_waves.get(0).map(|w| w.jobs.len()).unwrap_or(0));
        println!("  • Review {} critical jobs carefully before migration", critical_jobs.len());
        
        if output.summary.has_circular_dependencies {
            println!("  • ⚠️  Resolve circular dependencies before migration");
        }
        
        println!("  • {} quick wins can be migrated immediately", quick_wins.len());
    }

    /// Executes the analyze command
    ///
    /// Parses Control-M XML file, analyzes jobs, generates reports in requested formats,
    /// and prints a summary to the console.
    ///
    /// # Arguments
    ///
    /// * `input_path` - Path to the Control-M XML file
    /// * `output_dir` - Directory for output files
    /// * `generate_json` - Whether to generate JSON report
    /// * `generate_csv` - Whether to generate CSV report
    /// * `generate_html` - Whether to generate HTML report
    /// * `generate_markdown` - Whether to generate Markdown report
    ///
    /// # Returns
    ///
    /// Result indicating success or error
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - XML file cannot be parsed
    /// - Analysis fails
    /// - Output directory cannot be created
    /// - Report generation fails
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
            // Generate main CSV
            let csv_path = output_dir.as_ref().join("analysis.csv");
            info!("Generating CSV report: {:?}", csv_path);
            let csv_gen = CsvGenerator::new();
            csv_gen.generate(&output, &csv_path)
                .context("Failed to generate CSV report")?;
            
            // Generate separate CSV for each wave
            info!("Generating per-wave CSV reports...");
            for wave in &output.migration_waves {
                let wave_csv_path = output_dir.as_ref().join(format!("wave_{}.csv", wave.wave_number));
                let wave_jobs: Vec<_> = output.jobs.iter()
                    .filter(|j| j.migration_wave == wave.wave)
                    .cloned()
                    .collect();
                let wave_output = AnalysisOutput {
                    summary: output.summary.clone(),
                    jobs: wave_jobs,
                    migration_waves: vec![wave.clone()],
                };
                csv_gen.generate(&wave_output, &wave_csv_path)
                    .with_context(|| format!("Failed to generate CSV for wave {}", wave.wave_number))?;
            }
            info!("Generated {} wave-specific CSV files", output.migration_waves.len());
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
        
        // Print detailed summary
        Self::print_summary(&output);
        
        Ok(())
    }
}

pub struct ExportSqliteCommand;

impl ExportSqliteCommand {
    pub fn execute<P: AsRef<Path>>(
        input_path: P,
        output_db_path: P,
    ) -> Result<()> {
        info!("Starting Control-M XML to SQLite export...");
        
        // Create spinner for parsing
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                .template("{spinner:.cyan} {msg}")
                .unwrap()
        );
        spinner.set_message("📖 Parsing XML file...");
        spinner.enable_steady_tick(std::time::Duration::from_millis(80));
        
        let parser = ControlMXmlParser::new();
        info!("Parsing XML file: {:?}", input_path.as_ref());
        let folders = parser.parse_file(&input_path)
            .context("Failed to parse Control-M XML file")?;
        
        spinner.finish_with_message(format!("✓ Found {} folders", folders.len()));
        
        let total_jobs: usize = folders.iter().map(|f| f.total_jobs()).sum();
        info!("Total jobs: {}", total_jobs);

        if total_jobs == 0 {
            warn!("No jobs found in the XML file");
            return Ok(());
        }

        // Create progress bar for export
        let pb = ProgressBar::new(total_jobs as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
                .unwrap()
                .progress_chars("█▓▒░ ")
        );
        pb.set_message("🚀 Starting export...");

        info!("Creating SQLite database: {:?}", output_db_path.as_ref());
        
        let pb_clone = pb.clone();
        let exporter = SqliteExporter::new(&output_db_path)
            .context("Failed to create SQLite database")?
            .with_progress_callback(move |msg: &str| {
                if msg.starts_with("  → Job:") {
                    pb_clone.inc(1);
                    pb_clone.set_message(msg.trim_start_matches("  → ").to_string());
                } else if msg.starts_with("Exporting folder") {
                    pb_clone.set_message(format!("📁 {}", msg));
                } else {
                    pb_clone.set_message(msg.to_string());
                }
            });

        info!("Exporting folders and jobs to SQLite...");
        exporter.export_folders(&folders)
            .context("Failed to export data to SQLite")?;

        pb.finish_with_message("✓ Export completed!");

        let stats = exporter.get_statistics()
            .context("Failed to get database statistics")?;

        println!("\n{}", "=".repeat(80));
        println!("✅ SQLITE EXPORT COMPLETED");
        println!("{}", "=".repeat(80));
        println!("\n📊 Export Statistics:");
        println!("  • Database file:           {:?}", output_db_path.as_ref());
        println!("  • Folders exported:        {}", stats.folder_count);
        println!("  • Jobs exported:           {}", stats.job_count);
        println!("  • In conditions:           {}", stats.in_condition_count);
        println!("  • Out conditions:          {}", stats.out_condition_count);
        println!("  • Control resources:       {}", stats.control_resource_count);
        println!("\n💡 You can now query the database using SQLite tools:");
        println!("  sqlite3 {:?}", output_db_path.as_ref());
        println!("\n📋 Example queries:");
        println!("  • List all jobs:           SELECT job_name, folder_name FROM jobs;");
        println!("  • Critical jobs:           SELECT job_name FROM jobs WHERE critical = 1;");
        println!("  • Jobs with dependencies:  SELECT j.job_name, COUNT(ic.id) as dep_count");
        println!("                             FROM jobs j LEFT JOIN in_conditions ic ON j.id = ic.job_id");
        println!("                             GROUP BY j.id HAVING dep_count > 0;");
        println!("{}", "=".repeat(80));

        info!("SQLite export complete!");
        
        Ok(())
    }
}

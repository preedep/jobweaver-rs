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
    fn print_summary(output: &AnalysisOutput) {
        println!("\n{}", "=".repeat(80));
        println!("📊 CONTROL-M MIGRATION ANALYSIS SUMMARY");
        println!("{}", "=".repeat(80));
        
        // Overall Statistics
        println!("\n📈 Overall Statistics:");
        println!("  • Total Jobs:              {}", output.summary.total_jobs);
        println!("  • Total Folders:           {}", output.summary.total_folders);
        println!("  • Average Complexity:      {:.2}", output.summary.average_complexity_score);
        println!("  • Migration Waves:         {}", output.migration_waves.len());
        
        if output.summary.has_circular_dependencies {
            println!("  ⚠️  Circular Dependencies:  DETECTED");
        }
        
        // Difficulty Distribution
        let easy_count = output.jobs.iter().filter(|j| j.migration_difficulty == "Easy").count();
        let medium_count = output.jobs.iter().filter(|j| j.migration_difficulty == "Medium").count();
        let hard_count = output.jobs.iter().filter(|j| j.migration_difficulty == "Hard").count();
        
        println!("\n🎯 Migration Difficulty Distribution:");
        println!("  • Easy (0-30):             {} jobs ({:.1}%)", 
            easy_count, (easy_count as f64 / output.summary.total_jobs as f64) * 100.0);
        println!("  • Medium (31-60):          {} jobs ({:.1}%)", 
            medium_count, (medium_count as f64 / output.summary.total_jobs as f64) * 100.0);
        println!("  • Hard (61+):              {} jobs ({:.1}%)", 
            hard_count, (hard_count as f64 / output.summary.total_jobs as f64) * 100.0);
        
        // Migration Waves Summary
        println!("\n🌊 Migration Waves Breakdown:");
        for wave in &output.migration_waves {
            let wave_jobs: Vec<_> = output.jobs.iter()
                .filter(|j| j.migration_wave == wave.wave)
                .collect();
            let avg_complexity: f64 = if !wave_jobs.is_empty() {
                wave_jobs.iter()
                    .map(|j| j.complexity_score as f64)
                    .sum::<f64>() / wave_jobs.len() as f64
            } else {
                0.0
            };
            
            println!("  Wave {}: {} jobs (avg complexity: {:.1})", 
                wave.wave_number, wave.jobs.len(), avg_complexity);
        }
        
        // Top 10 Most Complex Jobs
        println!("\n🔥 Top 10 Most Complex Jobs:");
        let mut sorted_jobs = output.jobs.clone();
        sorted_jobs.sort_by(|a, b| b.complexity_score.cmp(&a.complexity_score));
        
        for (i, job) in sorted_jobs.iter().take(10).enumerate() {
            println!("  {}. {} (Complexity: {}, Wave: {})", 
                i + 1, job.job_name, job.complexity_score, job.migration_wave);
            println!("     Folder: {} | Difficulty: {}", 
                job.folder_name, job.migration_difficulty);
        }
        
        // Critical Jobs (High complexity + Critical flag)
        let critical_jobs: Vec<_> = output.jobs.iter()
            .filter(|j| j.is_critical)
            .collect();
        
        if !critical_jobs.is_empty() {
            println!("\n⚡ Critical Jobs ({} total):", critical_jobs.len());
            for (i, job) in critical_jobs.iter().take(5).enumerate() {
                println!("  {}. {} (Complexity: {}, Wave: {})", 
                    i + 1, job.job_name, job.complexity_score, job.migration_wave);
            }
            if critical_jobs.len() > 5 {
                println!("  ... and {} more critical jobs", critical_jobs.len() - 5);
            }
        }
        
        // Quick Wins
        let quick_wins: Vec<_> = output.jobs.iter()
            .filter(|j| j.migration_difficulty == "Easy" && j.dependency_count == 0)
            .collect();
        
        if !quick_wins.is_empty() {
            println!("\n✅ Quick Wins ({} jobs with no dependencies):", quick_wins.len());
            for (i, job) in quick_wins.iter().take(5).enumerate() {
                println!("  {}. {} (Complexity: {})", 
                    i + 1, job.job_name, job.complexity_score);
            }
            if quick_wins.len() > 5 {
                println!("  ... and {} more quick wins", quick_wins.len() - 5);
            }
        }
        
        // Recommendations
        println!("\n💡 Recommendations:");
        println!("  • Start with Wave 1 ({} jobs) - Low complexity, no dependencies", 
            output.migration_waves.get(0).map(|w| w.jobs.len()).unwrap_or(0));
        println!("  • Review {} critical jobs carefully before migration", critical_jobs.len());
        if output.summary.has_circular_dependencies {
            println!("  • ⚠️  Resolve circular dependencies before migration");
        }
        println!("  • {} quick wins can be migrated immediately", quick_wins.len());
        
        println!("\n{}", "=".repeat(80));
    }

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

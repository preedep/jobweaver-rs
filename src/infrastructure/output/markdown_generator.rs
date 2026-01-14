use anyhow::Result;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::presentation::dto::AnalysisOutput;

pub struct MarkdownGenerator;

impl MarkdownGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate<P: AsRef<Path>>(&self, output: &AnalysisOutput, path: P) -> Result<()> {
        let markdown = self.generate_string(output)?;
        let mut file = File::create(path)?;
        file.write_all(markdown.as_bytes())?;
        Ok(())
    }

    pub fn generate_string(&self, output: &AnalysisOutput) -> Result<String> {
        let mut md = String::new();

        md.push_str("# Control-M to Airflow Migration Analysis Report\n\n");
        
        md.push_str("## Executive Summary\n\n");
        md.push_str(&format!("- **Analysis Date**: {}\n", output.summary.analysis_date));
        md.push_str(&format!("- **Total Jobs**: {}\n", output.summary.total_jobs));
        md.push_str(&format!("- **Total Folders**: {}\n", output.summary.total_folders));
        md.push_str(&format!("- **Average Complexity Score**: {:.2}\n", output.summary.average_complexity_score));
        md.push_str(&format!("- **Circular Dependencies**: {}\n\n", 
            if output.summary.has_circular_dependencies { "⚠️ Yes" } else { "✅ No" }));

        md.push_str("## Migration Waves\n\n");
        for wave in &output.migration_waves {
            md.push_str(&format!("### Wave {} - {} jobs\n", wave.wave, wave.jobs.len()));
            md.push_str(&format!("**Reason**: {}\n\n", wave.reason));
            md.push_str("**Jobs**:\n");
            for job in &wave.jobs {
                md.push_str(&format!("- {}\n", job));
            }
            md.push_str("\n");
        }

        md.push_str("## Job Details\n\n");
        md.push_str("| Job Name | Folder | Complexity | Difficulty | Priority | Dependencies | Effort (hrs) |\n");
        md.push_str("|----------|--------|------------|------------|----------|--------------|-------------|\n");
        
        for job in &output.jobs {
            md.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {} |\n",
                job.job_name,
                job.folder,
                job.complexity_score,
                job.migration_difficulty,
                job.migration_priority,
                job.metrics.dependency_count,
                job.airflow_mapping.estimated_effort_hours
            ));
        }

        md.push_str("\n## High-Risk Jobs\n\n");
        let high_risk_jobs: Vec<_> = output.jobs.iter()
            .filter(|j| j.complexity_score > 60 || j.metrics.is_critical)
            .collect();

        if high_risk_jobs.is_empty() {
            md.push_str("No high-risk jobs identified.\n\n");
        } else {
            for job in high_risk_jobs {
                md.push_str(&format!("### {}\n", job.job_name));
                md.push_str(&format!("- **Complexity Score**: {}\n", job.complexity_score));
                md.push_str(&format!("- **Migration Difficulty**: {}\n", job.migration_difficulty));
                md.push_str("- **Risks**:\n");
                for risk in &job.risks {
                    md.push_str(&format!("  - {}\n", risk));
                }
                md.push_str("\n");
            }
        }

        md.push_str("## Recommendations\n\n");
        md.push_str("1. Start with Wave 1 jobs for quick wins and team familiarization\n");
        md.push_str("2. Address circular dependencies before migration\n");
        md.push_str("3. Plan extra time for critical and high-complexity jobs\n");
        md.push_str("4. Consider breaking down jobs with complexity > 80 into smaller DAGs\n");
        md.push_str("5. Establish thorough testing procedures for cyclic jobs\n\n");

        Ok(md)
    }
}

impl Default for MarkdownGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::presentation::dto::SummaryOutput;

    #[test]
    fn test_generate_string() {
        let generator = MarkdownGenerator::new();
        let output = AnalysisOutput {
            summary: SummaryOutput {
                total_jobs: 10,
                total_folders: 2,
                analysis_date: "2026-01-12".to_string(),
                average_complexity_score: 42.5,
                has_circular_dependencies: false,
            },
            jobs: vec![],
            migration_waves: vec![],
        };

        let md = generator.generate_string(&output).unwrap();
        assert!(md.contains("# Control-M to Airflow Migration Analysis Report"));
        assert!(md.contains("Total Jobs"));
    }
}

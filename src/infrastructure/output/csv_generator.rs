use anyhow::Result;
use std::fs::File;
use std::path::Path;
use csv::Writer;
use crate::presentation::dto::AnalysisOutput;

pub struct CsvGenerator;

impl CsvGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate<P: AsRef<Path>>(&self, output: &AnalysisOutput, path: P) -> Result<()> {
        let file = File::create(path)?;
        let mut wtr = Writer::from_writer(file);

        wtr.write_record(&[
            "Job Name",
            "Folder",
            "Complexity Score",
            "Migration Difficulty",
            "Priority",
            "Dependencies",
            "Critical",
            "Cyclic",
            "Effort Hours",
            "Wave",
        ])?;

        for job in &output.jobs {
            let wave = output.migration_waves
                .iter()
                .find(|w| w.jobs.contains(&job.job_name))
                .map(|w| w.wave.to_string())
                .unwrap_or_else(|| "N/A".to_string());

            wtr.write_record(&[
                &job.job_name,
                &job.folder,
                &job.complexity_score.to_string(),
                &job.migration_difficulty,
                &job.migration_priority.to_string(),
                &job.metrics.dependency_count.to_string(),
                &job.metrics.is_critical.to_string(),
                &job.metrics.is_cyclic.to_string(),
                &job.airflow_mapping.estimated_effort_hours.to_string(),
                &wave,
            ])?;
        }

        wtr.flush()?;
        Ok(())
    }
}

impl Default for CsvGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_generator_creation() {
        let _generator = CsvGenerator::new();
        assert!(true);
    }
}

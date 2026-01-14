use anyhow::Result;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::presentation::dto::AnalysisOutput;

pub struct JsonGenerator;

impl JsonGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate<P: AsRef<Path>>(&self, output: &AnalysisOutput, path: P) -> Result<()> {
        let json = serde_json::to_string_pretty(output)?;
        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn generate_string(&self, output: &AnalysisOutput) -> Result<String> {
        Ok(serde_json::to_string_pretty(output)?)
    }
}

impl Default for JsonGenerator {
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
        let generator = JsonGenerator::new();
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

        let json = generator.generate_string(&output).unwrap();
        assert!(json.contains("total_jobs"));
        assert!(json.contains("10"));
    }
}

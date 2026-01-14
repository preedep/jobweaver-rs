use anyhow::Result;
use crate::domain::entities::{Job, Folder};
use crate::application::use_cases::{
    CalculateComplexity, BuildDependencyGraph, DetermineMigrationWaves,
};

pub struct AnalyzeJobs {
    calculate_complexity: CalculateComplexity,
    determine_waves: DetermineMigrationWaves,
}

impl AnalyzeJobs {
    pub fn new() -> Self {
        Self {
            calculate_complexity: CalculateComplexity::new(),
            determine_waves: DetermineMigrationWaves::new(),
        }
    }

    pub fn execute(&self, folders: &[Folder]) -> Result<AnalysisResult> {
        let all_jobs: Vec<&Job> = folders.iter()
            .flat_map(|f| f.all_jobs())
            .collect();

        let mut complexity_results = self.calculate_complexity.execute_batch(&all_jobs);

        let mut build_graph = BuildDependencyGraph::new();
        let graph_result = build_graph.execute(&all_jobs);

        let migration_waves = self.determine_waves.execute(&complexity_results);

        // Update each job with its wave number
        for wave in &migration_waves {
            for job_name in &wave.jobs {
                if let Some(result) = complexity_results.iter_mut().find(|r| &r.job_name == job_name) {
                    result.migration_wave = wave.wave;
                }
            }
        }

        let total_jobs = all_jobs.len();
        let total_folders = folders.len();
        let average_complexity: f64 = if !complexity_results.is_empty() {
            complexity_results.iter()
                .map(|r| r.complexity_score.value() as f64)
                .sum::<f64>() / complexity_results.len() as f64
        } else {
            0.0
        };

        Ok(AnalysisResult {
            total_jobs,
            total_folders,
            average_complexity,
            complexity_results,
            migration_waves,
            has_circular_dependencies: graph_result.has_circular_dependencies,
        })
    }
}

impl Default for AnalyzeJobs {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct AnalysisResult {
    pub total_jobs: usize,
    pub total_folders: usize,
    pub average_complexity: f64,
    pub complexity_results: Vec<super::calculate_complexity::JobComplexityResult>,
    pub migration_waves: Vec<super::determine_migration_waves::MigrationWave>,
    pub has_circular_dependencies: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::FolderType;

    #[test]
    fn test_analyze_empty_folders() {
        let use_case = AnalyzeJobs::new();
        let folders = vec![];
        
        let result = use_case.execute(&folders).unwrap();
        assert_eq!(result.total_jobs, 0);
        assert_eq!(result.total_folders, 0);
    }

    #[test]
    fn test_analyze_folders_with_jobs() {
        let use_case = AnalyzeJobs::new();
        
        let mut folder = Folder::new("TEST_FOLDER".to_string(), FolderType::Simple);
        let job1 = Job::new("JOB1".to_string(), "TEST_FOLDER".to_string());
        let job2 = Job::new("JOB2".to_string(), "TEST_FOLDER".to_string());
        folder.add_job(job1);
        folder.add_job(job2);
        
        let folders = vec![folder];
        let result = use_case.execute(&folders).unwrap();
        
        assert_eq!(result.total_jobs, 2);
        assert_eq!(result.total_folders, 1);
        assert_eq!(result.complexity_results.len(), 2);
    }
}

use crate::domain::entities::Job;
use crate::domain::value_objects::{ComplexityScore, MigrationDifficulty, MigrationPriority};
use crate::application::services::ComplexityCalculator;

pub struct CalculateComplexity {
    calculator: ComplexityCalculator,
}

impl CalculateComplexity {
    pub fn new() -> Self {
        Self {
            calculator: ComplexityCalculator::new(),
        }
    }

    pub fn execute(&self, job: &Job) -> JobComplexityResult {
        let complexity_score = self.calculator.calculate_job_complexity(job);
        let migration_difficulty = self.calculator.calculate_migration_difficulty(job);
        let migration_priority = self.calculator.calculate_migration_priority(job);

        JobComplexityResult {
            job_name: job.job_name.clone(),
            folder_name: job.folder_name.clone(),
            complexity_score,
            migration_difficulty,
            migration_priority,
            migration_wave: 0, // Will be set by wave determination
            dependency_count: job.dependency_count(),
            is_critical: job.is_critical(),
            is_cyclic: job.cyclic,
        }
    }

    pub fn execute_batch(&self, jobs: &[&Job]) -> Vec<JobComplexityResult> {
        jobs.iter().map(|job| self.execute(job)).collect()
    }
}

impl Default for CalculateComplexity {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct JobComplexityResult {
    pub job_name: String,
    pub folder_name: String,
    pub complexity_score: ComplexityScore,
    pub migration_difficulty: MigrationDifficulty,
    pub migration_priority: MigrationPriority,
    pub migration_wave: usize,
    pub dependency_count: usize,
    pub is_critical: bool,
    pub is_cyclic: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute() {
        let use_case = CalculateComplexity::new();
        let job = Job::new("TEST_JOB".to_string(), "TEST_FOLDER".to_string());
        
        let result = use_case.execute(&job);
        assert_eq!(result.job_name, "TEST_JOB");
        assert_eq!(result.folder_name, "TEST_FOLDER");
    }

    #[test]
    fn test_execute_batch() {
        let use_case = CalculateComplexity::new();
        let job1 = Job::new("JOB1".to_string(), "FOLDER".to_string());
        let job2 = Job::new("JOB2".to_string(), "FOLDER".to_string());
        
        let jobs = vec![&job1, &job2];
        let results = use_case.execute_batch(&jobs);
        
        assert_eq!(results.len(), 2);
    }
}

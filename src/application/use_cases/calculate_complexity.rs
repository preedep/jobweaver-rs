//! Calculate Complexity use case module
//!
//! This module provides the use case for calculating job complexity metrics.
//! It orchestrates the complexity calculation service to analyze jobs.

use crate::domain::entities::Job;
use crate::domain::value_objects::{ComplexityScore, MigrationDifficulty, MigrationPriority};
use crate::application::services::ComplexityCalculator;

/// Use case for calculating job complexity
///
/// This use case encapsulates the business logic for analyzing job complexity
/// and producing complexity results that can be used for migration planning.
pub struct CalculateComplexity {
    /// The complexity calculator service
    calculator: ComplexityCalculator,
}

impl CalculateComplexity {
    /// Creates a new CalculateComplexity use case
    ///
    /// # Returns
    ///
    /// A new CalculateComplexity instance with an initialized calculator
    pub fn new() -> Self {
        Self {
            calculator: ComplexityCalculator::new(),
        }
    }

    /// Executes complexity calculation for a single job
    ///
    /// # Arguments
    ///
    /// * `job` - The job to analyze
    ///
    /// # Returns
    ///
    /// A JobComplexityResult containing all calculated metrics
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

    /// Executes complexity calculation for multiple jobs
    ///
    /// # Arguments
    ///
    /// * `jobs` - Slice of job references to analyze
    ///
    /// # Returns
    ///
    /// Vector of JobComplexityResult for all jobs
    pub fn execute_batch(&self, jobs: &[&Job]) -> Vec<JobComplexityResult> {
        jobs.iter().map(|job| self.execute(job)).collect()
    }
}

impl Default for CalculateComplexity {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of job complexity analysis
///
/// Contains all calculated metrics for a job including complexity score,
/// migration difficulty, priority, and other relevant information.
#[derive(Debug, Clone)]
pub struct JobComplexityResult {
    /// Name of the analyzed job
    pub job_name: String,
    /// Folder containing the job
    pub folder_name: String,
    /// Calculated complexity score
    pub complexity_score: ComplexityScore,
    /// Derived migration difficulty level
    pub migration_difficulty: MigrationDifficulty,
    /// Calculated migration priority
    pub migration_priority: MigrationPriority,
    /// Assigned migration wave (set by wave determination)
    pub migration_wave: usize,
    /// Number of dependencies
    pub dependency_count: usize,
    /// Whether the job is critical
    pub is_critical: bool,
    /// Whether the job is cyclic
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

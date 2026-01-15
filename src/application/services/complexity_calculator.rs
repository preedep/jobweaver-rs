//! Complexity Calculator service module
//!
//! This service provides functionality to calculate complexity scores,
//! migration difficulty, and migration priority for jobs.

use crate::domain::entities::Job;
use crate::domain::value_objects::{ComplexityScore, MigrationDifficulty, MigrationPriority};

/// Service for calculating job complexity metrics
///
/// The ComplexityCalculator analyzes various aspects of a job to determine
/// its complexity score, migration difficulty, and migration priority.
pub struct ComplexityCalculator;

impl ComplexityCalculator {
    /// Creates a new ComplexityCalculator instance
    ///
    /// # Returns
    ///
    /// A new ComplexityCalculator
    pub fn new() -> Self {
        Self
    }

    /// Calculates the complexity score for a job
    ///
    /// This method analyzes all aspects of a job including dependencies,
    /// conditions, resources, variables, and scheduling to produce a
    /// comprehensive complexity score.
    ///
    /// # Arguments
    ///
    /// * `job` - The job to analyze
    ///
    /// # Returns
    ///
    /// A ComplexityScore representing the job's overall complexity
    pub fn calculate_job_complexity(&self, job: &Job) -> ComplexityScore {
        // Gather all complexity metrics from the job
        let dependency_count = job.dependency_count();
        let dependency_depth = self.estimate_dependency_depth(job);
        let in_conditions = job.in_conditions.len();
        let out_conditions = job.out_conditions.len();
        let variables_count = job.variables.len() + job.auto_edits.len();
        let on_conditions = job.on_conditions.len();
        let on_conditions_complexity: usize = job.on_conditions
            .iter()
            .map(|oc| oc.complexity())
            .sum();
        let is_cyclic = job.cyclic;
        let quantitative_resources = job.quantitative_resources.len();
        let control_resources = job.control_resources.len();
        let scheduling_complexity = job.scheduling.complexity();

        ComplexityScore::from_metrics(
            dependency_count,
            dependency_depth,
            in_conditions,
            out_conditions,
            variables_count,
            on_conditions,
            on_conditions_complexity,
            is_cyclic,
            quantitative_resources,
            control_resources,
            scheduling_complexity,
        )
    }

    /// Calculates the migration difficulty for a job
    ///
    /// Migration difficulty is derived from the complexity score and
    /// categorizes jobs into Easy, Medium, or Hard difficulty levels.
    ///
    /// # Arguments
    ///
    /// * `job` - The job to analyze
    ///
    /// # Returns
    ///
    /// A MigrationDifficulty level (Easy, Medium, or Hard)
    pub fn calculate_migration_difficulty(&self, job: &Job) -> MigrationDifficulty {
        let complexity = self.calculate_job_complexity(job);
        MigrationDifficulty::from_complexity_score(complexity)
    }

    /// Calculates the migration priority for a job
    ///
    /// Priority is based on complexity (easier jobs get higher priority),
    /// criticality (critical jobs get bonus priority), and dependencies
    /// (fewer dependencies means higher priority).
    ///
    /// # Arguments
    ///
    /// * `job` - The job to analyze
    ///
    /// # Returns
    ///
    /// A MigrationPriority value (higher means more urgent to migrate)
    pub fn calculate_migration_priority(&self, job: &Job) -> MigrationPriority {
        let complexity = self.calculate_job_complexity(job);
        let is_critical = job.is_critical();
        let dependency_count = job.dependency_count();
        
        MigrationPriority::calculate(complexity, is_critical, dependency_count)
    }

    /// Estimates the dependency depth for a job
    ///
    /// This is a simplified estimation. A more accurate calculation would
    /// require analyzing the full dependency graph.
    ///
    /// # Arguments
    ///
    /// * `job` - The job to analyze
    ///
    /// # Returns
    ///
    /// Estimated dependency depth (0 if no dependencies, 1 if has dependencies)
    fn estimate_dependency_depth(&self, job: &Job) -> usize {
        if job.in_conditions.is_empty() && job.control_resources.is_empty() {
            0
        } else {
            1
        }
    }
}

impl Default for ComplexityCalculator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::Condition;

    #[test]
    fn test_calculate_simple_job_complexity() {
        let calculator = ComplexityCalculator::new();
        let job = Job::new("SIMPLE_JOB".to_string(), "FOLDER".to_string());
        
        let complexity = calculator.calculate_job_complexity(&job);
        assert_eq!(complexity.value(), 0);
    }

    #[test]
    fn test_calculate_complex_job_complexity() {
        let calculator = ComplexityCalculator::new();
        let mut job = Job::new("COMPLEX_JOB".to_string(), "FOLDER".to_string());
        
        job.in_conditions.push(Condition::new_in("COND1".to_string()));
        job.in_conditions.push(Condition::new_in("COND2".to_string()));
        job.out_conditions.push(Condition::new_out("COND3".to_string()));
        job.cyclic = true;
        job.variables.insert("VAR1".to_string(), "VALUE1".to_string());
        
        let complexity = calculator.calculate_job_complexity(&job);
        assert!(complexity.value() > 0);
    }

    #[test]
    fn test_calculate_migration_difficulty() {
        let calculator = ComplexityCalculator::new();
        let job = Job::new("TEST_JOB".to_string(), "FOLDER".to_string());
        
        let difficulty = calculator.calculate_migration_difficulty(&job);
        assert_eq!(difficulty, MigrationDifficulty::Easy);
    }
}

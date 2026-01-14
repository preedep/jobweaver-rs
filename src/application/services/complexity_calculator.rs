use crate::domain::entities::Job;
use crate::domain::value_objects::{ComplexityScore, MigrationDifficulty, MigrationPriority};

pub struct ComplexityCalculator;

impl ComplexityCalculator {
    pub fn new() -> Self {
        Self
    }

    pub fn calculate_job_complexity(&self, job: &Job) -> ComplexityScore {
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

    pub fn calculate_migration_difficulty(&self, job: &Job) -> MigrationDifficulty {
        let complexity = self.calculate_job_complexity(job);
        MigrationDifficulty::from_complexity_score(complexity)
    }

    pub fn calculate_migration_priority(&self, job: &Job) -> MigrationPriority {
        let complexity = self.calculate_job_complexity(job);
        let is_critical = job.is_critical();
        let dependency_count = job.dependency_count();
        
        MigrationPriority::calculate(complexity, is_critical, dependency_count)
    }

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

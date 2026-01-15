//! Dependency entity module
//!
//! This module defines job dependencies and their types.
//! Dependencies represent relationships between jobs in the workflow.

use serde::{Deserialize, Serialize};

/// Types of dependencies between jobs
///
/// Different dependency types represent different mechanisms for job orchestration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DependencyType {
    /// Dependency via input condition
    InCondition,
    /// Dependency via output condition
    OutCondition,
    /// Dependency via control resource (mutex)
    ControlResource,
    /// Dependency via quantitative resource (semaphore)
    QuantitativeResource,
}

/// Represents a dependency relationship between two jobs
///
/// A dependency defines that one job (from_job) must complete or satisfy
/// certain conditions before another job (to_job) can execute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    /// Name of the job that must complete first (predecessor)
    pub from_job: String,
    /// Name of the job that depends on the predecessor (successor)
    pub to_job: String,
    /// Type of dependency mechanism
    pub dependency_type: DependencyType,
    /// Name of the condition (if dependency is condition-based)
    pub condition_name: Option<String>,
    /// Name of the resource (if dependency is resource-based)
    pub resource_name: Option<String>,
}

impl Dependency {
    /// Creates a new dependency between two jobs
    ///
    /// # Arguments
    ///
    /// * `from_job` - Name of the predecessor job
    /// * `to_job` - Name of the successor job
    /// * `dependency_type` - Type of dependency mechanism
    ///
    /// # Returns
    ///
    /// A new Dependency instance
    pub fn new(
        from_job: String,
        to_job: String,
        dependency_type: DependencyType,
    ) -> Self {
        Self {
            from_job,
            to_job,
            dependency_type,
            condition_name: None,
            resource_name: None,
        }
    }

    /// Adds a condition name to this dependency
    ///
    /// # Arguments
    ///
    /// * `condition_name` - Name of the condition
    ///
    /// # Returns
    ///
    /// Self with the condition name set
    pub fn with_condition(mut self, condition_name: String) -> Self {
        self.condition_name = Some(condition_name);
        self
    }

    /// Adds a resource name to this dependency
    ///
    /// # Arguments
    ///
    /// * `resource_name` - Name of the resource
    ///
    /// # Returns
    ///
    /// Self with the resource name set
    pub fn with_resource(mut self, resource_name: String) -> Self {
        self.resource_name = Some(resource_name);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_dependency() {
        let dep = Dependency::new(
            "JOB_A".to_string(),
            "JOB_B".to_string(),
            DependencyType::InCondition,
        );
        assert_eq!(dep.from_job, "JOB_A");
        assert_eq!(dep.to_job, "JOB_B");
    }

    #[test]
    fn test_dependency_with_condition() {
        let dep = Dependency::new(
            "JOB_A".to_string(),
            "JOB_B".to_string(),
            DependencyType::InCondition,
        )
        .with_condition("COND1".to_string());
        
        assert_eq!(dep.condition_name, Some("COND1".to_string()));
    }
}

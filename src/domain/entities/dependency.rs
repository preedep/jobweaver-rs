use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DependencyType {
    InCondition,
    OutCondition,
    ControlResource,
    QuantitativeResource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub from_job: String,
    pub to_job: String,
    pub dependency_type: DependencyType,
    pub condition_name: Option<String>,
    pub resource_name: Option<String>,
}

impl Dependency {
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

    pub fn with_condition(mut self, condition_name: String) -> Self {
        self.condition_name = Some(condition_name);
        self
    }

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

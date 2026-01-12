use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Job {
    pub job_name: String,
    pub folder_name: String,
    pub application: Option<String>,
    pub sub_application: Option<String>,
    pub description: Option<String>,
    pub owner: Option<String>,
    pub run_as: Option<String>,
    pub priority: Option<String>,
    pub critical: bool,
    pub task_type: Option<String>,
    pub cyclic: bool,
    pub node_id: Option<String>,
    pub cmdline: Option<String>,
    
    pub scheduling: super::SchedulingInfo,
    
    pub in_conditions: Vec<super::Condition>,
    pub out_conditions: Vec<super::Condition>,
    pub on_conditions: Vec<super::OnCondition>,
    
    pub control_resources: Vec<super::ControlResource>,
    pub quantitative_resources: Vec<super::QuantitativeResource>,
    
    pub variables: HashMap<String, String>,
    pub auto_edits: HashMap<String, String>,
    
    pub created_by: Option<String>,
    pub creation_date: Option<String>,
    pub change_userid: Option<String>,
    pub change_date: Option<String>,
    
    pub metadata: HashMap<String, String>,
}

impl Job {
    pub fn new(job_name: String, folder_name: String) -> Self {
        Self {
            job_name,
            folder_name,
            application: None,
            sub_application: None,
            description: None,
            owner: None,
            run_as: None,
            priority: None,
            critical: false,
            task_type: None,
            cyclic: false,
            node_id: None,
            cmdline: None,
            scheduling: super::SchedulingInfo::default(),
            in_conditions: Vec::new(),
            out_conditions: Vec::new(),
            on_conditions: Vec::new(),
            control_resources: Vec::new(),
            quantitative_resources: Vec::new(),
            variables: HashMap::new(),
            auto_edits: HashMap::new(),
            created_by: None,
            creation_date: None,
            change_userid: None,
            change_date: None,
            metadata: HashMap::new(),
        }
    }

    pub fn is_critical(&self) -> bool {
        self.critical
    }

    pub fn has_dependencies(&self) -> bool {
        !self.in_conditions.is_empty() || !self.control_resources.is_empty()
    }

    pub fn dependency_count(&self) -> usize {
        self.in_conditions.len() + self.control_resources.len()
    }

    pub fn has_complex_scheduling(&self) -> bool {
        self.scheduling.has_calendar() || self.cyclic
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_job() {
        let job = Job::new("TEST_JOB".to_string(), "TEST_FOLDER".to_string());
        assert_eq!(job.job_name, "TEST_JOB");
        assert_eq!(job.folder_name, "TEST_FOLDER");
        assert!(!job.critical);
        assert!(!job.cyclic);
    }

    #[test]
    fn test_dependency_count() {
        let mut job = Job::new("TEST_JOB".to_string(), "TEST_FOLDER".to_string());
        assert_eq!(job.dependency_count(), 0);
        
        job.in_conditions.push(super::super::Condition {
            name: "COND1".to_string(),
            condition_type: super::super::ConditionType::In,
            odate: None,
            and_or: None,
        });
        
        assert_eq!(job.dependency_count(), 1);
    }
}

//! Job entity module
//!
//! This module defines the core Job entity which represents a Control-M job
//! with all its properties, dependencies, and scheduling information.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a Control-M job with all its configuration and metadata
///
/// A Job is the fundamental unit of work in Control-M, containing scheduling information,
/// dependencies (conditions and resources), variables, and execution parameters.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Job {
    /// Unique job name identifier
    pub job_name: String,
    /// Name of the folder containing this job
    pub folder_name: String,
    /// Application name this job belongs to
    pub application: Option<String>,
    /// Sub-application classification
    pub sub_application: Option<String>,
    /// Application type (e.g., Command, Database, File Transfer)
    pub appl_type: Option<String>,
    /// Application version
    pub appl_ver: Option<String>,
    /// Human-readable description of the job's purpose
    pub description: Option<String>,
    /// Owner of the job
    pub owner: Option<String>,
    /// User account under which the job runs
    pub run_as: Option<String>,
    /// Job execution priority
    pub priority: Option<String>,
    /// Flag indicating if this is a critical job
    pub critical: bool,
    /// Type of task (e.g., Command, Script, Database)
    pub task_type: Option<String>,
    /// Flag indicating if this job runs cyclically
    pub cyclic: bool,
    /// Node/agent ID where the job executes
    pub node_id: Option<String>,
    /// Command line to execute
    pub cmdline: Option<String>,
    
    /// Scheduling configuration (time windows, calendars, etc.)
    pub scheduling: super::SchedulingInfo,
    
    /// Input conditions that must be satisfied before job runs
    pub in_conditions: Vec<super::Condition>,
    /// Output conditions that this job sets upon completion
    pub out_conditions: Vec<super::Condition>,
    /// Event-based conditions with actions (e.g., on completion, on error)
    pub on_conditions: Vec<super::OnCondition>,
    
    /// Control resources (mutexes) required by this job
    pub control_resources: Vec<super::ControlResource>,
    /// Quantitative resources (semaphores) required by this job
    pub quantitative_resources: Vec<super::QuantitativeResource>,
    
    /// Job variables (key-value pairs)
    pub variables: HashMap<String, String>,
    /// Auto-edit variables that are automatically set
    pub auto_edits: HashMap<String, String>,
    
    /// User who created this job
    pub created_by: Option<String>,
    /// Date when this job was created
    pub creation_date: Option<String>,
    /// User who last modified this job
    pub change_userid: Option<String>,
    /// Date of last modification
    pub change_date: Option<String>,
    
    /// Additional metadata as key-value pairs
    pub metadata: HashMap<String, String>,
}

impl Job {
    /// Creates a new Job with minimal required fields
    ///
    /// # Arguments
    ///
    /// * `job_name` - Unique identifier for the job
    /// * `folder_name` - Name of the folder containing this job
    ///
    /// # Returns
    ///
    /// A new Job instance with default values for optional fields
    pub fn new(job_name: String, folder_name: String) -> Self {
        Self {
            job_name,
            folder_name,
            application: None,
            sub_application: None,
            appl_type: None,
            appl_ver: None,
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

    /// Checks if this job is marked as critical
    ///
    /// # Returns
    ///
    /// `true` if the job is critical, `false` otherwise
    pub fn is_critical(&self) -> bool {
        self.critical
    }

    /// Checks if this job has any dependencies
    ///
    /// Dependencies include input conditions and control resources
    ///
    /// # Returns
    ///
    /// `true` if the job has at least one dependency, `false` otherwise
    pub fn has_dependencies(&self) -> bool {
        !self.in_conditions.is_empty() || !self.control_resources.is_empty()
    }

    /// Counts the total number of dependencies
    ///
    /// # Returns
    ///
    /// The sum of input conditions and control resources
    pub fn dependency_count(&self) -> usize {
        self.in_conditions.len() + self.control_resources.len()
    }

    /// Checks if this job has complex scheduling requirements
    ///
    /// Complex scheduling includes calendar-based scheduling or cyclic execution
    ///
    /// # Returns
    ///
    /// `true` if the job uses calendars or is cyclic, `false` otherwise
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

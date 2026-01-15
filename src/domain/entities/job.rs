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
    
    // Additional job metadata from Control-M XML
    /// Job ISN (Internal Sequence Number)
    pub jobisn: Option<i32>,
    /// Job group
    pub group: Option<String>,
    /// Member name
    pub memname: Option<String>,
    /// Author (different from created_by)
    pub author: Option<String>,
    /// Documentation library
    pub doclib: Option<String>,
    /// Documentation member
    pub docmem: Option<String>,
    /// Cyclic interval
    pub interval: Option<String>,
    /// Override path
    pub override_path: Option<String>,
    /// Override library
    pub overlib: Option<String>,
    /// Member library
    pub memlib: Option<String>,
    /// Confirmation required
    pub confirm: Option<String>,
    /// Retroactive scheduling
    pub retro: Option<String>,
    /// Maximum wait time
    pub maxwait: Option<i32>,
    /// Maximum reruns
    pub maxrerun: Option<i32>,
    /// Auto archive
    pub autoarch: Option<String>,
    /// Maximum days
    pub maxdays: Option<i32>,
    /// Maximum runs
    pub maxruns: Option<i32>,
    /// Days specification
    pub days: Option<String>,
    /// Weekdays specification
    pub weekdays: Option<String>,
    /// Monthly scheduling flags
    pub jan: Option<String>,
    pub feb: Option<String>,
    pub mar: Option<String>,
    pub apr: Option<String>,
    pub may: Option<String>,
    pub jun: Option<String>,
    pub jul: Option<String>,
    pub aug: Option<String>,
    pub sep: Option<String>,
    pub oct: Option<String>,
    pub nov: Option<String>,
    pub dec: Option<String>,
    /// Date specification
    pub date: Option<String>,
    /// Rerun member
    pub rerunmem: Option<String>,
    /// Days AND/OR logic
    pub days_and_or: Option<String>,
    /// Category
    pub category: Option<String>,
    /// Shift
    pub shift: Option<String>,
    /// Shift number
    pub shiftnum: Option<String>,
    /// PDS name
    pub pdsname: Option<String>,
    /// Minimum value
    pub minimum: Option<String>,
    /// Prevent NCT2
    pub preventnct2: Option<String>,
    /// Option
    pub option: Option<String>,
    /// From value
    pub from: Option<String>,
    /// Parameter
    pub par: Option<String>,
    /// System database
    pub sysdb: Option<String>,
    /// Due out time
    pub due_out: Option<String>,
    /// Retention days
    pub reten_days: Option<String>,
    /// Retention generation
    pub reten_gen: Option<String>,
    /// Task class
    pub task_class: Option<String>,
    /// Previous day
    pub prev_day: Option<String>,
    /// Adjust condition
    pub adjust_cond: Option<String>,
    /// Jobs in group
    pub jobs_in_group: Option<String>,
    /// Large size flag
    pub large_size: Option<String>,
    /// Independent cyclic
    pub ind_cyclic: Option<String>,
    /// Creation user (different from created_by)
    pub creation_user: Option<String>,
    /// Creation time
    pub creation_time: Option<String>,
    /// Change time
    pub change_time: Option<String>,
    /// Job version
    pub job_version: Option<String>,
    /// Rule-based calendar relationship
    pub rule_based_calendar_relationship: Option<String>,
    /// Tag relationship
    pub tag_relationship: Option<String>,
    /// Timezone
    pub timezone: Option<String>,
    /// Application form
    pub appl_form: Option<String>,
    /// Control-M version
    pub cm_ver: Option<String>,
    /// Multi-agent
    pub multy_agent: Option<String>,
    /// Active from date
    pub active_from: Option<String>,
    /// Active till date
    pub active_till: Option<String>,
    /// Scheduling environment
    pub scheduling_environment: Option<String>,
    /// System affinity
    pub system_affinity: Option<String>,
    /// Request NJE node
    pub request_nje_node: Option<String>,
    /// Statistical calendar
    pub stat_cal: Option<String>,
    /// Instream JCL
    pub instream_jcl: Option<String>,
    /// Use instream JCL flag
    pub use_instream_jcl: Option<String>,
    /// Due out days offset
    pub due_out_daysoffset: Option<String>,
    /// From days offset
    pub from_daysoffset: Option<String>,
    /// To days offset
    pub to_daysoffset: Option<String>,
    /// Version opcode
    pub version_opcode: Option<String>,
    /// Is current version flag
    pub is_current_version: Option<String>,
    /// Version serial
    pub version_serial: Option<i32>,
    /// Version host
    pub version_host: Option<String>,
    /// Cyclic interval sequence
    pub cyclic_interval_sequence: Option<String>,
    /// Cyclic times sequence
    pub cyclic_times_sequence: Option<String>,
    /// Cyclic tolerance
    pub cyclic_tolerance: Option<i32>,
    /// Cyclic type
    pub cyclic_type: Option<String>,
    /// Parent folder name
    pub parent_folder: Option<String>,
    /// Parent table name
    pub parent_table: Option<String>,
    /// End folder
    pub end_folder: Option<String>,
    /// Order date
    pub odate: Option<String>,
    /// From procedures
    pub fprocs: Option<String>,
    /// To programs
    pub tpgms: Option<String>,
    /// To procedures
    pub tprocs: Option<String>,
    
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
            jobisn: None,
            group: None,
            memname: None,
            author: None,
            doclib: None,
            docmem: None,
            interval: None,
            override_path: None,
            overlib: None,
            memlib: None,
            confirm: None,
            retro: None,
            maxwait: None,
            maxrerun: None,
            autoarch: None,
            maxdays: None,
            maxruns: None,
            days: None,
            weekdays: None,
            jan: None,
            feb: None,
            mar: None,
            apr: None,
            may: None,
            jun: None,
            jul: None,
            aug: None,
            sep: None,
            oct: None,
            nov: None,
            dec: None,
            date: None,
            rerunmem: None,
            days_and_or: None,
            category: None,
            shift: None,
            shiftnum: None,
            pdsname: None,
            minimum: None,
            preventnct2: None,
            option: None,
            from: None,
            par: None,
            sysdb: None,
            due_out: None,
            reten_days: None,
            reten_gen: None,
            task_class: None,
            prev_day: None,
            adjust_cond: None,
            jobs_in_group: None,
            large_size: None,
            ind_cyclic: None,
            creation_user: None,
            creation_time: None,
            change_time: None,
            job_version: None,
            rule_based_calendar_relationship: None,
            tag_relationship: None,
            timezone: None,
            appl_form: None,
            cm_ver: None,
            multy_agent: None,
            active_from: None,
            active_till: None,
            scheduling_environment: None,
            system_affinity: None,
            request_nje_node: None,
            stat_cal: None,
            instream_jcl: None,
            use_instream_jcl: None,
            due_out_daysoffset: None,
            from_daysoffset: None,
            to_daysoffset: None,
            version_opcode: None,
            is_current_version: None,
            version_serial: None,
            version_host: None,
            cyclic_interval_sequence: None,
            cyclic_times_sequence: None,
            cyclic_tolerance: None,
            cyclic_type: None,
            parent_folder: None,
            parent_table: None,
            end_folder: None,
            odate: None,
            fprocs: None,
            tpgms: None,
            tprocs: None,
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

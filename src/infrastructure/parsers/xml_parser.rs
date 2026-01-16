//! Control-M XML Parser module
//!
//! This module provides functionality to parse Control-M XML export files
//! and convert them into domain entities (Folders, Jobs, Conditions, etc.).
//! Handles various encoding issues and XML sanitization.

use anyhow::{Context, Result};
use roxmltree::Document;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use encoding_rs_io::DecodeReaderBytesBuilder;

use crate::domain::entities::*;
use crate::domain::entities::condition::DoAction;
use crate::domain::entities::folder::FolderType;

/// Parser for Control-M XML export files
///
/// Handles parsing of Control-M XML files with support for:
/// - Multiple folder types (Simple, Smart, Table, SmartTable)
/// - Job definitions with all attributes
/// - Dependencies (conditions, resources)
/// - Scheduling information
/// - Windows-1252 encoding
pub struct ControlMXmlParser;

impl ControlMXmlParser {
    /// Creates a new ControlMXmlParser instance
    ///
    /// # Returns
    ///
    /// A new ControlMXmlParser
    pub fn new() -> Self {
        Self
    }

    /// Parses a Control-M XML file from disk
    ///
    /// Reads the file with Windows-1252 encoding, sanitizes invalid characters,
    /// and parses the XML structure into domain entities.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the XML file
    ///
    /// # Returns
    ///
    /// Result containing a vector of Folder entities or an error
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - File cannot be opened
    /// - File cannot be read or decoded
    /// - XML is malformed
    pub fn parse_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<Folder>> {
        // Open file and set up Windows-1252 decoder
        let file = File::open(path)
            .context("Failed to open XML file")?;
        
        // Control-M exports often use Windows-1252 encoding
        let mut decoder = DecodeReaderBytesBuilder::new()
            .encoding(Some(encoding_rs::WINDOWS_1252))
            .build(file);
        
        let mut xml_content = String::new();
        decoder.read_to_string(&mut xml_content)
            .context("Failed to read XML file")?;
        
        // Sanitize XML by removing invalid control characters
        let sanitized = self.sanitize_xml(&xml_content);
        
        self.parse_xml(&sanitized)
    }

    /// Sanitizes XML content by removing invalid control characters
    ///
    /// Control-M XML exports may contain invalid control characters that
    /// cause XML parsing to fail. This method filters them out while
    /// preserving valid whitespace.
    ///
    /// According to XML 1.0 spec, valid characters are:
    /// - #x9 | #xA | #xD | [#x20-#xD7FF] | [#xE000-#xFFFD] | [#x10000-#x10FFFF]
    ///
    /// # Arguments
    ///
    /// * `xml` - Raw XML string to sanitize
    ///
    /// # Returns
    ///
    /// Sanitized XML string with only valid characters
    fn sanitize_xml(&self, xml: &str) -> String {
        xml.chars()
            .filter(|&c| {
                let code = c as u32;
                // Valid XML 1.0 characters
                code == 0x09      // Tab
                || code == 0x0A   // Line Feed
                || code == 0x0D   // Carriage Return
                || (code >= 0x20 && code <= 0xD7FF)
                || (code >= 0xE000 && code <= 0xFFFD)
                || (code >= 0x10000 && code <= 0x10FFFF)
            })
            .collect()
    }

    /// Parses XML content into domain entities
    ///
    /// Processes the XML document and extracts all folder definitions
    /// including FOLDER, SMART_FOLDER, TABLE, and SMART_TABLE types.
    ///
    /// # Arguments
    ///
    /// * `xml_content` - XML string to parse
    ///
    /// # Returns
    ///
    /// Result containing a vector of Folder entities or an error
    pub fn parse_xml(&self, xml_content: &str) -> Result<Vec<Folder>> {
        let doc = Document::parse(xml_content)
            .context("Failed to parse XML")?;

        let mut folders = Vec::new();
        
        let root = doc.root_element();
        
        // Iterate through root-level elements to find folders
        for node in root.children() {
            if !node.is_element() {
                continue;
            }
            
            let tag_name = node.tag_name().name();
            
            match tag_name {
                "FOLDER" => {
                    if let Ok(folder) = self.parse_folder_node(&node, FolderType::Simple) {
                        folders.push(folder);
                    }
                }
                "SMART_FOLDER" => {
                    if let Ok(folder) = self.parse_folder_node(&node, FolderType::Smart) {
                        folders.push(folder);
                    }
                }
                "TABLE" => {
                    if let Ok(folder) = self.parse_folder_node(&node, FolderType::Table) {
                        folders.push(folder);
                    }
                }
                "SMART_TABLE" => {
                    if let Ok(folder) = self.parse_folder_node(&node, FolderType::SmartTable) {
                        folders.push(folder);
                    }
                }
                _ => {}
            }
        }

        Ok(folders)
    }
    
    /// Parses a folder node from XML
    ///
    /// Extracts folder attributes and recursively parses all jobs within the folder.
    ///
    /// # Arguments
    ///
    /// * `node` - XML node representing the folder
    /// * `folder_type` - Type of folder (Simple, Smart, Table, SmartTable)
    ///
    /// # Returns
    ///
    /// Result containing a Folder entity or an error
    fn parse_folder_node(&self, node: &roxmltree::Node, folder_type: FolderType) -> Result<Folder> {
        // Folder name can be in FOLDER_NAME or TABLE_NAME attribute
        let folder_name = node.attribute("FOLDER_NAME")
            .or_else(|| node.attribute("TABLE_NAME"))
            .unwrap_or("UNKNOWN")
            .to_string();
        
        let mut folder = Folder::new(folder_name, folder_type);
        
        // Core attributes
        folder.datacenter = node.attribute("DATACENTER").map(|s| s.to_string());
        folder.application = node.attribute("APPLICATION").map(|s| s.to_string());
        folder.description = node.attribute("DESCRIPTION").map(|s| s.to_string());
        folder.owner = node.attribute("OWNER").map(|s| s.to_string());
        
        // Additional folder metadata
        folder.version = node.attribute("VERSION").map(|s| s.to_string());
        folder.platform = node.attribute("PLATFORM").map(|s| s.to_string());
        folder.table_name = node.attribute("TABLE_NAME").map(|s| s.to_string());
        folder.folder_dsn = node.attribute("FOLDER_DSN").map(|s| s.to_string());
        folder.table_dsn = node.attribute("TABLE_DSN").map(|s| s.to_string());
        folder.modified = node.attribute("MODIFIED").and_then(|s| match s {
            "1" | "true" | "True" => Some(true),
            "0" | "false" | "False" => Some(false),
            _ => None,
        });
        folder.last_upload = node.attribute("LAST_UPLOAD").map(|s| s.to_string());
        folder.folder_order_method = node.attribute("FOLDER_ORDER_METHOD").map(|s| s.to_string());
        folder.table_userdaily = node.attribute("TABLE_USERDAILY").map(|s| s.to_string());
        folder.real_folder_id = node.attribute("REAL_FOLDER_ID").and_then(|s| s.parse().ok());
        folder.real_tableid = node.attribute("REAL_TABLEID").and_then(|s| s.parse().ok());
        folder.type_code = node.attribute("TYPE").and_then(|s| s.parse().ok());
        folder.used_by = node.attribute("USED_BY").map(|s| s.to_string());
        folder.used_by_code = node.attribute("USED_BY_CODE").and_then(|s| s.parse().ok());
        folder.enforce_validation = node.attribute("ENFORCE_VALIDATION").map(|s| s.to_string());
        folder.site_standard_name = node.attribute("SITE_STANDARD_NAME").map(|s| s.to_string());
        
        // Parse all jobs within this folder
        for child in node.children() {
            if !child.is_element() {
                continue;
            }
            
            if child.tag_name().name() == "JOB" {
                if let Ok(job) = self.parse_job_node(&child, folder.folder_name.clone()) {
                    folder.add_job(job);
                }
            }
        }
        
        Ok(folder)
    }
    
    /// Parses a job node from XML
    ///
    /// Extracts all job attributes, scheduling information, and child elements
    /// (conditions, resources, variables, etc.).
    ///
    /// # Arguments
    ///
    /// * `node` - XML node representing the job
    /// * `folder_name` - Name of the parent folder
    ///
    /// # Returns
    ///
    /// Result containing a Job entity or an error
    fn parse_job_node(&self, node: &roxmltree::Node, folder_name: String) -> Result<Job> {
        let job_name = node.attribute("JOBNAME").unwrap_or("UNKNOWN").to_string();
        let mut job = Job::new(job_name, folder_name);
        
        self.parse_basic_attributes(node, &mut job);
        self.parse_scheduling_attributes(node, &mut job);
        self.parse_child_elements(node, &mut job);
        
        Ok(job)
    }
    
    /// Parses basic job attributes from XML node
    ///
    /// Extracts attributes like application, owner, priority, critical flag, etc.
    ///
    /// # Arguments
    ///
    /// * `node` - XML node containing job attributes
    /// * `job` - Mutable reference to Job to populate
    fn parse_basic_attributes(&self, node: &roxmltree::Node, job: &mut Job) {
        // Core attributes
        job.application = node.attribute("APPLICATION").map(|s| s.to_string());
        job.sub_application = node.attribute("SUB_APPLICATION").map(|s| s.to_string());
        job.appl_type = node.attribute("APPL_TYPE").map(|s| s.to_string());
        job.appl_ver = node.attribute("APPL_VER").map(|s| s.to_string());
        job.description = node.attribute("DESCRIPTION").map(|s| s.to_string());
        job.owner = node.attribute("OWNER").map(|s| s.to_string());
        job.run_as = node.attribute("RUN_AS").map(|s| s.to_string());
        job.priority = node.attribute("PRIORITY").map(|s| s.to_string());
        job.critical = node.attribute("CRITICAL") == Some("Y");
        job.task_type = node.attribute("TASKTYPE").map(|s| s.to_string());
        job.cyclic = node.attribute("CYCLIC") == Some("Y");
        job.node_id = node.attribute("NODEID").map(|s| s.to_string());
        job.cmdline = node.attribute("CMDLINE").map(|s| s.to_string());
        
        // Additional job metadata
        job.jobisn = self.get_int_attr(node, "JOBISN");
        job.group = node.attribute("GROUP").map(|s| s.to_string());
        job.memname = node.attribute("MEMNAME").map(|s| s.to_string());
        job.author = node.attribute("AUTHOR").map(|s| s.to_string());
        job.doclib = node.attribute("DOCLIB").map(|s| s.to_string());
        job.docmem = node.attribute("DOCMEM").map(|s| s.to_string());
        job.interval = node.attribute("INTERVAL").map(|s| s.to_string());
        job.override_path = node.attribute("OVERRIDE_PATH").map(|s| s.to_string());
        job.overlib = node.attribute("OVERLIB").map(|s| s.to_string());
        job.memlib = node.attribute("MEMLIB").map(|s| s.to_string());
        job.confirm = node.attribute("CONFIRM").map(|s| s.to_string());
        job.retro = node.attribute("RETRO").map(|s| s.to_string());
        job.maxwait = self.get_int_attr(node, "MAXWAIT");
        job.maxrerun = self.get_int_attr(node, "MAXRERUN");
        job.autoarch = node.attribute("AUTOARCH").map(|s| s.to_string());
        job.maxdays = self.get_int_attr(node, "MAXDAYS");
        job.maxruns = self.get_int_attr(node, "MAXRUNS");
        
        // Scheduling attributes
        job.days = node.attribute("DAYS").map(|s| s.to_string());
        job.weekdays = node.attribute("WEEKDAYS").map(|s| s.to_string());
        job.jan = node.attribute("JAN").map(|s| s.to_string());
        job.feb = node.attribute("FEB").map(|s| s.to_string());
        job.mar = node.attribute("MAR").map(|s| s.to_string());
        job.apr = node.attribute("APR").map(|s| s.to_string());
        job.may = node.attribute("MAY").map(|s| s.to_string());
        job.jun = node.attribute("JUN").map(|s| s.to_string());
        job.jul = node.attribute("JUL").map(|s| s.to_string());
        job.aug = node.attribute("AUG").map(|s| s.to_string());
        job.sep = node.attribute("SEP").map(|s| s.to_string());
        job.oct = node.attribute("OCT").map(|s| s.to_string());
        job.nov = node.attribute("NOV").map(|s| s.to_string());
        job.dec = node.attribute("DEC").map(|s| s.to_string());
        job.date = node.attribute("DATE").map(|s| s.to_string());
        job.rerunmem = node.attribute("RERUNMEM").map(|s| s.to_string());
        job.days_and_or = node.attribute("DAYS_AND_OR").map(|s| s.to_string());
        job.category = node.attribute("CATEGORY").map(|s| s.to_string());
        job.shift = node.attribute("SHIFT").map(|s| s.to_string());
        job.shiftnum = node.attribute("SHIFTNUM").map(|s| s.to_string());
        job.pdsname = node.attribute("PDSNAME").map(|s| s.to_string());
        job.minimum = node.attribute("MINIMUM").map(|s| s.to_string());
        job.preventnct2 = node.attribute("PREVENTNCT2").map(|s| s.to_string());
        job.option = node.attribute("OPTION").map(|s| s.to_string());
        job.from = node.attribute("FROM").map(|s| s.to_string());
        job.par = node.attribute("PAR").map(|s| s.to_string());
        job.sysdb = node.attribute("SYSDB").map(|s| s.to_string());
        job.due_out = node.attribute("DUE_OUT").map(|s| s.to_string());
        job.reten_days = node.attribute("RETEN_DAYS").map(|s| s.to_string());
        job.reten_gen = node.attribute("RETEN_GEN").map(|s| s.to_string());
        job.task_class = node.attribute("TASK_CLASS").map(|s| s.to_string());
        job.prev_day = node.attribute("PREV_DAY").map(|s| s.to_string());
        job.adjust_cond = node.attribute("ADJUST_COND").map(|s| s.to_string());
        job.jobs_in_group = node.attribute("JOBS_IN_GROUP").map(|s| s.to_string());
        job.large_size = node.attribute("LARGE_SIZE").map(|s| s.to_string());
        job.ind_cyclic = node.attribute("IND_CYCLIC").map(|s| s.to_string());
        
        // Audit fields
        job.creation_user = node.attribute("CREATION_USER").map(|s| s.to_string());
        job.creation_time = node.attribute("CREATION_TIME").map(|s| s.to_string());
        job.created_by = node.attribute("CREATED_BY").map(|s| s.to_string());
        job.creation_date = node.attribute("CREATION_DATE").map(|s| s.to_string());
        job.change_userid = node.attribute("CHANGE_USERID").map(|s| s.to_string());
        job.change_date = node.attribute("CHANGE_DATE").map(|s| s.to_string());
        job.change_time = node.attribute("CHANGE_TIME").map(|s| s.to_string());
        
        // Version control
        job.job_version = node.attribute("JOB_VERSION").map(|s| s.to_string());
        job.version_opcode = node.attribute("VERSION_OPCODE").map(|s| s.to_string());
        job.is_current_version = node.attribute("IS_CURRENT_VERSION").map(|s| s.to_string());
        job.version_serial = self.get_int_attr(node, "VERSION_SERIAL");
        job.version_host = node.attribute("VERSION_HOST").map(|s| s.to_string());
        
        // Advanced features
        job.rule_based_calendar_relationship = node.attribute("RULE_BASED_CALENDAR_RELATIONSHIP").map(|s| s.to_string());
        job.tag_relationship = node.attribute("TAG_RELATIONSHIP").map(|s| s.to_string());
        job.timezone = node.attribute("TIMEZONE").map(|s| s.to_string());
        job.appl_form = node.attribute("APPL_FORM").map(|s| s.to_string());
        job.cm_ver = node.attribute("CM_VER").map(|s| s.to_string());
        job.multy_agent = node.attribute("MULTY_AGENT").map(|s| s.to_string());
        job.active_from = node.attribute("ACTIVE_FROM").map(|s| s.to_string());
        job.active_till = node.attribute("ACTIVE_TILL").map(|s| s.to_string());
        job.scheduling_environment = node.attribute("SCHEDULING_ENVIRONMENT").map(|s| s.to_string());
        job.system_affinity = node.attribute("SYSTEM_AFFINITY").map(|s| s.to_string());
        job.request_nje_node = node.attribute("REQUEST_NJE_NODE").map(|s| s.to_string());
        job.stat_cal = node.attribute("STAT_CAL").map(|s| s.to_string());
        job.instream_jcl = node.attribute("INSTREAM_JCL").map(|s| s.to_string());
        job.use_instream_jcl = node.attribute("USE_INSTREAM_JCL").map(|s| s.to_string());
        job.due_out_daysoffset = node.attribute("DUE_OUT_DAYSOFFSET").map(|s| s.to_string());
        job.from_daysoffset = node.attribute("FROM_DAYSOFFSET").map(|s| s.to_string());
        job.to_daysoffset = node.attribute("TO_DAYSOFFSET").map(|s| s.to_string());
        
        // Cyclic attributes
        job.cyclic_interval_sequence = node.attribute("CYCLIC_INTERVAL_SEQUENCE").map(|s| s.to_string());
        job.cyclic_times_sequence = node.attribute("CYCLIC_TIMES_SEQUENCE").map(|s| s.to_string());
        job.cyclic_tolerance = self.get_int_attr(node, "CYCLIC_TOLERANCE");
        job.cyclic_type = node.attribute("CYCLIC_TYPE").map(|s| s.to_string());
        
        // Hierarchy
        job.parent_folder = node.attribute("PARENT_FOLDER").map(|s| s.to_string());
        job.parent_table = node.attribute("PARENT_TABLE").map(|s| s.to_string());
        job.end_folder = node.attribute("END_FOLDER").map(|s| s.to_string());
        job.odate = node.attribute("ODATE").map(|s| s.to_string());
        job.fprocs = node.attribute("FPROCS").map(|s| s.to_string());
        job.tpgms = node.attribute("TPGMS").map(|s| s.to_string());
        job.tprocs = node.attribute("TPROCS").map(|s| s.to_string());
    }
    
    /// Parses scheduling-related attributes from XML node
    ///
    /// Extracts time windows and calendar information.
    ///
    /// # Arguments
    ///
    /// * `node` - XML node containing scheduling attributes
    /// * `job` - Mutable reference to Job to populate
    fn parse_scheduling_attributes(&self, node: &roxmltree::Node, job: &mut Job) {
        job.scheduling.time_from = node.attribute("TIMEFROM").map(|s| s.to_string());
        job.scheduling.time_to = node.attribute("TIMETO").map(|s| s.to_string());
        job.scheduling.days = node.attribute("DAYS").map(|s| s.to_string());
        job.scheduling.weekdays = node.attribute("WEEKDAYS").map(|s| s.to_string());
        job.scheduling.days_calendar = node.attribute("DAYSCAL").map(|s| s.to_string());
        job.scheduling.weeks_calendar = node.attribute("WEEKSCAL").map(|s| s.to_string());
        job.scheduling.conf_calendar = node.attribute("CONFCAL").map(|s| s.to_string());
        job.scheduling.cyclic_interval = node.attribute("INTERVAL").map(|s| s.to_string());
        job.scheduling.max_wait = self.get_int_attr(node, "MAXWAIT");
        job.scheduling.max_rerun = self.get_int_attr(node, "MAXRERUN");
        
        // Additional scheduling attributes
        job.scheduling.shift = node.attribute("SHIFT").map(|s| s.to_string());
        job.scheduling.shift_num = node.attribute("SHIFTNUM").map(|s| s.to_string());
        job.scheduling.retro = node.attribute("RETRO").map(|s| s.to_string());
        job.scheduling.stat_cal = node.attribute("STAT_CAL").map(|s| s.to_string());
        job.scheduling.date = node.attribute("DATE").map(|s| s.to_string());
        job.scheduling.days_and_or = node.attribute("DAYS_AND_OR").map(|s| s.to_string());
        job.scheduling.maxdays = self.get_int_attr(node, "MAXDAYS");
        job.scheduling.maxruns = self.get_int_attr(node, "MAXRUNS");
        job.scheduling.autoarch = node.attribute("AUTOARCH").map(|s| s.to_string());
        job.scheduling.confirm = node.attribute("CONFIRM").map(|s| s.to_string());
        job.scheduling.timezone = node.attribute("TIMEZONE").map(|s| s.to_string());
        job.scheduling.active_from = node.attribute("ACTIVE_FROM").map(|s| s.to_string());
        job.scheduling.active_till = node.attribute("ACTIVE_TILL").map(|s| s.to_string());
        job.scheduling.due_out = node.attribute("DUE_OUT").map(|s| s.to_string());
        job.scheduling.due_out_daysoffset = node.attribute("DUE_OUT_DAYSOFFSET").map(|s| s.to_string());
        job.scheduling.from_daysoffset = node.attribute("FROM_DAYSOFFSET").map(|s| s.to_string());
        job.scheduling.to_daysoffset = node.attribute("TO_DAYSOFFSET").map(|s| s.to_string());
        job.scheduling.prev_day = node.attribute("PREV_DAY").map(|s| s.to_string());
        job.scheduling.adjust_cond = node.attribute("ADJUST_COND").map(|s| s.to_string());
    }
    
    /// Parses child elements of a job node
    ///
    /// Processes INCOND, OUTCOND, VARIABLE, CONTROL, QUANTITATIVE, and ON elements.
    ///
    /// # Arguments
    ///
    /// * `node` - XML node containing child elements
    /// * `job` - Mutable reference to Job to populate
    fn parse_child_elements(&self, node: &roxmltree::Node, job: &mut Job) {
        for child in node.children().filter(|n| n.is_element()) {
            match child.tag_name().name() {
                "INCOND" => self.parse_in_condition(&child, job),
                "OUTCOND" => self.parse_out_condition(&child, job),
                "VARIABLE" => self.parse_variable(&child, job),
                "CONTROL" => self.parse_control_resource(&child, job),
                "QUANTITATIVE" => self.parse_quantitative_resource(&child, job),
                "ON" => self.parse_on_condition(&child, job),
                _ => {}
            }
        }
    }
    
    /// Parses an input condition (INCOND) element
    ///
    /// # Arguments
    ///
    /// * `node` - XML node representing the input condition
    /// * `job` - Mutable reference to Job to add condition to
    fn parse_in_condition(&self, node: &roxmltree::Node, job: &mut Job) {
        if let Some(name) = node.attribute("NAME") {
            let mut condition = Condition::new_in(name.to_string());
            condition.odate = node.attribute("ODATE").map(|s| s.to_string());
            condition.and_or = node.attribute("AND_OR").map(|s| s.to_string());
            job.in_conditions.push(condition);
        }
    }
    
    /// Parses an output condition (OUTCOND) element
    ///
    /// # Arguments
    ///
    /// * `node` - XML node representing the output condition
    /// * `job` - Mutable reference to Job to add condition to
    fn parse_out_condition(&self, node: &roxmltree::Node, job: &mut Job) {
        if let Some(name) = node.attribute("NAME") {
            let mut condition = Condition::new_out(name.to_string());
            condition.odate = node.attribute("ODATE").map(|s| s.to_string());
            // Note: SIGN attribute is stored in and_or field for out conditions
            condition.and_or = node.attribute("SIGN").map(|s| s.to_string());
            job.out_conditions.push(condition);
        }
    }
    
    /// Parses a variable (VARIABLE) element
    ///
    /// # Arguments
    ///
    /// * `node` - XML node representing the variable
    /// * `job` - Mutable reference to Job to add variable to
    fn parse_variable(&self, node: &roxmltree::Node, job: &mut Job) {
        if let (Some(name), Some(value)) = (node.attribute("NAME"), node.attribute("VALUE")) {
            job.variables.insert(name.to_string(), value.to_string());
        }
    }
    
    /// Parses a control resource (CONTROL) element
    ///
    /// Control resources act as mutexes for job synchronization.
    ///
    /// # Arguments
    ///
    /// * `node` - XML node representing the control resource
    /// * `job` - Mutable reference to Job to add resource to
    fn parse_control_resource(&self, node: &roxmltree::Node, job: &mut Job) {
        if let Some(name) = node.attribute("NAME") {
            job.control_resources.push(ControlResource::new(name.to_string()));
        }
    }
    
    /// Parses a quantitative resource (QUANTITATIVE) element
    ///
    /// Quantitative resources manage limited resource pools.
    ///
    /// # Arguments
    ///
    /// * `node` - XML node representing the quantitative resource
    /// * `job` - Mutable reference to Job to add resource to
    fn parse_quantitative_resource(&self, node: &roxmltree::Node, job: &mut Job) {
        if let Some(name) = node.attribute("NAME") {
            let quant = node.attribute("QUANT")
                .and_then(|q| q.parse::<i32>().ok())
                .unwrap_or(1);
            job.quantitative_resources.push(QuantitativeResource::new(name.to_string(), quant));
        }
    }
    
    /// Parses an ON condition element
    ///
    /// ON conditions define event-based actions that execute when
    /// specific conditions are met (e.g., job completion, failure).
    ///
    /// # Arguments
    ///
    /// * `node` - XML node representing the ON condition
    /// * `job` - Mutable reference to Job to add ON condition to
    fn parse_on_condition(&self, node: &roxmltree::Node, job: &mut Job) {
        let mut on_cond = OnCondition::new();
        on_cond.stmt = node.attribute("STMT").map(|s| s.to_string());
        on_cond.code = node.attribute("CODE").map(|s| s.to_string());
        
        for action_node in node.children().filter(|n| n.is_element()) {
            if action_node.tag_name().name() == "DOACTION" {
                if let Some(action) = action_node.attribute("ACTION") {
                    on_cond.actions.push(DoAction::Action(action.to_string()));
                }
            }
        }
        
        job.on_conditions.push(on_cond);
    }
    
    /// Helper method to parse integer attributes
    ///
    /// # Arguments
    ///
    /// * `node` - XML node containing the attribute
    /// * `attr_name` - Name of the attribute to parse
    ///
    /// # Returns
    ///
    /// Option containing the parsed integer or None if parsing fails
    fn get_int_attr(&self, node: &roxmltree::Node, attr_name: &str) -> Option<i32> {
        node.attribute(attr_name).and_then(|s| s.parse().ok())
    }
}

impl Default for ControlMXmlParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_xml() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<DEFTABLE>
    <FOLDER FOLDER_NAME="TEST_FOLDER" DATACENTER="DC1">
        <JOB JOBNAME="JOB1" APPLICATION="APP1" CRITICAL="Y">
            <INCOND NAME="COND1"/>
            <OUTCOND NAME="COND2"/>
        </JOB>
    </FOLDER>
</DEFTABLE>"#;

        let parser = ControlMXmlParser::new();
        let folders = parser.parse_xml(xml).unwrap();
        
        assert_eq!(folders.len(), 1);
        assert_eq!(folders[0].folder_name, "TEST_FOLDER");
        assert_eq!(folders[0].jobs.len(), 1);
        assert_eq!(folders[0].jobs[0].job_name, "JOB1");
        assert!(folders[0].jobs[0].critical);
    }
}

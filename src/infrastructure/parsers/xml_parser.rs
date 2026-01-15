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
                // Keep valid XML characters:
                // - Tab (0x09), LF (0x0A), CR (0x0D)
                // - Printable ASCII and Unicode (>= 0x20)
                c == '\t' || c == '\n' || c == '\r' || c >= ' '
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
        folder.datacenter = node.attribute("DATACENTER").map(|s| s.to_string());
        folder.application = node.attribute("APPLICATION").map(|s| s.to_string());
        
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
        job.scheduling.days_calendar = node.attribute("DAYSCAL").map(|s| s.to_string());
        job.scheduling.weeks_calendar = node.attribute("WEEKSCAL").map(|s| s.to_string());
        job.scheduling.conf_calendar = node.attribute("CONFCAL").map(|s| s.to_string());
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
            job.in_conditions.push(Condition::new_in(name.to_string()));
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
            job.out_conditions.push(Condition::new_out(name.to_string()));
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

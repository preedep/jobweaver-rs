use anyhow::{Context, Result};
use roxmltree::Document;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use encoding_rs_io::DecodeReaderBytesBuilder;

use crate::domain::entities::*;
use crate::domain::entities::condition::DoAction;
use crate::domain::entities::folder::FolderType;

pub struct ControlMXmlParser;

impl ControlMXmlParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<Folder>> {
        let file = File::open(path)
            .context("Failed to open XML file")?;
        
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

    pub fn parse_xml(&self, xml_content: &str) -> Result<Vec<Folder>> {
        let doc = Document::parse(xml_content)
            .context("Failed to parse XML")?;

        let mut folders = Vec::new();
        
        let root = doc.root_element();
        
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
    
    fn parse_folder_node(&self, node: &roxmltree::Node, folder_type: FolderType) -> Result<Folder> {
        let folder_name = node.attribute("FOLDER_NAME")
            .or_else(|| node.attribute("TABLE_NAME"))
            .unwrap_or("UNKNOWN")
            .to_string();
        
        let mut folder = Folder::new(folder_name, folder_type);
        folder.datacenter = node.attribute("DATACENTER").map(|s| s.to_string());
        folder.application = node.attribute("APPLICATION").map(|s| s.to_string());
        
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
    
    fn parse_job_node(&self, node: &roxmltree::Node, folder_name: String) -> Result<Job> {
        let job_name = node.attribute("JOBNAME").unwrap_or("UNKNOWN").to_string();
        let mut job = Job::new(job_name, folder_name);
        
        self.parse_basic_attributes(node, &mut job);
        self.parse_scheduling_attributes(node, &mut job);
        self.parse_child_elements(node, &mut job);
        
        Ok(job)
    }
    
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
    
    fn parse_scheduling_attributes(&self, node: &roxmltree::Node, job: &mut Job) {
        job.scheduling.time_from = node.attribute("TIMEFROM").map(|s| s.to_string());
        job.scheduling.time_to = node.attribute("TIMETO").map(|s| s.to_string());
        job.scheduling.days_calendar = node.attribute("DAYSCAL").map(|s| s.to_string());
        job.scheduling.weeks_calendar = node.attribute("WEEKSCAL").map(|s| s.to_string());
        job.scheduling.conf_calendar = node.attribute("CONFCAL").map(|s| s.to_string());
    }
    
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
    
    fn parse_in_condition(&self, node: &roxmltree::Node, job: &mut Job) {
        if let Some(name) = node.attribute("NAME") {
            job.in_conditions.push(Condition::new_in(name.to_string()));
        }
    }
    
    fn parse_out_condition(&self, node: &roxmltree::Node, job: &mut Job) {
        if let Some(name) = node.attribute("NAME") {
            job.out_conditions.push(Condition::new_out(name.to_string()));
        }
    }
    
    fn parse_variable(&self, node: &roxmltree::Node, job: &mut Job) {
        if let (Some(name), Some(value)) = (node.attribute("NAME"), node.attribute("VALUE")) {
            job.variables.insert(name.to_string(), value.to_string());
        }
    }
    
    fn parse_control_resource(&self, node: &roxmltree::Node, job: &mut Job) {
        if let Some(name) = node.attribute("NAME") {
            job.control_resources.push(ControlResource::new(name.to_string()));
        }
    }
    
    fn parse_quantitative_resource(&self, node: &roxmltree::Node, job: &mut Job) {
        if let Some(name) = node.attribute("NAME") {
            let quant = node.attribute("QUANT")
                .and_then(|q| q.parse::<i32>().ok())
                .unwrap_or(1);
            job.quantitative_resources.push(QuantitativeResource::new(name.to_string(), quant));
        }
    }
    
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

use anyhow::{Context, Result};
use quick_xml::de::from_str;
use std::fs;
use std::path::Path;

use crate::domain::entities::*;
use crate::domain::entities::condition::DoAction;
use crate::domain::entities::folder::FolderType;
use super::control_m_models::*;

pub struct ControlMXmlParser;

impl ControlMXmlParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<Folder>> {
        let xml_content = fs::read_to_string(path)
            .context("Failed to read XML file")?;
        
        self.parse_xml(&xml_content)
    }

    pub fn parse_xml(&self, xml_content: &str) -> Result<Vec<Folder>> {
        let def_table: DefTable = from_str(xml_content)
            .context("Failed to parse XML")?;

        let mut folders = Vec::new();

        for xml_folder in def_table.folders {
            folders.push(self.convert_folder(xml_folder, FolderType::Simple)?);
        }

        for xml_folder in def_table.smart_folders {
            folders.push(self.convert_smart_folder(xml_folder, FolderType::Smart)?);
        }

        for xml_folder in def_table.tables {
            folders.push(self.convert_folder(xml_folder, FolderType::Table)?);
        }

        for xml_folder in def_table.smart_tables {
            folders.push(self.convert_smart_folder(xml_folder, FolderType::SmartTable)?);
        }

        Ok(folders)
    }

    fn convert_folder(&self, xml_folder: XmlFolder, folder_type: FolderType) -> Result<Folder> {
        let folder_name = xml_folder.folder_name
            .or(xml_folder.table_name)
            .unwrap_or_else(|| "UNKNOWN".to_string());

        let mut folder = Folder::new(folder_name.clone(), folder_type);
        folder.datacenter = xml_folder.datacenter;
        folder.application = xml_folder.application;

        for xml_job in xml_folder.jobs {
            let job = self.convert_job(xml_job, folder_name.clone())?;
            folder.add_job(job);
        }

        Ok(folder)
    }

    fn convert_smart_folder(&self, xml_folder: XmlSmartFolder, folder_type: FolderType) -> Result<Folder> {
        let folder_name = xml_folder.folder_name
            .or(xml_folder.table_name)
            .unwrap_or_else(|| "UNKNOWN".to_string());

        let mut folder = Folder::new(folder_name.clone(), folder_type);
        folder.datacenter = xml_folder.datacenter;
        folder.application = xml_folder.application;

        for xml_job in xml_folder.jobs {
            let job = self.convert_job(xml_job, folder_name.clone())?;
            folder.add_job(job);
        }

        for xml_sub_folder in xml_folder.sub_folders {
            let sub_folder = self.convert_smart_folder(xml_sub_folder, FolderType::Smart)?;
            folder.add_sub_folder(sub_folder);
        }

        Ok(folder)
    }

    fn convert_job(&self, xml_job: XmlJob, folder_name: String) -> Result<Job> {
        let job_name = xml_job.job_name.clone().unwrap_or_else(|| "UNKNOWN".to_string());
        let mut job = Job::new(job_name, folder_name);

        job.application = xml_job.application.clone();
        job.sub_application = xml_job.sub_application.clone();
        job.description = xml_job.description.clone();
        job.owner = xml_job.owner.clone();
        job.run_as = xml_job.run_as.clone();
        job.priority = xml_job.priority.clone();
        job.critical = xml_job.critical.as_deref() == Some("Y");
        job.task_type = xml_job.task_type.clone();
        job.cyclic = xml_job.cyclic.as_deref() == Some("Y");
        job.node_id = xml_job.node_id.clone();
        job.cmdline = xml_job.cmdline.clone();

        job.scheduling = self.convert_scheduling(&xml_job);

        job.created_by = xml_job.created_by.clone();
        job.creation_date = xml_job.creation_date.clone();
        job.change_userid = xml_job.change_userid.clone();
        job.change_date = xml_job.change_date.clone();

        for xml_incond in xml_job.in_conditions {
            if let Some(name) = xml_incond.name {
                let mut cond = Condition::new_in(name);
                cond.odate = xml_incond.odate;
                cond.and_or = xml_incond.and_or;
                job.in_conditions.push(cond);
            }
        }

        for xml_outcond in xml_job.out_conditions {
            if let Some(name) = xml_outcond.name {
                let cond = Condition::new_out(name);
                job.out_conditions.push(cond);
            }
        }

        for xml_on in xml_job.on_conditions {
            let mut on_cond = OnCondition::new();
            on_cond.stmt = xml_on.stmt;
            on_cond.code = xml_on.code;
            on_cond.pattern = xml_on.pattern;
            
            for xml_action in xml_on.do_actions {
                if let Some(action) = xml_action.action {
                    on_cond.actions.push(DoAction::Action(action));
                }
            }
            
            job.on_conditions.push(on_cond);
        }

        for xml_ctrl in xml_job.control_resources {
            if let Some(name) = xml_ctrl.name {
                let mut resource = ControlResource::new(name);
                resource.resource_type = xml_ctrl.resource_type;
                resource.on_fail = xml_ctrl.on_fail;
                job.control_resources.push(resource);
            }
        }

        for xml_quant in xml_job.quantitative_resources {
            if let Some(name) = xml_quant.name {
                let quantity = xml_quant.quantity
                    .and_then(|q| q.parse::<i32>().ok())
                    .unwrap_or(1);
                let mut resource = QuantitativeResource::new(name, quantity);
                resource.on_fail = xml_quant.on_fail;
                resource.on_ok = xml_quant.on_ok;
                job.quantitative_resources.push(resource);
            }
        }

        for xml_var in xml_job.variables {
            if let (Some(name), Some(value)) = (xml_var.name, xml_var.value) {
                job.variables.insert(name, value);
            }
        }

        for xml_var in xml_job.auto_edits {
            if let (Some(name), Some(value)) = (xml_var.name, xml_var.value) {
                job.auto_edits.insert(name, value);
            }
        }

        Ok(job)
    }

    fn convert_scheduling(&self, xml_job: &XmlJob) -> SchedulingInfo {
        let mut sched = SchedulingInfo::new();
        
        sched.time_from = xml_job.time_from.clone();
        sched.time_to = xml_job.time_to.clone();
        sched.days = xml_job.days.clone();
        sched.weekdays = xml_job.weekdays.clone();
        sched.days_calendar = xml_job.days_cal.clone();
        sched.weeks_calendar = xml_job.weeks_cal.clone();
        sched.conf_calendar = xml_job.conf_cal.clone();
        sched.cyclic_interval = xml_job.cyclic_interval_sequence.clone();
        sched.cyclic_times = xml_job.cyclic_times_sequence.clone();
        
        if let Some(max_wait) = &xml_job.max_wait {
            sched.max_wait = max_wait.parse::<i32>().ok();
        }
        
        if let Some(max_rerun) = &xml_job.max_rerun {
            sched.max_rerun = max_rerun.parse::<i32>().ok();
        }

        sched
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

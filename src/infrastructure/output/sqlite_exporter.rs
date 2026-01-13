use anyhow::{Context, Result};
use rusqlite::{Connection, params};
use std::path::Path;
use crate::domain::entities::*;

pub type ProgressCallback = Box<dyn Fn(&str)>;

pub struct SqliteExporter {
    conn: Connection,
    progress_callback: Option<ProgressCallback>,
}

impl SqliteExporter {
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let conn = Connection::open(db_path)
            .context("Failed to open SQLite database")?;
        
        let exporter = Self { 
            conn,
            progress_callback: None,
        };
        exporter.create_schema()?;
        
        Ok(exporter)
    }

    pub fn with_progress_callback<F>(mut self, callback: F) -> Self 
    where
        F: Fn(&str) + 'static,
    {
        self.progress_callback = Some(Box::new(callback));
        self
    }

    fn report_progress(&self, message: &str) {
        if let Some(callback) = &self.progress_callback {
            callback(message);
        }
    }

    fn create_schema(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            -- Folders table
            CREATE TABLE IF NOT EXISTS folders (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                folder_name TEXT NOT NULL,
                folder_type TEXT NOT NULL,
                datacenter TEXT,
                application TEXT,
                description TEXT,
                owner TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                UNIQUE(folder_name, datacenter)
            );

            -- Jobs table
            CREATE TABLE IF NOT EXISTS jobs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                job_name TEXT NOT NULL,
                folder_name TEXT NOT NULL,
                application TEXT,
                sub_application TEXT,
                description TEXT,
                owner TEXT,
                run_as TEXT,
                priority TEXT,
                critical INTEGER DEFAULT 0,
                task_type TEXT,
                cyclic INTEGER DEFAULT 0,
                node_id TEXT,
                cmdline TEXT,
                created_by TEXT,
                creation_date TEXT,
                change_userid TEXT,
                change_date TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                UNIQUE(job_name, folder_name)
            );

            -- Job scheduling table
            CREATE TABLE IF NOT EXISTS job_scheduling (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                job_id INTEGER NOT NULL,
                time_from TEXT,
                time_to TEXT,
                days_calendar TEXT,
                weeks_calendar TEXT,
                conf_calendar TEXT,
                interval TEXT,
                max_wait TEXT,
                max_rerun TEXT,
                FOREIGN KEY (job_id) REFERENCES jobs(id) ON DELETE CASCADE
            );

            -- In conditions table
            CREATE TABLE IF NOT EXISTS in_conditions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                job_id INTEGER NOT NULL,
                condition_name TEXT NOT NULL,
                odate TEXT,
                and_or TEXT,
                FOREIGN KEY (job_id) REFERENCES jobs(id) ON DELETE CASCADE
            );

            -- Out conditions table
            CREATE TABLE IF NOT EXISTS out_conditions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                job_id INTEGER NOT NULL,
                condition_name TEXT NOT NULL,
                odate TEXT,
                sign TEXT,
                FOREIGN KEY (job_id) REFERENCES jobs(id) ON DELETE CASCADE
            );

            -- On conditions table
            CREATE TABLE IF NOT EXISTS on_conditions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                job_id INTEGER NOT NULL,
                stmt TEXT,
                code TEXT,
                pattern TEXT,
                FOREIGN KEY (job_id) REFERENCES jobs(id) ON DELETE CASCADE
            );

            -- Do actions table (for on conditions)
            CREATE TABLE IF NOT EXISTS do_actions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                on_condition_id INTEGER NOT NULL,
                action_type TEXT NOT NULL,
                action_value TEXT,
                additional_data TEXT,
                FOREIGN KEY (on_condition_id) REFERENCES on_conditions(id) ON DELETE CASCADE
            );

            -- Control resources table
            CREATE TABLE IF NOT EXISTS control_resources (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                job_id INTEGER NOT NULL,
                resource_name TEXT NOT NULL,
                resource_type TEXT,
                on_fail TEXT,
                FOREIGN KEY (job_id) REFERENCES jobs(id) ON DELETE CASCADE
            );

            -- Quantitative resources table
            CREATE TABLE IF NOT EXISTS quantitative_resources (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                job_id INTEGER NOT NULL,
                resource_name TEXT NOT NULL,
                quantity INTEGER NOT NULL,
                on_fail TEXT,
                on_ok TEXT,
                FOREIGN KEY (job_id) REFERENCES jobs(id) ON DELETE CASCADE
            );

            -- Variables table
            CREATE TABLE IF NOT EXISTS job_variables (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                job_id INTEGER NOT NULL,
                variable_name TEXT NOT NULL,
                variable_value TEXT,
                FOREIGN KEY (job_id) REFERENCES jobs(id) ON DELETE CASCADE
            );

            -- Auto edits table
            CREATE TABLE IF NOT EXISTS job_auto_edits (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                job_id INTEGER NOT NULL,
                edit_name TEXT NOT NULL,
                edit_value TEXT,
                FOREIGN KEY (job_id) REFERENCES jobs(id) ON DELETE CASCADE
            );

            -- Metadata table (for additional job metadata)
            CREATE TABLE IF NOT EXISTS job_metadata (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                job_id INTEGER NOT NULL,
                meta_key TEXT NOT NULL,
                meta_value TEXT,
                FOREIGN KEY (job_id) REFERENCES jobs(id) ON DELETE CASCADE
            );

            -- Create indexes for better query performance
            CREATE INDEX IF NOT EXISTS idx_jobs_folder ON jobs(folder_name);
            CREATE INDEX IF NOT EXISTS idx_jobs_application ON jobs(application);
            CREATE INDEX IF NOT EXISTS idx_jobs_critical ON jobs(critical);
            CREATE INDEX IF NOT EXISTS idx_in_conditions_job ON in_conditions(job_id);
            CREATE INDEX IF NOT EXISTS idx_out_conditions_job ON out_conditions(job_id);
            CREATE INDEX IF NOT EXISTS idx_control_resources_job ON control_resources(job_id);
            CREATE INDEX IF NOT EXISTS idx_quantitative_resources_job ON quantitative_resources(job_id);
            "#
        ).context("Failed to create database schema")?;

        Ok(())
    }

    pub fn export_folders(&self, folders: &[Folder]) -> Result<()> {
        self.report_progress("Starting export...");
        
        for (idx, folder) in folders.iter().enumerate() {
            self.report_progress(&format!("Exporting folder {}/{}: {}", 
                idx + 1, folders.len(), folder.folder_name));
            self.export_folder(folder)?;
        }
        
        self.report_progress("Export completed!");
        Ok(())
    }

    fn export_folder(&self, folder: &Folder) -> Result<()> {
        let folder_type_str = match folder.folder_type {
            FolderType::Simple => "Simple",
            FolderType::Smart => "Smart",
            FolderType::Table => "Table",
            FolderType::SmartTable => "SmartTable",
        };

        self.conn.execute(
            r#"
            INSERT OR REPLACE INTO folders 
            (folder_name, folder_type, datacenter, application, description, owner)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
            params![
                &folder.folder_name,
                folder_type_str,
                &folder.datacenter,
                &folder.application,
                &folder.description,
                &folder.owner,
            ],
        ).context("Failed to insert folder")?;

        for job in &folder.jobs {
            self.export_job(job)?;
        }

        for sub_folder in &folder.sub_folders {
            self.export_folder(sub_folder)?;
        }

        Ok(())
    }

    fn export_job(&self, job: &Job) -> Result<()> {
        self.report_progress(&format!("  → Job: {}", job.job_name));
        
        self.conn.execute(
            r#"
            INSERT OR REPLACE INTO jobs 
            (job_name, folder_name, application, sub_application, description, 
             owner, run_as, priority, critical, task_type, cyclic, node_id, cmdline,
             created_by, creation_date, change_userid, change_date)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)
            "#,
            params![
                &job.job_name,
                &job.folder_name,
                &job.application,
                &job.sub_application,
                &job.description,
                &job.owner,
                &job.run_as,
                &job.priority,
                job.critical as i32,
                &job.task_type,
                job.cyclic as i32,
                &job.node_id,
                &job.cmdline,
                &job.created_by,
                &job.creation_date,
                &job.change_userid,
                &job.change_date,
            ],
        ).context("Failed to insert job")?;

        let job_id = self.conn.last_insert_rowid();

        self.export_job_scheduling(job_id, &job.scheduling)?;
        self.export_in_conditions(job_id, &job.in_conditions)?;
        self.export_out_conditions(job_id, &job.out_conditions)?;
        self.export_on_conditions(job_id, &job.on_conditions)?;
        self.export_control_resources(job_id, &job.control_resources)?;
        self.export_quantitative_resources(job_id, &job.quantitative_resources)?;
        self.export_variables(job_id, &job.variables)?;
        self.export_auto_edits(job_id, &job.auto_edits)?;
        self.export_metadata(job_id, &job.metadata)?;

        Ok(())
    }

    fn export_job_scheduling(&self, job_id: i64, scheduling: &SchedulingInfo) -> Result<()> {
        self.conn.execute(
            r#"
            INSERT INTO job_scheduling 
            (job_id, time_from, time_to, days_calendar, weeks_calendar, conf_calendar)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
            params![
                job_id,
                &scheduling.time_from,
                &scheduling.time_to,
                &scheduling.days_calendar,
                &scheduling.weeks_calendar,
                &scheduling.conf_calendar,
            ],
        ).context("Failed to insert job scheduling")?;

        Ok(())
    }

    fn export_in_conditions(&self, job_id: i64, conditions: &[Condition]) -> Result<()> {
        for condition in conditions {
            if matches!(condition.condition_type, ConditionType::In) {
                self.conn.execute(
                    r#"
                    INSERT INTO in_conditions 
                    (job_id, condition_name, odate, and_or)
                    VALUES (?1, ?2, ?3, ?4)
                    "#,
                    params![
                        job_id,
                        &condition.name,
                        &condition.odate,
                        &condition.and_or,
                    ],
                ).context("Failed to insert in condition")?;
            }
        }
        Ok(())
    }

    fn export_out_conditions(&self, job_id: i64, conditions: &[Condition]) -> Result<()> {
        for condition in conditions {
            if matches!(condition.condition_type, ConditionType::Out) {
                self.conn.execute(
                    r#"
                    INSERT INTO out_conditions 
                    (job_id, condition_name, odate)
                    VALUES (?1, ?2, ?3)
                    "#,
                    params![
                        job_id,
                        &condition.name,
                        &condition.odate,
                    ],
                ).context("Failed to insert out condition")?;
            }
        }
        Ok(())
    }

    fn export_on_conditions(&self, job_id: i64, on_conditions: &[OnCondition]) -> Result<()> {
        for on_cond in on_conditions {
            self.conn.execute(
                r#"
                INSERT INTO on_conditions 
                (job_id, stmt, code, pattern)
                VALUES (?1, ?2, ?3, ?4)
                "#,
                params![
                    job_id,
                    &on_cond.stmt,
                    &on_cond.code,
                    &on_cond.pattern,
                ],
            ).context("Failed to insert on condition")?;

            let on_condition_id = self.conn.last_insert_rowid();

            for action in &on_cond.actions {
                self.export_do_action(on_condition_id, action)?;
            }
        }
        Ok(())
    }

    fn export_do_action(&self, on_condition_id: i64, action: &DoAction) -> Result<()> {
        let (action_type, action_value, additional_data) = match action {
            DoAction::Action(val) => ("Action", val.clone(), None),
            DoAction::Condition { name, sign } => {
                ("Condition", name.clone(), sign.clone())
            }
            DoAction::ForceJob { name, table_name } => {
                ("ForceJob", name.clone(), table_name.clone())
            }
            DoAction::Mail { dest, message } => {
                ("Mail", dest.clone(), Some(message.clone()))
            }
            DoAction::Shout { dest, message } => {
                ("Shout", dest.clone(), Some(message.clone()))
            }
            DoAction::SetVariable { name, value } => {
                ("SetVariable", name.clone(), Some(value.clone()))
            }
        };

        self.conn.execute(
            r#"
            INSERT INTO do_actions 
            (on_condition_id, action_type, action_value, additional_data)
            VALUES (?1, ?2, ?3, ?4)
            "#,
            params![
                on_condition_id,
                action_type,
                &action_value,
                &additional_data,
            ],
        ).context("Failed to insert do action")?;

        Ok(())
    }

    fn export_control_resources(&self, job_id: i64, resources: &[ControlResource]) -> Result<()> {
        for resource in resources {
            self.conn.execute(
                r#"
                INSERT INTO control_resources 
                (job_id, resource_name, resource_type, on_fail)
                VALUES (?1, ?2, ?3, ?4)
                "#,
                params![
                    job_id,
                    &resource.name,
                    &resource.resource_type,
                    &resource.on_fail,
                ],
            ).context("Failed to insert control resource")?;
        }
        Ok(())
    }

    fn export_quantitative_resources(&self, job_id: i64, resources: &[QuantitativeResource]) -> Result<()> {
        for resource in resources {
            self.conn.execute(
                r#"
                INSERT INTO quantitative_resources 
                (job_id, resource_name, quantity, on_fail, on_ok)
                VALUES (?1, ?2, ?3, ?4, ?5)
                "#,
                params![
                    job_id,
                    &resource.name,
                    resource.quantity,
                    &resource.on_fail,
                    &resource.on_ok,
                ],
            ).context("Failed to insert quantitative resource")?;
        }
        Ok(())
    }

    fn export_variables(&self, job_id: i64, variables: &std::collections::HashMap<String, String>) -> Result<()> {
        for (name, value) in variables {
            self.conn.execute(
                r#"
                INSERT INTO job_variables 
                (job_id, variable_name, variable_value)
                VALUES (?1, ?2, ?3)
                "#,
                params![job_id, name, value],
            ).context("Failed to insert job variable")?;
        }
        Ok(())
    }

    fn export_auto_edits(&self, job_id: i64, auto_edits: &std::collections::HashMap<String, String>) -> Result<()> {
        for (name, value) in auto_edits {
            self.conn.execute(
                r#"
                INSERT INTO job_auto_edits 
                (job_id, edit_name, edit_value)
                VALUES (?1, ?2, ?3)
                "#,
                params![job_id, name, value],
            ).context("Failed to insert auto edit")?;
        }
        Ok(())
    }

    fn export_metadata(&self, job_id: i64, metadata: &std::collections::HashMap<String, String>) -> Result<()> {
        for (key, value) in metadata {
            self.conn.execute(
                r#"
                INSERT INTO job_metadata 
                (job_id, meta_key, meta_value)
                VALUES (?1, ?2, ?3)
                "#,
                params![job_id, key, value],
            ).context("Failed to insert metadata")?;
        }
        Ok(())
    }

    pub fn get_statistics(&self) -> Result<DatabaseStatistics> {
        let folder_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM folders",
            [],
            |row| row.get(0),
        )?;

        let job_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM jobs",
            [],
            |row| row.get(0),
        )?;

        let in_condition_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM in_conditions",
            [],
            |row| row.get(0),
        )?;

        let out_condition_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM out_conditions",
            [],
            |row| row.get(0),
        )?;

        let control_resource_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM control_resources",
            [],
            |row| row.get(0),
        )?;

        Ok(DatabaseStatistics {
            folder_count: folder_count as usize,
            job_count: job_count as usize,
            in_condition_count: in_condition_count as usize,
            out_condition_count: out_condition_count as usize,
            control_resource_count: control_resource_count as usize,
        })
    }
}

#[derive(Debug)]
pub struct DatabaseStatistics {
    pub folder_count: usize,
    pub job_count: usize,
    pub in_condition_count: usize,
    pub out_condition_count: usize,
    pub control_resource_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_exporter() {
        let exporter = SqliteExporter::new(":memory:").unwrap();
        let stats = exporter.get_statistics().unwrap();
        assert_eq!(stats.folder_count, 0);
        assert_eq!(stats.job_count, 0);
    }

    #[test]
    fn test_export_folder() {
        let exporter = SqliteExporter::new(":memory:").unwrap();
        
        let folder = Folder::new("TEST_FOLDER".to_string(), FolderType::Simple);
        
        exporter.export_folders(&[folder]).unwrap();
        
        let stats = exporter.get_statistics().unwrap();
        assert_eq!(stats.folder_count, 1);
    }

    #[test]
    fn test_export_job_with_conditions() {
        let exporter = SqliteExporter::new(":memory:").unwrap();
        
        let mut folder = Folder::new("TEST_FOLDER".to_string(), FolderType::Simple);
        let mut job = Job::new("TEST_JOB".to_string(), "TEST_FOLDER".to_string());
        
        job.in_conditions.push(Condition::new_in("COND1".to_string()));
        job.out_conditions.push(Condition::new_out("COND2".to_string()));
        
        folder.add_job(job);
        
        exporter.export_folders(&[folder]).unwrap();
        
        let stats = exporter.get_statistics().unwrap();
        assert_eq!(stats.job_count, 1);
        assert_eq!(stats.in_condition_count, 1);
        assert_eq!(stats.out_condition_count, 1);
    }

    #[test]
    fn test_export_job_with_resources() {
        let exporter = SqliteExporter::new(":memory:").unwrap();
        
        let mut folder = Folder::new("TEST_FOLDER".to_string(), FolderType::Simple);
        let mut job = Job::new("TEST_JOB".to_string(), "TEST_FOLDER".to_string());
        
        job.control_resources.push(ControlResource::new("DB_LOCK".to_string()));
        job.quantitative_resources.push(QuantitativeResource::new("CPU_POOL".to_string(), 5));
        
        folder.add_job(job);
        
        exporter.export_folders(&[folder]).unwrap();
        
        let stats = exporter.get_statistics().unwrap();
        assert_eq!(stats.control_resource_count, 1);
    }
}

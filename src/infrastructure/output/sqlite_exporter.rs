//! SQLite Database Exporter module
//!
//! This module provides functionality to export Control-M job definitions
//! to a SQLite database with optimized schema and indexing for fast queries.
//! Supports progress reporting and bulk insert operations.

use anyhow::{Context, Result};
use rusqlite::{Connection, params, Transaction};
use std::path::Path;
use crate::domain::entities::*;

/// Type alias for progress callback function
///
/// Callbacks receive progress messages during export operations
pub type ProgressCallback = Box<dyn Fn(&str)>;

/// SQLite database exporter for Control-M job definitions
///
/// Exports folders, jobs, and all related entities (conditions, resources, variables)
/// to a SQLite database with a normalized schema. Optimized for bulk inserts
/// with WAL mode and appropriate indexing for fast queries.
pub struct SqliteExporter {
    /// SQLite database connection
    conn: Connection,
    /// Optional callback for progress reporting
    progress_callback: Option<ProgressCallback>,
    /// Counter for tracking exported jobs (used for throttled progress reporting)
    job_counter: std::cell::Cell<usize>,
}

impl SqliteExporter {
    /// Creates a new SQLite exporter and initializes the database
    ///
    /// Opens or creates a SQLite database at the specified path,
    /// configures it for optimal bulk insert performance, and creates
    /// the necessary schema.
    ///
    /// # Arguments
    ///
    /// * `db_path` - Path to the SQLite database file
    ///
    /// # Returns
    ///
    /// Result containing the SqliteExporter or an error
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Database file cannot be opened/created
    /// - Schema creation fails
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let conn = Connection::open(db_path)
            .context("Failed to open SQLite database")?;
        
        // Optimize SQLite for bulk inserts with WAL mode and memory optimizations
        conn.execute_batch(
            r#"
            PRAGMA journal_mode = WAL;        -- Write-Ahead Logging for better concurrency
            PRAGMA synchronous = NORMAL;      -- Balance between safety and speed
            PRAGMA cache_size = 10000;        -- Larger cache for better performance
            PRAGMA temp_store = MEMORY;       -- Store temp tables in memory
            "#
        )?;
        
        let exporter = Self { 
            conn,
            progress_callback: None,
            job_counter: std::cell::Cell::new(0),
        };
        exporter.create_schema()?;
        
        Ok(exporter)
    }

    /// Adds a progress callback to the exporter
    ///
    /// The callback will be invoked during export operations to report progress.
    ///
    /// # Arguments
    ///
    /// * `callback` - Function to call with progress messages
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_progress_callback<F>(mut self, callback: F) -> Self 
    where
        F: Fn(&str) + 'static,
    {
        self.progress_callback = Some(Box::new(callback));
        self
    }

    /// Reports progress to the callback if one is set
    ///
    /// # Arguments
    ///
    /// * `message` - Progress message to report
    fn report_progress(&self, message: &str) {
        if let Some(callback) = &self.progress_callback {
            callback(message);
        }
    }

    /// Reports progress with throttling to avoid excessive callbacks
    ///
    /// Only reports every 10 jobs unless forced, reducing callback overhead
    /// during bulk operations.
    ///
    /// # Arguments
    ///
    /// * `message` - Progress message to report
    /// * `force` - If true, bypasses throttling and always reports
    fn report_progress_throttled(&self, message: &str, force: bool) {
        let count = self.job_counter.get();
        // Report every 10 jobs or when forced
        if force || count % 10 == 0 {
            self.report_progress(message);
        }
    }

    /// Creates the database schema with all tables and indexes
    ///
    /// Creates a normalized schema with:
    /// - Folders and jobs tables
    /// - Child tables for conditions, resources, variables
    /// - Comprehensive indexes for query performance
    /// - Foreign key constraints with CASCADE delete
    ///
    /// # Returns
    ///
    /// Result indicating success or error
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
                appl_type TEXT,
                appl_ver TEXT,
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
            
            -- Single column indexes for exact match searches
            CREATE INDEX IF NOT EXISTS idx_jobs_folder ON jobs(folder_name);
            CREATE INDEX IF NOT EXISTS idx_jobs_application ON jobs(application);
            CREATE INDEX IF NOT EXISTS idx_jobs_critical ON jobs(critical);
            CREATE INDEX IF NOT EXISTS idx_jobs_appl_type ON jobs(appl_type);
            CREATE INDEX IF NOT EXISTS idx_jobs_appl_ver ON jobs(appl_ver);
            CREATE INDEX IF NOT EXISTS idx_jobs_task_type ON jobs(task_type);
            CREATE INDEX IF NOT EXISTS idx_jobs_owner ON jobs(owner);
            
            -- Composite indexes for common filter combinations
            CREATE INDEX IF NOT EXISTS idx_jobs_app_type ON jobs(application, appl_type);
            CREATE INDEX IF NOT EXISTS idx_jobs_folder_app ON jobs(folder_name, application);
            CREATE INDEX IF NOT EXISTS idx_jobs_critical_app ON jobs(critical, application);
            
            -- Full-text search support for job_name (using trigram for LIKE queries)
            CREATE INDEX IF NOT EXISTS idx_jobs_name ON jobs(job_name);
            
            -- Foreign key indexes for all child tables
            CREATE INDEX IF NOT EXISTS idx_in_conditions_job ON in_conditions(job_id);
            CREATE INDEX IF NOT EXISTS idx_out_conditions_job ON out_conditions(job_id);
            CREATE INDEX IF NOT EXISTS idx_on_conditions_job ON on_conditions(job_id);
            CREATE INDEX IF NOT EXISTS idx_do_actions_on_condition ON do_actions(on_condition_id);
            CREATE INDEX IF NOT EXISTS idx_control_resources_job ON control_resources(job_id);
            CREATE INDEX IF NOT EXISTS idx_quantitative_resources_job ON quantitative_resources(job_id);
            CREATE INDEX IF NOT EXISTS idx_job_scheduling_job ON job_scheduling(job_id);
            CREATE INDEX IF NOT EXISTS idx_job_variables_job ON job_variables(job_id);
            CREATE INDEX IF NOT EXISTS idx_job_auto_edits_job ON job_auto_edits(job_id);
            CREATE INDEX IF NOT EXISTS idx_job_metadata_job ON job_metadata(job_id);
            "#
        ).context("Failed to create database schema")?;

        Ok(())
    }

    /// Exports folders and all their jobs to the database
    ///
    /// Exports all folders recursively, including jobs and sub-folders,
    /// within a single transaction for atomicity and performance.
    ///
    /// # Arguments
    ///
    /// * `folders` - Slice of Folder entities to export
    ///
    /// # Returns
    ///
    /// Result indicating success or error
    pub fn export_folders(&self, folders: &[Folder]) -> Result<()> {
        self.report_progress("Starting export...");
        
        // Use transaction for all exports (atomic and faster)
        let tx = self.conn.unchecked_transaction()?;
        
        for (idx, folder) in folders.iter().enumerate() {
            self.report_progress(&format!("📁 Exporting folder {}/{}: {}", 
                idx + 1, folders.len(), folder.folder_name));
            self.export_folder_tx(&tx, folder)?;
        }
        
        self.report_progress("💾 Committing to database...");
        tx.commit()?;
        
        self.report_progress("Export completed!");
        Ok(())
    }

    /// Exports a single folder within a transaction
    ///
    /// Recursively exports the folder, all its jobs, and sub-folders.
    ///
    /// # Arguments
    ///
    /// * `tx` - Active database transaction
    /// * `folder` - Folder entity to export
    ///
    /// # Returns
    ///
    /// Result indicating success or error
    fn export_folder_tx(&self, tx: &Transaction, folder: &Folder) -> Result<()> {
        // Convert folder type enum to string for database storage
        let folder_type_str = match folder.folder_type {
            FolderType::Simple => "Simple",
            FolderType::Smart => "Smart",
            FolderType::Table => "Table",
            FolderType::SmartTable => "SmartTable",
        };

        tx.execute(
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

        // Export all jobs in this folder
        for job in &folder.jobs {
            self.export_job_tx(tx, job)?;
        }

        // Recursively export sub-folders
        for sub_folder in &folder.sub_folders {
            self.export_folder_tx(tx, sub_folder)?;
        }

        Ok(())
    }

    /// Exports a single job and all its related entities within a transaction
    ///
    /// Exports the job along with scheduling info, conditions, resources,
    /// variables, and metadata.
    ///
    /// # Arguments
    ///
    /// * `tx` - Active database transaction
    /// * `job` - Job entity to export
    ///
    /// # Returns
    ///
    /// Result indicating success or error
    fn export_job_tx(&self, tx: &Transaction, job: &Job) -> Result<()> {
        // Increment job counter for progress reporting
        let count = self.job_counter.get() + 1;
        self.job_counter.set(count);
        
        // Throttled progress reporting (every 10 jobs to reduce overhead)
        self.report_progress_throttled(&format!("Job: {}", job.job_name), false);
        
        tx.prepare_cached(
            r#"
            INSERT OR REPLACE INTO jobs (
                job_name, folder_name, application, sub_application,
                appl_type, appl_ver,
                description, owner, run_as, priority, critical, task_type, cyclic,
                node_id, cmdline, created_by, creation_date, change_userid, change_date
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )?
        .execute(params![
            &job.job_name,
            &job.folder_name,
            &job.application,
            &job.sub_application,
            &job.appl_type,
            &job.appl_ver,
            &job.description,
            &job.owner,
            &job.run_as,
            &job.priority,
            if job.critical { 1 } else { 0 },
            &job.task_type,
            if job.cyclic { 1 } else { 0 },
            &job.node_id,
            &job.cmdline,
            &job.created_by,
            &job.creation_date,
            &job.change_userid,
            &job.change_date,
        ])?;

        let job_id = tx.last_insert_rowid();

        self.export_job_scheduling_tx(tx, job_id, &job.scheduling)?;
        self.export_in_conditions_tx(tx, job_id, &job.in_conditions)?;
        self.export_out_conditions_tx(tx, job_id, &job.out_conditions)?;
        self.export_on_conditions_tx(tx, job_id, &job.on_conditions)?;
        self.export_control_resources_tx(tx, job_id, &job.control_resources)?;
        self.export_quantitative_resources_tx(tx, job_id, &job.quantitative_resources)?;
        self.export_variables_tx(tx, job_id, &job.variables)?;
        self.export_auto_edits_tx(tx, job_id, &job.auto_edits)?;
        self.export_metadata_tx(tx, job_id, &job.metadata)?;

        Ok(())
    }

    /// Exports job scheduling information within a transaction
    ///
    /// # Arguments
    ///
    /// * `tx` - Active database transaction
    /// * `job_id` - ID of the parent job
    /// * `scheduling` - Scheduling information to export
    ///
    /// # Returns
    ///
    /// Result indicating success or error
    fn export_job_scheduling_tx(&self, tx: &Transaction, job_id: i64, scheduling: &SchedulingInfo) -> Result<()> {
        tx.execute(
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

    /// Exports input conditions for a job within a transaction
    ///
    /// Uses prepared statements for efficient batch insertion.
    ///
    /// # Arguments
    ///
    /// * `tx` - Active database transaction
    /// * `job_id` - ID of the parent job
    /// * `conditions` - Slice of conditions to export
    ///
    /// # Returns
    ///
    /// Result indicating success or error
    fn export_in_conditions_tx(&self, tx: &Transaction, job_id: i64, conditions: &[Condition]) -> Result<()> {
        if conditions.is_empty() {
            return Ok(());
        }

        // Use prepared statement for better performance
        let mut stmt = tx.prepare_cached(
            "INSERT INTO in_conditions (job_id, condition_name, odate, and_or) VALUES (?1, ?2, ?3, ?4)"
        )?;

        for condition in conditions {
            if matches!(condition.condition_type, ConditionType::In) {
                stmt.execute(params![
                    job_id,
                    &condition.name,
                    &condition.odate,
                    &condition.and_or,
                ]).context("Failed to insert in condition")?;
            }
        }
        Ok(())
    }

    /// Exports output conditions for a job within a transaction
    ///
    /// Uses prepared statements for efficient batch insertion.
    ///
    /// # Arguments
    ///
    /// * `tx` - Active database transaction
    /// * `job_id` - ID of the parent job
    /// * `conditions` - Slice of conditions to export
    ///
    /// # Returns
    ///
    /// Result indicating success or error
    fn export_out_conditions_tx(&self, tx: &Transaction, job_id: i64, conditions: &[Condition]) -> Result<()> {
        if conditions.is_empty() {
            return Ok(());
        }

        // Use prepared statement for better performance
        let mut stmt = tx.prepare_cached(
            "INSERT INTO out_conditions (job_id, condition_name, odate) VALUES (?1, ?2, ?3)"
        )?;

        for condition in conditions {
            if matches!(condition.condition_type, ConditionType::Out) {
                stmt.execute(params![
                    job_id,
                    &condition.name,
                    &condition.odate,
                ]).context("Failed to insert out condition")?;
            }
        }
        Ok(())
    }

    /// Exports ON conditions and their actions for a job within a transaction
    ///
    /// ON conditions are event-based triggers with associated actions.
    /// Exports both the condition and all its actions.
    ///
    /// # Arguments
    ///
    /// * `tx` - Active database transaction
    /// * `job_id` - ID of the parent job
    /// * `on_conditions` - Slice of ON conditions to export
    ///
    /// # Returns
    ///
    /// Result indicating success or error
    fn export_on_conditions_tx(&self, tx: &Transaction, job_id: i64, on_conditions: &[OnCondition]) -> Result<()> {
        if on_conditions.is_empty() {
            return Ok(());
        }

        let mut stmt = tx.prepare_cached(
            "INSERT INTO on_conditions (job_id, stmt, code, pattern) VALUES (?1, ?2, ?3, ?4)"
        )?;

        for on_cond in on_conditions {
            stmt.execute(params![
                job_id,
                &on_cond.stmt,
                &on_cond.code,
                &on_cond.pattern,
            ]).context("Failed to insert on condition")?;

            // Get the ID of the just-inserted ON condition
            let on_condition_id = tx.last_insert_rowid();

            // Export all actions for this ON condition
            for action in &on_cond.actions {
                self.export_do_action_tx(tx, on_condition_id, action)?;
            }
        }
        Ok(())
    }

    /// Exports a single DO action for an ON condition within a transaction
    ///
    /// Converts the DoAction enum to database-friendly format with
    /// action type, value, and additional data fields.
    ///
    /// # Arguments
    ///
    /// * `tx` - Active database transaction
    /// * `on_condition_id` - ID of the parent ON condition
    /// * `action` - DoAction to export
    ///
    /// # Returns
    ///
    /// Result indicating success or error
    fn export_do_action_tx(&self, tx: &Transaction, on_condition_id: i64, action: &DoAction) -> Result<()> {
        // Convert DoAction enum to database fields
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

        tx.execute(
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

    /// Exports control resources for a job within a transaction
    ///
    /// Control resources act as mutexes for job synchronization.
    ///
    /// # Arguments
    ///
    /// * `tx` - Active database transaction
    /// * `job_id` - ID of the parent job
    /// * `resources` - Slice of control resources to export
    ///
    /// # Returns
    ///
    /// Result indicating success or error
    fn export_control_resources_tx(&self, tx: &Transaction, job_id: i64, resources: &[ControlResource]) -> Result<()> {
        if resources.is_empty() {
            return Ok(());
        }

        let mut stmt = tx.prepare_cached(
            "INSERT INTO control_resources (job_id, resource_name, resource_type, on_fail) VALUES (?1, ?2, ?3, ?4)"
        )?;

        for resource in resources {
            stmt.execute(params![
                job_id,
                &resource.name,
                &resource.resource_type,
                &resource.on_fail,
            ]).context("Failed to insert control resource")?;
        }
        Ok(())
    }

    /// Exports quantitative resources for a job within a transaction
    ///
    /// Quantitative resources manage limited resource pools with quantities.
    ///
    /// # Arguments
    ///
    /// * `tx` - Active database transaction
    /// * `job_id` - ID of the parent job
    /// * `resources` - Slice of quantitative resources to export
    ///
    /// # Returns
    ///
    /// Result indicating success or error
    fn export_quantitative_resources_tx(&self, tx: &Transaction, job_id: i64, resources: &[QuantitativeResource]) -> Result<()> {
        if resources.is_empty() {
            return Ok(());
        }

        let mut stmt = tx.prepare_cached(
            "INSERT INTO quantitative_resources (job_id, resource_name, quantity, on_fail, on_ok) VALUES (?1, ?2, ?3, ?4, ?5)"
        )?;

        for resource in resources {
            stmt.execute(params![
                job_id,
                &resource.name,
                resource.quantity,
                &resource.on_fail,
                &resource.on_ok,
            ]).context("Failed to insert quantitative resource")?;
        }
        Ok(())
    }

    /// Exports job variables within a transaction
    ///
    /// # Arguments
    ///
    /// * `tx` - Active database transaction
    /// * `job_id` - ID of the parent job
    /// * `variables` - HashMap of variable names to values
    ///
    /// # Returns
    ///
    /// Result indicating success or error
    fn export_variables_tx(&self, tx: &Transaction, job_id: i64, variables: &std::collections::HashMap<String, String>) -> Result<()> {
        if variables.is_empty() {
            return Ok(());
        }

        let mut stmt = tx.prepare_cached(
            "INSERT INTO job_variables (job_id, variable_name, variable_value) VALUES (?1, ?2, ?3)"
        )?;

        for (name, value) in variables {
            stmt.execute(params![job_id, name, value])
                .context("Failed to insert job variable")?;
        }
        Ok(())
    }

    /// Exports job auto-edits within a transaction
    ///
    /// Auto-edits are automatic variable modifications.
    ///
    /// # Arguments
    ///
    /// * `tx` - Active database transaction
    /// * `job_id` - ID of the parent job
    /// * `auto_edits` - HashMap of auto-edit names to values
    ///
    /// # Returns
    ///
    /// Result indicating success or error
    fn export_auto_edits_tx(&self, tx: &Transaction, job_id: i64, auto_edits: &std::collections::HashMap<String, String>) -> Result<()> {
        if auto_edits.is_empty() {
            return Ok(());
        }

        let mut stmt = tx.prepare_cached(
            "INSERT INTO job_auto_edits (job_id, edit_name, edit_value) VALUES (?1, ?2, ?3)"
        )?;

        for (name, value) in auto_edits {
            stmt.execute(params![job_id, name, value])
                .context("Failed to insert auto edit")?;
        }
        Ok(())
    }

    /// Exports job metadata within a transaction
    ///
    /// Metadata stores additional key-value pairs for jobs.
    ///
    /// # Arguments
    ///
    /// * `tx` - Active database transaction
    /// * `job_id` - ID of the parent job
    /// * `metadata` - HashMap of metadata keys to values
    ///
    /// # Returns
    ///
    /// Result indicating success or error
    fn export_metadata_tx(&self, tx: &Transaction, job_id: i64, metadata: &std::collections::HashMap<String, String>) -> Result<()> {
        if metadata.is_empty() {
            return Ok(());
        }

        let mut stmt = tx.prepare_cached(
            "INSERT INTO job_metadata (job_id, meta_key, meta_value) VALUES (?1, ?2, ?3)"
        )?;

        for (key, value) in metadata {
            stmt.execute(params![job_id, key, value])
                .context("Failed to insert metadata")?;
        }
        Ok(())
    }

    /// Retrieves statistics about the exported data
    ///
    /// Queries the database to count folders, jobs, conditions, and resources.
    ///
    /// # Returns
    ///
    /// Result containing DatabaseStatistics or an error
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

/// Database statistics structure
///
/// Contains counts of various entities in the exported database.
#[derive(Debug)]
pub struct DatabaseStatistics {
    /// Number of folders in the database
    pub folder_count: usize,
    /// Number of jobs in the database
    pub job_count: usize,
    /// Number of input conditions in the database
    pub in_condition_count: usize,
    /// Number of output conditions in the database
    pub out_condition_count: usize,
    /// Number of control resources in the database
    pub control_resource_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests creating an exporter with in-memory database
    #[test]
    fn test_create_exporter() {
        let exporter = SqliteExporter::new(":memory:").unwrap();
        let stats = exporter.get_statistics().unwrap();
        assert_eq!(stats.folder_count, 0);
        assert_eq!(stats.job_count, 0);
    }

    /// Tests exporting a simple folder
    #[test]
    fn test_export_folder() {
        let exporter = SqliteExporter::new(":memory:").unwrap();
        
        let folder = Folder::new("TEST_FOLDER".to_string(), FolderType::Simple);
        
        exporter.export_folders(&[folder]).unwrap();
        
        let stats = exporter.get_statistics().unwrap();
        assert_eq!(stats.folder_count, 1);
    }

    /// Tests exporting a job with conditions
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

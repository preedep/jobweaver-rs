use anyhow::Result;
use rusqlite::{Connection, params, OptionalExtension};
use std::sync::{Arc, Mutex};

use crate::web::models::*;

pub struct JobRepository {
    conn: Arc<Mutex<Connection>>,
}

impl JobRepository {
    pub fn new(database_path: &str) -> Result<Self> {
        let conn = Connection::open(database_path)?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub fn search_jobs(&self, request: &JobSearchRequest) -> Result<JobSearchResponse> {
        tracing::info!("🔍 [SEARCH] Backend received search request");
        tracing::debug!("[SEARCH] Request: job_name={:?}, folder={:?}, app={:?}, task_type={:?}", 
                       request.job_name, request.folder_name, request.application, request.task_type);
        
        let conn = self.conn.lock().unwrap();
        
        let page = request.page.unwrap_or(1);
        let per_page = request.per_page.unwrap_or(50);
        let offset = (page - 1) * per_page;
        
        let (where_clause, params_vec) = self.build_where_clause(request);
        tracing::info!("📋 [SEARCH] WHERE clause: {}", if where_clause.is_empty() { "(none)" } else { &where_clause });
        tracing::debug!("[SEARCH] Params count: {}", params_vec.len());
        
        let (sort_by, sort_order) = self.get_sort_params(request);
        
        let total = self.count_total_jobs(&conn, &where_clause, &params_vec)?;
        tracing::info!("✅ [SEARCH] Found {} total jobs matching criteria", total);
        
        let jobs = self.execute_search_query(&conn, &where_clause, &params_vec, &sort_by, &sort_order, per_page, offset)?;
        tracing::info!("📦 [SEARCH] Returning {} jobs for page {}", jobs.len(), page);
        
        let total_pages = (total + per_page - 1) / per_page;
        
        Ok(JobSearchResponse {
            jobs,
            total,
            page,
            per_page,
            total_pages,
        })
    }
    
    fn build_where_clause(&self, request: &JobSearchRequest) -> (String, Vec<Box<dyn rusqlite::ToSql>>) {
        let mut where_clauses = Vec::new();
        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
        
        if let Some(ref job_name) = request.job_name {
            where_clauses.push("j.job_name LIKE ?");
            params_vec.push(Box::new(format!("%{}%", job_name)));
        }
        
        if let Some(ref folder_name) = request.folder_name {
            where_clauses.push("j.folder_name LIKE ?");
            params_vec.push(Box::new(format!("%{}%", folder_name)));
        }
        
        if let Some(ref application) = request.application {
            where_clauses.push("j.application = ?");
            params_vec.push(Box::new(application.clone()));
        }
        
        if let Some(critical) = request.critical {
            where_clauses.push("j.critical = ?");
            params_vec.push(Box::new(if critical { 1 } else { 0 }));
        }
        
        if let Some(ref task_type) = request.task_type {
            where_clauses.push("j.task_type = ?");
            params_vec.push(Box::new(task_type.clone()));
        }
        
        if let Some(ref appl_type) = request.appl_type {
            where_clauses.push("j.appl_type = ?");
            params_vec.push(Box::new(appl_type.clone()));
        }
        
        if let Some(ref appl_ver) = request.appl_ver {
            where_clauses.push("j.appl_ver = ?");
            params_vec.push(Box::new(appl_ver.clone()));
        }
        
        if let Some(min_deps) = request.min_dependencies {
            where_clauses.push("(SELECT COUNT(*) FROM in_conditions WHERE in_conditions.job_id = j.id) >= ?");
            params_vec.push(Box::new(min_deps));
        }
        
        if let Some(max_deps) = request.max_dependencies {
            where_clauses.push("(SELECT COUNT(*) FROM in_conditions WHERE in_conditions.job_id = j.id) <= ?");
            params_vec.push(Box::new(max_deps));
        }
        
        if let Some(has_vars) = request.has_variables {
            if has_vars {
                where_clauses.push("(SELECT COUNT(*) FROM job_variables WHERE job_id = j.id) > 0");
            } else {
                where_clauses.push("(SELECT COUNT(*) FROM job_variables WHERE job_id = j.id) = 0");
            }
        }
        
        if let Some(min_vars) = request.min_variables {
            where_clauses.push("(SELECT COUNT(*) FROM job_variables WHERE job_id = j.id) >= ?");
            params_vec.push(Box::new(min_vars));
        }
        
        let where_clause = if where_clauses.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", where_clauses.join(" AND "))
        };
        
        (where_clause, params_vec)
    }
    
    fn get_sort_params(&self, request: &JobSearchRequest) -> (String, String) {
        let sort_by = request.sort_by.as_deref().unwrap_or("job_name").to_string();
        let sort_order = match request.sort_order {
            Some(SortOrder::Desc) => "DESC".to_string(),
            _ => "ASC".to_string(),
        };
        (sort_by, sort_order)
    }
    
    fn count_total_jobs(
        &self,
        conn: &rusqlite::Connection,
        where_clause: &str,
        params_vec: &[Box<dyn rusqlite::ToSql>]
    ) -> Result<u32> {
        let count_query = format!("SELECT COUNT(*) FROM jobs j {}", where_clause);
        let total: u32 = conn.query_row(
            &count_query,
            rusqlite::params_from_iter(params_vec.iter().map(|p| p.as_ref())),
            |row| row.get(0),
        )?;
        Ok(total)
    }
    
    fn execute_search_query(
        &self,
        conn: &rusqlite::Connection,
        where_clause: &str,
        params_vec: &[Box<dyn rusqlite::ToSql>],
        sort_by: &String,
        sort_order: &String,
        per_page: u32,
        offset: u32
    ) -> Result<Vec<JobDetail>> {
        let query = format!(
            r#"
            SELECT 
                j.id, j.job_name, j.folder_name, j.application, j.sub_application,
                COALESCE(j.appl_type, '') as appl_type, COALESCE(j.appl_ver, '') as appl_ver,
                j.description, j.owner, j.run_as, j.priority, j.critical,
                j.task_type, j.cyclic, j.node_id, j.cmdline,
                (SELECT COUNT(*) FROM in_conditions WHERE job_id = j.id) as in_cond_count,
                (SELECT COUNT(*) FROM out_conditions WHERE job_id = j.id) as out_cond_count,
                (SELECT COUNT(*) FROM control_resources WHERE job_id = j.id) as ctrl_res_count,
                (SELECT COUNT(*) FROM job_variables WHERE job_id = j.id) as var_count
            FROM jobs j
            {}
            ORDER BY {} {}
            LIMIT ? OFFSET ?
            "#,
            where_clause, sort_by, sort_order
        );
        
        let mut stmt = conn.prepare(&query)?;
        let mut all_params = params_vec.iter().map(|p| p.as_ref()).collect::<Vec<_>>();
        all_params.push(&per_page as &dyn rusqlite::ToSql);
        all_params.push(&offset as &dyn rusqlite::ToSql);
        
        let jobs = stmt.query_map(
            rusqlite::params_from_iter(all_params),
            Self::map_row_to_job_detail,
        )?
        .collect::<Result<Vec<_>, _>>()?;
        
        Ok(jobs)
    }
    
    fn map_row_to_job_detail(row: &rusqlite::Row) -> rusqlite::Result<JobDetail> {
        let appl_type: String = row.get(5)?;
        let appl_ver: String = row.get(6)?;
        
        Ok(JobDetail {
            id: row.get(0)?,
            job_name: row.get(1)?,
            folder_name: row.get(2)?,
            application: row.get(3)?,
            sub_application: row.get(4)?,
            appl_type: if appl_type.is_empty() { None } else { Some(appl_type) },
            appl_ver: if appl_ver.is_empty() { None } else { Some(appl_ver) },
            description: row.get(7)?,
            owner: row.get(8)?,
            run_as: row.get(9)?,
            priority: row.get(10)?,
            critical: row.get::<_, i32>(11)? == 1,
            task_type: row.get(12)?,
            cyclic: row.get::<_, i32>(13)? == 1,
            node_id: row.get(14)?,
            cmdline: row.get(15)?,
            in_conditions_count: row.get(16)?,
            out_conditions_count: row.get(17)?,
            control_resources_count: row.get(18)?,
            variables_count: row.get(19)?,
        })
    }

    pub fn get_job_detail(&self, job_id: i64) -> Result<Option<JobDetailFull>> {
        let conn = self.conn.lock().unwrap();
        
        let job: Option<JobDetail> = conn.query_row(
            r#"
            SELECT 
                j.id, j.job_name, j.folder_name, j.application, j.sub_application,
                COALESCE(j.appl_type, '') as appl_type, COALESCE(j.appl_ver, '') as appl_ver,
                j.description, j.owner, j.run_as, j.priority, j.critical,
                j.task_type, j.cyclic, j.node_id, j.cmdline,
                (SELECT COUNT(*) FROM in_conditions WHERE job_id = j.id),
                (SELECT COUNT(*) FROM out_conditions WHERE job_id = j.id),
                (SELECT COUNT(*) FROM control_resources WHERE job_id = j.id),
                (SELECT COUNT(*) FROM job_variables WHERE job_id = j.id)
            FROM jobs j
            WHERE j.id = ?
            "#,
            params![job_id],
            |row| {
                let appl_type: String = row.get(5)?;
                let appl_ver: String = row.get(6)?;
                Ok(JobDetail {
                    id: row.get(0)?,
                    job_name: row.get(1)?,
                    folder_name: row.get(2)?,
                    application: row.get(3)?,
                    sub_application: row.get(4)?,
                    appl_type: if appl_type.is_empty() { None } else { Some(appl_type) },
                    appl_ver: if appl_ver.is_empty() { None } else { Some(appl_ver) },
                    description: row.get(7)?,
                    owner: row.get(8)?,
                    run_as: row.get(9)?,
                    priority: row.get(10)?,
                    critical: row.get::<_, i32>(11)? == 1,
                    task_type: row.get(12)?,
                    cyclic: row.get::<_, i32>(13)? == 1,
                    node_id: row.get(14)?,
                    cmdline: row.get(15)?,
                    in_conditions_count: row.get(16)?,
                    out_conditions_count: row.get(17)?,
                    control_resources_count: row.get(18)?,
                    variables_count: row.get(19)?,
                })
            },
        ).optional()?;
        
        if let Some(job) = job {
            let scheduling = self.get_job_scheduling(&conn, job_id)?;
            let in_conditions = self.get_in_conditions(&conn, job_id)?;
            let out_conditions = self.get_out_conditions(&conn, job_id)?;
            let on_conditions = self.get_on_conditions(&conn, job_id)?;
            let control_resources = self.get_control_resources(&conn, job_id)?;
            let quantitative_resources = self.get_quantitative_resources(&conn, job_id)?;
            let variables = self.get_variables(&conn, job_id)?;
            let auto_edits = self.get_auto_edits(&conn, job_id)?;
            let metadata = self.get_metadata(&conn, job_id)?;
            
            Ok(Some(JobDetailFull {
                job,
                scheduling,
                in_conditions,
                out_conditions,
                on_conditions,
                control_resources,
                quantitative_resources,
                variables,
                auto_edits,
                metadata,
            }))
        } else {
            Ok(None)
        }
    }

    fn get_job_scheduling(&self, conn: &Connection, job_id: i64) -> Result<Option<JobScheduling>> {
        conn.query_row(
            "SELECT time_from, time_to, days_calendar, weeks_calendar, conf_calendar FROM job_scheduling WHERE job_id = ?",
            params![job_id],
            |row| {
                Ok(JobScheduling {
                    time_from: row.get(0)?,
                    time_to: row.get(1)?,
                    days_calendar: row.get(2)?,
                    weeks_calendar: row.get(3)?,
                    conf_calendar: row.get(4)?,
                })
            },
        ).optional().map_err(Into::into)
    }

    fn get_in_conditions(&self, conn: &Connection, job_id: i64) -> Result<Vec<Condition>> {
        let mut stmt = conn.prepare("SELECT condition_name, odate, and_or FROM in_conditions WHERE job_id = ?")?;
        let conditions = stmt.query_map(params![job_id], |row| {
            Ok(Condition {
                condition_name: row.get(0)?,
                odate: row.get(1)?,
                and_or: row.get(2)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        Ok(conditions)
    }

    fn get_out_conditions(&self, conn: &Connection, job_id: i64) -> Result<Vec<Condition>> {
        let mut stmt = conn.prepare("SELECT condition_name, odate, NULL FROM out_conditions WHERE job_id = ?")?;
        let conditions = stmt.query_map(params![job_id], |row| {
            Ok(Condition {
                condition_name: row.get(0)?,
                odate: row.get(1)?,
                and_or: row.get(2)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        Ok(conditions)
    }

    fn get_on_conditions(&self, conn: &Connection, job_id: i64) -> Result<Vec<OnCondition>> {
        let mut stmt = conn.prepare("SELECT id, stmt, code, pattern FROM on_conditions WHERE job_id = ?")?;
        let mut on_conditions = Vec::new();
        
        let rows = stmt.query_map(params![job_id], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, Option<String>>(1)?,
                row.get::<_, Option<String>>(2)?,
                row.get::<_, Option<String>>(3)?,
            ))
        })?;
        
        for row in rows {
            let (on_cond_id, stmt, code, pattern) = row?;
            let actions = self.get_do_actions(conn, on_cond_id)?;
            on_conditions.push(OnCondition {
                stmt,
                code,
                pattern,
                actions,
            });
        }
        
        Ok(on_conditions)
    }

    fn get_do_actions(&self, conn: &Connection, on_condition_id: i64) -> Result<Vec<DoAction>> {
        let mut stmt = conn.prepare("SELECT action_type, action_value, additional_data FROM do_actions WHERE on_condition_id = ?")?;
        let actions = stmt.query_map(params![on_condition_id], |row| {
            Ok(DoAction {
                action_type: row.get(0)?,
                action_value: row.get(1)?,
                additional_data: row.get(2)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        Ok(actions)
    }

    fn get_control_resources(&self, conn: &Connection, job_id: i64) -> Result<Vec<Resource>> {
        let mut stmt = conn.prepare("SELECT resource_name, resource_type, on_fail FROM control_resources WHERE job_id = ?")?;
        let resources = stmt.query_map(params![job_id], |row| {
            Ok(Resource {
                resource_name: row.get(0)?,
                resource_type: row.get(1)?,
                on_fail: row.get(2)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        Ok(resources)
    }

    fn get_quantitative_resources(&self, conn: &Connection, job_id: i64) -> Result<Vec<QuantitativeResource>> {
        let mut stmt = conn.prepare("SELECT resource_name, quantity, on_fail, on_ok FROM quantitative_resources WHERE job_id = ?")?;
        let resources = stmt.query_map(params![job_id], |row| {
            Ok(QuantitativeResource {
                resource_name: row.get(0)?,
                quantity: row.get(1)?,
                on_fail: row.get(2)?,
                on_ok: row.get(3)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        Ok(resources)
    }

    fn get_variables(&self, conn: &Connection, job_id: i64) -> Result<Vec<Variable>> {
        let mut stmt = conn.prepare("SELECT variable_name, variable_value FROM job_variables WHERE job_id = ?")?;
        let variables = stmt.query_map(params![job_id], |row| {
            Ok(Variable {
                name: row.get(0)?,
                value: row.get(1)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        Ok(variables)
    }

    fn get_auto_edits(&self, conn: &Connection, job_id: i64) -> Result<Vec<Variable>> {
        let mut stmt = conn.prepare("SELECT edit_name, edit_value FROM job_auto_edits WHERE job_id = ?")?;
        let edits = stmt.query_map(params![job_id], |row| {
            Ok(Variable {
                name: row.get(0)?,
                value: row.get(1)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        Ok(edits)
    }

    fn get_metadata(&self, conn: &Connection, job_id: i64) -> Result<Vec<Variable>> {
        let mut stmt = conn.prepare("SELECT meta_key, meta_value FROM job_metadata WHERE job_id = ?")?;
        let metadata = stmt.query_map(params![job_id], |row| {
            Ok(Variable {
                name: row.get(0)?,
                value: row.get(1)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        Ok(metadata)
    }

    pub fn get_dashboard_stats(&self) -> Result<DashboardStats> {
        let conn = self.conn.lock().unwrap();
        
        let total_jobs: u32 = conn.query_row("SELECT COUNT(*) FROM jobs", [], |row| row.get(0))?;
        let total_folders: u32 = conn.query_row("SELECT COUNT(DISTINCT folder_name) FROM jobs", [], |row| row.get(0))?;
        let critical_jobs: u32 = conn.query_row("SELECT COUNT(*) FROM jobs WHERE critical = 1", [], |row| row.get(0))?;
        let cyclic_jobs: u32 = conn.query_row("SELECT COUNT(*) FROM jobs WHERE cyclic = 1", [], |row| row.get(0))?;
        
        let file_transfer_jobs: u32 = conn.query_row(
            "SELECT COUNT(*) FROM jobs WHERE appl_type = 'FILE_TRANS' OR appl_type = 'FileWatch'",
            [],
            |row| row.get(0)
        )?;
        
        let cli_jobs: u32 = conn.query_row(
            "SELECT COUNT(*) FROM jobs WHERE task_type = 'Command' OR task_type = 'Script' OR cmdline IS NOT NULL",
            [],
            |row| row.get(0)
        )?;
        
        let mut stmt = conn.prepare("SELECT application, COUNT(*) as count FROM jobs WHERE application IS NOT NULL GROUP BY application ORDER BY count DESC LIMIT 10")?;
        let jobs_by_application = stmt.query_map([], |row| {
            Ok(ApplicationStat {
                application: row.get(0)?,
                count: row.get(1)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        
        let mut stmt = conn.prepare("SELECT folder_name, COUNT(*) as count FROM jobs GROUP BY folder_name ORDER BY count DESC LIMIT 10")?;
        let jobs_by_folder = stmt.query_map([], |row| {
            Ok(FolderStat {
                folder_name: row.get(0)?,
                job_count: row.get(1)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        
        let mut stmt = conn.prepare("SELECT COALESCE(task_type, 'Unknown'), COUNT(*) as count FROM jobs GROUP BY task_type ORDER BY count DESC")?;
        let jobs_by_task_type = stmt.query_map([], |row| {
            Ok(TaskTypeStat {
                task_type: row.get(0)?,
                count: row.get(1)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        
        let mut stmt = conn.prepare("SELECT COALESCE(appl_type, 'Unknown'), COUNT(*) as count FROM jobs GROUP BY appl_type ORDER BY count DESC")?;
        let jobs_by_appl_type = stmt.query_map([], |row| {
            Ok(ApplTypeStat {
                appl_type: row.get(0)?,
                count: row.get(1)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        
        Ok(DashboardStats {
            total_jobs,
            total_folders,
            critical_jobs,
            cyclic_jobs,
            file_transfer_jobs,
            cli_jobs,
            jobs_by_application,
            jobs_by_folder,
            jobs_by_task_type,
            jobs_by_appl_type,
        })
    }

    pub fn get_filter_options(&self) -> Result<FilterOptions> {
        let conn = self.conn.lock().unwrap();
        
        let mut stmt = conn.prepare("SELECT DISTINCT application FROM jobs WHERE application IS NOT NULL ORDER BY application")?;
        let applications = stmt.query_map([], |row| row.get(0))?.collect::<Result<Vec<_>, _>>()?;
        
        let mut stmt = conn.prepare("SELECT DISTINCT folder_name FROM jobs ORDER BY folder_name")?;
        let folders = stmt.query_map([], |row| row.get(0))?.collect::<Result<Vec<_>, _>>()?;
        
        let mut stmt = conn.prepare("SELECT DISTINCT task_type FROM jobs WHERE task_type IS NOT NULL ORDER BY task_type")?;
        let task_types = stmt.query_map([], |row| row.get(0))?.collect::<Result<Vec<_>, _>>()?;
        
        let appl_type_options: Vec<String> = conn.prepare(
            "SELECT DISTINCT appl_type FROM jobs WHERE appl_type IS NOT NULL AND appl_type != '' ORDER BY appl_type"
        )?
        .query_map([], |row| row.get(0))?
        .collect::<Result<Vec<_>, _>>()?;
        
        let appl_ver_options: Vec<String> = conn.prepare(
            "SELECT DISTINCT appl_ver FROM jobs WHERE appl_ver IS NOT NULL AND appl_ver != '' ORDER BY appl_ver"
        )?
        .query_map([], |row| row.get(0))?
        .collect::<Result<Vec<_>, _>>()?;
        
        Ok(FilterOptions {
            folders,
            applications,
            appl_types: appl_type_options,
            appl_vers: appl_ver_options,
            task_types,
        })
    }
    
    pub fn export_search_to_csv(&self, request: &JobSearchRequest) -> Result<String> {
        let conn = self.conn.lock().unwrap();
        
        let (where_clause, params_vec) = self.build_csv_where_clause(request);
        let query = self.build_csv_query(&where_clause);
        
        let rows = self.execute_csv_query(&conn, &query, &params_vec)?;
        let csv_output = self.format_csv_output(rows)?;
        
        Ok(csv_output)
    }
    
    fn build_csv_where_clause(&self, request: &JobSearchRequest) -> (String, Vec<Box<dyn rusqlite::ToSql>>) {
        let mut where_clauses = Vec::new();
        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
        
        if let Some(ref job_name) = request.job_name {
            where_clauses.push("job_name LIKE ?");
            params_vec.push(Box::new(format!("%{}%", job_name)));
        }
        
        if let Some(ref folder_name) = request.folder_name {
            where_clauses.push("folder_name LIKE ?");
            params_vec.push(Box::new(format!("%{}%", folder_name)));
        }
        
        if let Some(ref application) = request.application {
            where_clauses.push("application = ?");
            params_vec.push(Box::new(application.clone()));
        }
        
        if let Some(critical) = request.critical {
            where_clauses.push("critical = ?");
            params_vec.push(Box::new(if critical { 1 } else { 0 }));
        }
        
        if let Some(ref task_type) = request.task_type {
            where_clauses.push("task_type = ?");
            params_vec.push(Box::new(task_type.clone()));
        }
        
        if let Some(ref appl_type) = request.appl_type {
            where_clauses.push("appl_type = ?");
            params_vec.push(Box::new(appl_type.clone()));
        }
        
        if let Some(ref appl_ver) = request.appl_ver {
            where_clauses.push("appl_ver = ?");
            params_vec.push(Box::new(appl_ver.clone()));
        }
        
        let where_clause = if where_clauses.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", where_clauses.join(" AND "))
        };
        
        (where_clause, params_vec)
    }
    
    fn build_csv_query(&self, where_clause: &str) -> String {
        format!(
            r#"
            SELECT 
                j.job_name, j.folder_name, j.application, j.sub_application,
                COALESCE(j.appl_type, '') as appl_type, COALESCE(j.appl_ver, '') as appl_ver,
                j.task_type, j.critical, j.cyclic, j.owner, j.priority,
                j.description, j.cmdline
            FROM jobs j
            {}
            ORDER BY j.job_name
            "#,
            where_clause
        )
    }
    
    fn execute_csv_query(
        &self,
        conn: &rusqlite::Connection,
        query: &str,
        params_vec: &[Box<dyn rusqlite::ToSql>],
    ) -> Result<Vec<(String, Option<String>, Option<String>, Option<String>, String, String, Option<String>, i32, i32, Option<String>, Option<String>, Option<String>, Option<String>)>> {
        let mut stmt = conn.prepare(query)?;
        let rows = stmt.query_map(
            rusqlite::params_from_iter(params_vec.iter().map(|p| p.as_ref())),
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, Option<String>>(1)?,
                    row.get::<_, Option<String>>(2)?,
                    row.get::<_, Option<String>>(3)?,
                    row.get::<_, String>(4)?,
                    row.get::<_, String>(5)?,
                    row.get::<_, Option<String>>(6)?,
                    row.get::<_, i32>(7)?,
                    row.get::<_, i32>(8)?,
                    row.get::<_, Option<String>>(9)?,
                    row.get::<_, Option<String>>(10)?,
                    row.get::<_, Option<String>>(11)?,
                    row.get::<_, Option<String>>(12)?,
                ))
            },
        )?;
        
        rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.into())
    }
    
    fn format_csv_output(
        &self,
        rows: Vec<(String, Option<String>, Option<String>, Option<String>, String, String, Option<String>, i32, i32, Option<String>, Option<String>, Option<String>, Option<String>)>,
    ) -> Result<String> {
        let mut csv_output = self.get_csv_header();
        
        for row in rows {
            csv_output.push_str(&self.format_csv_row(row));
        }
        
        Ok(csv_output)
    }
    
    fn get_csv_header(&self) -> String {
        String::from("Job Name,Folder,Application,Sub Application,APPL_TYPE,APPL_VER,Task Type,Critical,Cyclic,Owner,Priority,Description,Command Line\n")
    }
    
    fn format_csv_row(
        &self,
        row: (String, Option<String>, Option<String>, Option<String>, String, String, Option<String>, i32, i32, Option<String>, Option<String>, Option<String>, Option<String>),
    ) -> String {
        let (job_name, folder, app, sub_app, appl_type, appl_ver, task_type, critical, cyclic, owner, priority, desc, cmdline) = row;
        
        format!(
            "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\n",
            job_name,
            folder.unwrap_or_default(),
            app.unwrap_or_default(),
            sub_app.unwrap_or_default(),
            appl_type,
            appl_ver,
            task_type.unwrap_or_default(),
            if critical == 1 { "Yes" } else { "No" },
            if cyclic == 1 { "Yes" } else { "No" },
            owner.unwrap_or_default(),
            priority.unwrap_or_default(),
            self.escape_csv_field(&desc.unwrap_or_default()),
            self.escape_csv_field(&cmdline.unwrap_or_default())
        )
    }
    
    fn escape_csv_field(&self, field: &str) -> String {
        field.replace("\"", "\"\"")
    }

    pub fn get_job_graph(&self, job_id: i64) -> Result<super::models::JobGraphData> {
        tracing::info!("📊 [GRAPH] Fetching dependency graph for job_id={}", job_id);
        let conn = self.conn.lock().unwrap();
        
        // Get the main job info
        tracing::debug!("[GRAPH] Querying main job info for job_id={}", job_id);
        let job = conn.query_row(
            "SELECT id, job_name, folder_name, application, description FROM jobs WHERE id = ?",
            [job_id],
            |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, Option<String>>(3)?,
                    row.get::<_, Option<String>>(4)?
                ))
            }
        ).map_err(|e| {
            tracing::error!("❌ [GRAPH] Failed to fetch job info for job_id={}: {}", job_id, e);
            e
        })?;
        
        tracing::info!("✅ [GRAPH] Found job: id={}, name='{}', folder='{}'", job.0, job.1, job.2);
        
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut visited_jobs = std::collections::HashSet::new();
        
        // Add current job as node
        nodes.push(super::models::GraphNode {
            id: job.0,
            label: job.1.clone(),
            folder: job.2.clone(),
            application: job.3.clone(),
            description: job.4.clone(),
            color: "#4CAF50".to_string(),
            is_current: true,
        });
        visited_jobs.insert(job.0);
        
        // Get incoming dependencies (jobs that this job depends on)
        // Query in_conditions to find what this job depends on
        tracing::debug!("[GRAPH] Querying incoming dependencies for job_id={}", job_id);
        let in_query = "SELECT DISTINCT condition_name FROM in_conditions WHERE job_id = ?";
        let mut in_stmt = conn.prepare(in_query)?;
        let condition_names: Vec<String> = in_stmt
            .query_map([job_id], |row| row.get(0))?
            .filter_map(|r| r.ok())
            .collect();
        
        tracing::info!("🔍 [GRAPH] Found {} condition names for incoming dependencies", condition_names.len());
        
        // Find jobs matching those condition names
        for cond_name in condition_names {
            tracing::debug!("[GRAPH] Looking for job matching condition_name='{}'", cond_name);
            
            // Try exact match first, then try stripping common suffixes
            let base_name = cond_name
                .trim_end_matches("-ENDED-OK")
                .trim_end_matches("-ENDED-NOTOK")
                .trim_end_matches("-ENDED")
                .trim_end_matches("-OK")
                .trim_end_matches("-NOTOK");
            
            let dep_job_result = conn.query_row(
                "SELECT id, job_name, folder_name, application, description FROM jobs WHERE job_name = ? OR job_name = ? LIMIT 1",
                [&cond_name, base_name],
                |row| {
                    Ok((
                        row.get::<_, i64>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, Option<String>>(3)?,
                        row.get::<_, Option<String>>(4)?
                    ))
                }
            );
            
            if let Ok(dep_job) = dep_job_result {
                tracing::debug!("✅ [GRAPH] Matched incoming dep: id={}, name='{}' (from condition '{}')", 
                               dep_job.0, dep_job.1, cond_name);
                if !visited_jobs.contains(&dep_job.0) {
                    nodes.push(super::models::GraphNode {
                        id: dep_job.0,
                        label: dep_job.1.clone(),
                        folder: dep_job.2.clone(),
                        application: dep_job.3.clone(),
                        description: dep_job.4.clone(),
                        color: "#2196F3".to_string(),
                        is_current: false,
                    });
                    visited_jobs.insert(dep_job.0);
                    tracing::debug!("[GRAPH] Added incoming dep node: {}", dep_job.1);
                }
                edges.push(super::models::GraphEdge {
                    from: dep_job.0,
                    to: job.0,
                    edge_type: "in".to_string(),
                });
                tracing::debug!("[GRAPH] Added edge: {} -> {}", dep_job.1, job.1);
            } else {
                tracing::warn!("⚠️  [GRAPH] No job found matching condition_name='{}' or base_name='{}'", cond_name, base_name);
            }
        }
        
        // Get outgoing dependencies (jobs that depend on this job)
        // Find jobs that have this job's name in their in_conditions
        tracing::debug!("[GRAPH] Querying outgoing dependencies for job_name='{}'", job.1);
        let out_query = "SELECT DISTINCT job_id FROM in_conditions WHERE condition_name = ?";
        let mut out_stmt = conn.prepare(out_query).map_err(|e| {
            tracing::error!("❌ [GRAPH] Failed to prepare outgoing deps query: {}", e);
            e
        })?;
        let dependent_job_ids: Vec<i64> = out_stmt
            .query_map([&job.1], |row| row.get(0))?
            .filter_map(|r| r.ok())
            .collect();
        
        tracing::info!("🔍 [GRAPH] Found {} jobs depending on '{}'", dependent_job_ids.len(), job.1);
        
        for dep_job_id in dependent_job_ids {
            tracing::debug!("[GRAPH] Processing outgoing dep job_id={}", dep_job_id);
            if let Ok(dep_job) = conn.query_row(
                "SELECT id, job_name, folder_name, application, description FROM jobs WHERE id = ?",
                [dep_job_id],
                |row| {
                    Ok((
                        row.get::<_, i64>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, Option<String>>(3)?,
                        row.get::<_, Option<String>>(4)?
                    ))
                }
            ) {
                tracing::debug!("✅ [GRAPH] Matched outgoing dep: id={}, name='{}'", dep_job.0, dep_job.1);
                if !visited_jobs.contains(&dep_job.0) {
                    nodes.push(super::models::GraphNode {
                        id: dep_job.0,
                        label: dep_job.1.clone(),
                        folder: dep_job.2.clone(),
                        application: dep_job.3.clone(),
                        description: dep_job.4.clone(),
                        color: "#FF9800".to_string(),
                        is_current: false,
                    });
                    visited_jobs.insert(dep_job.0);
                    tracing::debug!("[GRAPH] Added outgoing dep node: {}", dep_job.1);
                }
                edges.push(super::models::GraphEdge {
                    from: job.0,
                    to: dep_job.0,
                    edge_type: "out".to_string(),
                });
                tracing::debug!("[GRAPH] Added edge: {} -> {}", job.1, dep_job.1);
            } else {
                tracing::warn!("⚠️  [GRAPH] Failed to fetch job details for job_id={}", dep_job_id);
            }
        }
        
        tracing::info!("✅ [GRAPH] Graph complete: {} nodes, {} edges", nodes.len(), edges.len());
        tracing::debug!("[GRAPH] Nodes: {:?}", nodes.iter().map(|n| &n.label).collect::<Vec<_>>());
        
        Ok(super::models::JobGraphData {
            job_id: job.0,
            job_name: job.1,
            folder_name: job.2,
            nodes,
            edges,
        })
    }
}

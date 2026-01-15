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
        tracing::debug!("🔨 [WHERE] Building WHERE clause for search");
        let mut where_clauses = Vec::new();
        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
        
        // Apply all filter categories
        self.apply_basic_filters(&mut where_clauses, &mut params_vec, request);
        self.apply_folder_filters(&mut where_clauses, &mut params_vec, request);
        self.apply_critical_filter(&mut where_clauses, &mut params_vec, request);
        self.apply_dependency_filters(&mut where_clauses, &mut params_vec, request);
        self.apply_odate_filter(&mut where_clauses, request);
        self.apply_variable_filters(&mut where_clauses, &mut params_vec, request);
        
        self.format_where_clause(where_clauses, params_vec)
    }
    
    fn apply_basic_filters(
        &self,
        where_clauses: &mut Vec<String>,
        params_vec: &mut Vec<Box<dyn rusqlite::ToSql>>,
        request: &JobSearchRequest
    ) {
        self.add_string_filter_owned(where_clauses, params_vec, &request.job_name, "j.job_name LIKE ?", "LIKE", "job_name");
        self.add_string_filter_owned(where_clauses, params_vec, &request.folder_name, "j.folder_name LIKE ?", "LIKE", "folder_name");
        self.add_string_filter_owned(where_clauses, params_vec, &request.application, "j.application = ?", "=", "application");
        self.add_string_filter_owned(where_clauses, params_vec, &request.task_type, "j.task_type = ?", "=", "task_type");
        self.add_string_filter_owned(where_clauses, params_vec, &request.appl_type, "j.appl_type = ?", "=", "appl_type");
        self.add_string_filter_owned(where_clauses, params_vec, &request.appl_ver, "j.appl_ver = ?", "=", "appl_ver");
    }
    
    fn apply_folder_filters(
        &self,
        where_clauses: &mut Vec<String>,
        params_vec: &mut Vec<Box<dyn rusqlite::ToSql>>,
        request: &JobSearchRequest
    ) {
        // Datacenter filter
        self.add_string_filter_owned(where_clauses, params_vec, &request.datacenter, 
            "EXISTS (SELECT 1 FROM folders f WHERE f.folder_name = j.folder_name AND f.datacenter = ?)", "=", "datacenter");
        
        // Folder order method with special "(Empty)" handling
        if let Some(ref folder_order_method) = request.folder_order_method {
            if folder_order_method == "(Empty)" {
                tracing::debug!("  ➕ Adding folder_order_method filter: (Empty) - searching for NULL or empty");
                where_clauses.push("EXISTS (SELECT 1 FROM folders f WHERE f.folder_name = j.folder_name AND (f.folder_order_method IS NULL OR f.folder_order_method = ''))".to_string());
            } else {
                tracing::debug!("  ➕ Adding folder_order_method filter: {}", folder_order_method);
                where_clauses.push("EXISTS (SELECT 1 FROM folders f WHERE f.folder_name = j.folder_name AND f.folder_order_method = ?)".to_string());
                params_vec.push(Box::new(folder_order_method.clone()));
            }
        }
    }
    
    fn apply_critical_filter(
        &self,
        where_clauses: &mut Vec<String>,
        params_vec: &mut Vec<Box<dyn rusqlite::ToSql>>,
        request: &JobSearchRequest
    ) {
        if let Some(critical) = request.critical {
            tracing::debug!("  ➕ Adding critical filter: {}", critical);
            where_clauses.push("j.critical = ?".to_string());
            params_vec.push(Box::new(if critical { 1 } else { 0 }));
        }
    }
    
    fn apply_dependency_filters(
        &self,
        where_clauses: &mut Vec<String>,
        params_vec: &mut Vec<Box<dyn rusqlite::ToSql>>,
        request: &JobSearchRequest
    ) {
        // In conditions (dependencies)
        self.add_count_filter_owned(where_clauses, params_vec, request.min_dependencies,
            "(SELECT COUNT(*) FROM in_conditions WHERE in_conditions.job_id = j.id) >= ?", ">=", "min_dependencies", false);
        self.add_count_filter_owned(where_clauses, params_vec, request.max_dependencies,
            "(SELECT COUNT(*) FROM in_conditions WHERE in_conditions.job_id = j.id) <= ?", "<=", "max_dependencies", false);
        
        // ON conditions
        self.add_count_filter_owned(where_clauses, params_vec, request.min_on_conditions,
            "(SELECT COUNT(*) FROM on_conditions WHERE on_conditions.job_id = j.id) >= ?", ">=", "min_on_conditions", true);
        self.add_count_filter_owned(where_clauses, params_vec, request.max_on_conditions,
            "(SELECT COUNT(*) FROM on_conditions WHERE on_conditions.job_id = j.id) <= ?", "<=", "max_on_conditions", true);
    }
    
    fn apply_odate_filter(
        &self,
        where_clauses: &mut Vec<String>,
        request: &JobSearchRequest
    ) {
        if let Some(has_odate) = request.has_odate {
            tracing::debug!("  ➕ Adding has_odate filter: {}", has_odate);
            let clause = if has_odate {
                "(EXISTS (SELECT 1 FROM in_conditions ic WHERE ic.job_id = j.id AND ic.odate IS NOT NULL AND ic.odate != '') OR EXISTS (SELECT 1 FROM out_conditions oc WHERE oc.job_id = j.id AND oc.odate IS NOT NULL AND oc.odate != ''))".to_string()
            } else {
                "(NOT EXISTS (SELECT 1 FROM in_conditions ic WHERE ic.job_id = j.id AND ic.odate IS NOT NULL AND ic.odate != '') AND NOT EXISTS (SELECT 1 FROM out_conditions oc WHERE oc.job_id = j.id AND oc.odate IS NOT NULL AND oc.odate != ''))".to_string()
            };
            where_clauses.push(clause);
        }
    }
    
    fn apply_variable_filters(
        &self,
        where_clauses: &mut Vec<String>,
        params_vec: &mut Vec<Box<dyn rusqlite::ToSql>>,
        request: &JobSearchRequest
    ) {
        // Has variables boolean filter
        if let Some(has_vars) = request.has_variables {
            tracing::debug!("  ➕ Adding has_variables filter: {}", has_vars);
            let clause = if has_vars {
                "(SELECT COUNT(*) FROM job_variables WHERE job_id = j.id) > 0".to_string()
            } else {
                "(SELECT COUNT(*) FROM job_variables WHERE job_id = j.id) = 0".to_string()
            };
            where_clauses.push(clause);
        }
        
        // Minimum variables count
        self.add_count_filter_owned(where_clauses, params_vec, request.min_variables,
            "(SELECT COUNT(*) FROM job_variables WHERE job_id = j.id) >= ?", ">=", "min_variables", false);
    }
    
    fn format_where_clause(
        &self,
        where_clauses: Vec<String>,
        params_vec: Vec<Box<dyn rusqlite::ToSql>>
    ) -> (String, Vec<Box<dyn rusqlite::ToSql>>) {
        let where_clause = if where_clauses.is_empty() {
            tracing::info!("🔨 [WHERE] No filters applied - returning all jobs");
            String::new()
        } else {
            let clause = format!("WHERE {}", where_clauses.join(" AND "));
            tracing::info!("🔨 [WHERE] Built clause with {} conditions: {}", where_clauses.len(), clause);
            clause
        };
        
        (where_clause, params_vec)
    }
    
    fn add_string_filter_owned(
        &self,
        clauses: &mut Vec<String>,
        params: &mut Vec<Box<dyn rusqlite::ToSql>>,
        value: &Option<String>,
        column: &str,
        operator: &str,
        filter_name: &str,
    ) {
        if let Some(ref val) = value {
            tracing::debug!("  ➕ Adding {} filter: {}", filter_name, val);
            if operator == "LIKE" {
                clauses.push(column.to_string());
                params.push(Box::new(format!("%{}%", val)));
            } else {
                clauses.push(column.to_string());
                params.push(Box::new(val.clone()));
            }
        }
    }
    
    fn add_count_filter_owned(
        &self,
        clauses: &mut Vec<String>,
        params: &mut Vec<Box<dyn rusqlite::ToSql>>,
        value: Option<i32>,
        table: &str,
        operator: &str,
        filter_name: &str,
        use_info_log: bool,
    ) {
        if let Some(val) = value {
            if use_info_log {
                tracing::info!("  ➕ Adding {} filter: {} {}", filter_name, operator, val);
            } else {
                tracing::debug!("  ➕ Adding {} filter: {} {}", filter_name, operator, val);
            }
            clauses.push(table.to_string());
            params.push(Box::new(val));
        }
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
        tracing::info!("🔢 [COUNT] Executing count query: {}", count_query);
        tracing::debug!("🔢 [COUNT] With {} parameters", params_vec.len());
        
        let start = std::time::Instant::now();
        let total: u32 = conn.query_row(
            &count_query,
            rusqlite::params_from_iter(params_vec.iter().map(|p| p.as_ref())),
            |row| row.get(0),
        ).map_err(|e| {
            tracing::error!("❌ [COUNT] Query failed: {}", e);
            e
        })?;
        
        let duration = start.elapsed();
        tracing::info!("✅ [COUNT] Query completed in {:?}, found {} jobs", duration, total);
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
                j.id, j.job_name, j.folder_name,
                f.datacenter, f.folder_order_method,
                j.application, j.sub_application,
                COALESCE(j.appl_type, '') as appl_type, COALESCE(j.appl_ver, '') as appl_ver,
                j.description, j.owner, j.run_as, j.priority, j.critical,
                j.task_type, j.cyclic, j.node_id, j.cmdline,
                j.jobisn, j.job_group, j.memname, j.author,
                j.doclib, j.docmem, j.memlib, j.overlib, j.override_path,
                j.job_interval, j.confirm, j.retro, j.autoarch, j.rerunmem, j.category,
                j.pdsname, j.minimum, j.preventnct2, j.option_field, j.from_field, j.par,
                j.sysdb, j.due_out, j.reten_days, j.reten_gen, j.task_class, j.prev_day,
                j.adjust_cond, j.jobs_in_group, j.large_size, j.ind_cyclic,
                j.maxwait, j.maxrerun, j.maxdays, j.maxruns,
                j.shift, j.shiftnum,
                j.days, j.weekdays, j.jan, j.feb, j.mar, j.apr, j.may, j.jun,
                j.jul, j.aug, j.sep, j.oct, j.nov, j.dec, j.date, j.days_and_or,
                j.cyclic_interval_sequence, j.cyclic_times_sequence, j.cyclic_tolerance, j.cyclic_type,
                j.created_by, j.creation_date, j.creation_user, j.creation_time,
                j.change_userid, j.change_date, j.change_time,
                j.job_version, j.version_opcode, j.is_current_version, j.version_serial, j.version_host,
                j.rule_based_calendar_relationship, j.tag_relationship, j.timezone,
                j.appl_form, j.cm_ver, j.multy_agent, j.active_from, j.active_till,
                j.scheduling_environment, j.system_affinity, j.request_nje_node,
                j.stat_cal, j.instream_jcl, j.use_instream_jcl,
                j.due_out_daysoffset, j.from_daysoffset, j.to_daysoffset,
                j.parent_folder, j.parent_table, j.end_folder, j.odate,
                j.fprocs, j.tpgms, j.tprocs,
                (SELECT COUNT(*) FROM in_conditions WHERE job_id = j.id) as in_cond_count,
                (SELECT COUNT(*) FROM out_conditions WHERE job_id = j.id) as out_cond_count,
                (SELECT COUNT(*) FROM on_conditions WHERE job_id = j.id) as on_cond_count,
                (SELECT COUNT(*) FROM control_resources WHERE job_id = j.id) as ctrl_res_count,
                (SELECT COUNT(*) FROM job_variables WHERE job_id = j.id) as var_count
            FROM jobs j
            LEFT JOIN folders f ON j.folder_name = f.folder_name
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
        let appl_type: String = row.get(7)?;
        let appl_ver: String = row.get(8)?;
        
        Ok(JobDetail {
            id: row.get(0)?,
            job_name: row.get(1)?,
            folder_name: row.get(2)?,
            datacenter: row.get(3)?,
            folder_order_method: row.get(4)?,
            application: row.get(5)?,
            sub_application: row.get(6)?,
            appl_type: if appl_type.is_empty() { None } else { Some(appl_type) },
            appl_ver: if appl_ver.is_empty() { None } else { Some(appl_ver) },
            description: row.get(9)?,
            owner: row.get(10)?,
            run_as: row.get(11)?,
            priority: row.get(12)?,
            critical: row.get::<_, i32>(13)? == 1,
            task_type: row.get(14)?,
            cyclic: row.get::<_, i32>(15)? == 1,
            node_id: row.get(16)?,
            cmdline: row.get(17)?,
            jobisn: row.get(18)?,
            group: row.get(19)?,
            memname: row.get(20)?,
            author: row.get(21)?,
            doclib: row.get(22)?,
            docmem: row.get(23)?,
            memlib: row.get(24)?,
            overlib: row.get(25)?,
            override_path: row.get(26)?,
            interval: row.get(27)?,
            confirm: row.get(28)?,
            retro: row.get(29)?,
            autoarch: row.get(30)?,
            rerunmem: row.get(31)?,
            category: row.get(32)?,
            pdsname: row.get(33)?,
            minimum: row.get(34)?,
            preventnct2: row.get(35)?,
            option_field: row.get(36)?,
            from_field: row.get(37)?,
            par: row.get(38)?,
            sysdb: row.get(39)?,
            due_out: row.get(40)?,
            reten_days: row.get(41)?,
            reten_gen: row.get(42)?,
            task_class: row.get(43)?,
            prev_day: row.get(44)?,
            adjust_cond: row.get(45)?,
            jobs_in_group: row.get(46)?,
            large_size: row.get(47)?,
            ind_cyclic: row.get(48)?,
            maxwait: row.get(49)?,
            maxrerun: row.get(50)?,
            maxdays: row.get(51)?,
            maxruns: row.get(52)?,
            shift: row.get(53)?,
            shiftnum: row.get(54)?,
            days: row.get(55)?,
            weekdays: row.get(56)?,
            jan: row.get(57)?,
            feb: row.get(58)?,
            mar: row.get(59)?,
            apr: row.get(60)?,
            may: row.get(61)?,
            jun: row.get(62)?,
            jul: row.get(63)?,
            aug: row.get(64)?,
            sep: row.get(65)?,
            oct: row.get(66)?,
            nov: row.get(67)?,
            dec: row.get(68)?,
            date: row.get(69)?,
            days_and_or: row.get(70)?,
            cyclic_interval_sequence: row.get(71)?,
            cyclic_times_sequence: row.get(72)?,
            cyclic_tolerance: row.get(73)?,
            cyclic_type: row.get(74)?,
            created_by: row.get(75)?,
            creation_date: row.get(76)?,
            creation_user: row.get(77)?,
            creation_time: row.get(78)?,
            change_userid: row.get(79)?,
            change_date: row.get(80)?,
            change_time: row.get(81)?,
            job_version: row.get(82)?,
            version_opcode: row.get(83)?,
            is_current_version: row.get(84)?,
            version_serial: row.get(85)?,
            version_host: row.get(86)?,
            rule_based_calendar_relationship: row.get(87)?,
            tag_relationship: row.get(88)?,
            timezone: row.get(89)?,
            appl_form: row.get(90)?,
            cm_ver: row.get(91)?,
            multy_agent: row.get(92)?,
            active_from: row.get(93)?,
            active_till: row.get(94)?,
            scheduling_environment: row.get(95)?,
            system_affinity: row.get(96)?,
            request_nje_node: row.get(97)?,
            stat_cal: row.get(98)?,
            instream_jcl: row.get(99)?,
            use_instream_jcl: row.get(100)?,
            due_out_daysoffset: row.get(101)?,
            from_daysoffset: row.get(102)?,
            to_daysoffset: row.get(103)?,
            parent_folder: row.get(104)?,
            parent_table: row.get(105)?,
            end_folder: row.get(106)?,
            odate: row.get(107)?,
            fprocs: row.get(108)?,
            tpgms: row.get(109)?,
            tprocs: row.get(110)?,
            in_conditions_count: row.get(111)?,
            out_conditions_count: row.get(112)?,
            on_conditions_count: row.get(113)?,
            control_resources_count: row.get(114)?,
            variables_count: row.get(115)?,
        })
    }

    pub fn get_job_detail(&self, job_id: i64) -> Result<Option<JobDetailFull>> {
        let conn = self.conn.lock().unwrap();
        
        let job: Option<JobDetail> = conn.query_row(
            r#"
            SELECT 
                j.id, j.job_name, j.folder_name,
                f.datacenter, f.folder_order_method,
                j.application, j.sub_application,
                COALESCE(j.appl_type, '') as appl_type, COALESCE(j.appl_ver, '') as appl_ver,
                j.description, j.owner, j.run_as, j.priority, j.critical,
                j.task_type, j.cyclic, j.node_id, j.cmdline,
                j.jobisn, j.job_group, j.memname, j.author,
                j.doclib, j.docmem, j.memlib, j.overlib, j.override_path,
                j.job_interval, j.confirm, j.retro, j.autoarch, j.rerunmem, j.category,
                j.pdsname, j.minimum, j.preventnct2, j.option_field, j.from_field, j.par,
                j.sysdb, j.due_out, j.reten_days, j.reten_gen, j.task_class, j.prev_day,
                j.adjust_cond, j.jobs_in_group, j.large_size, j.ind_cyclic,
                j.maxwait, j.maxrerun, j.maxdays, j.maxruns,
                j.shift, j.shiftnum,
                j.days, j.weekdays, j.jan, j.feb, j.mar, j.apr, j.may, j.jun,
                j.jul, j.aug, j.sep, j.oct, j.nov, j.dec, j.date, j.days_and_or,
                j.cyclic_interval_sequence, j.cyclic_times_sequence, j.cyclic_tolerance, j.cyclic_type,
                j.created_by, j.creation_date, j.creation_user, j.creation_time,
                j.change_userid, j.change_date, j.change_time,
                j.job_version, j.version_opcode, j.is_current_version, j.version_serial, j.version_host,
                j.rule_based_calendar_relationship, j.tag_relationship, j.timezone,
                j.appl_form, j.cm_ver, j.multy_agent, j.active_from, j.active_till,
                j.scheduling_environment, j.system_affinity, j.request_nje_node,
                j.stat_cal, j.instream_jcl, j.use_instream_jcl,
                j.due_out_daysoffset, j.from_daysoffset, j.to_daysoffset,
                j.parent_folder, j.parent_table, j.end_folder, j.odate,
                j.fprocs, j.tpgms, j.tprocs,
                (SELECT COUNT(*) FROM in_conditions WHERE job_id = j.id),
                (SELECT COUNT(*) FROM out_conditions WHERE job_id = j.id),
                (SELECT COUNT(*) FROM on_conditions WHERE job_id = j.id),
                (SELECT COUNT(*) FROM control_resources WHERE job_id = j.id),
                (SELECT COUNT(*) FROM job_variables WHERE job_id = j.id)
            FROM jobs j
            LEFT JOIN folders f ON j.folder_name = f.folder_name
            WHERE j.id = ?
            "#,
            params![job_id],
            Self::map_row_to_job_detail,
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
        
        let datacenters: Vec<String> = conn.prepare(
            "SELECT DISTINCT datacenter FROM folders WHERE datacenter IS NOT NULL AND datacenter != '' ORDER BY datacenter"
        )?
        .query_map([], |row| row.get(0))?
        .collect::<Result<Vec<_>, _>>()?;
        
        let mut folder_order_methods: Vec<String> = conn.prepare(
            "SELECT DISTINCT folder_order_method FROM folders WHERE folder_order_method IS NOT NULL AND folder_order_method != '' ORDER BY folder_order_method"
        )?
        .query_map([], |row| row.get(0))?
        .collect::<Result<Vec<_>, _>>()?;
        
        // Add special "(Empty)" option for folders without folder_order_method
        let empty_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM folders WHERE folder_order_method IS NULL OR folder_order_method = ''",
            [],
            |row| row.get(0)
        )?;
        
        if empty_count > 0 {
            folder_order_methods.insert(0, "(Empty)".to_string());
        }
        
        Ok(FilterOptions {
            folders,
            applications,
            appl_types: appl_type_options,
            appl_vers: appl_ver_options,
            task_types,
            datacenters,
            folder_order_methods,
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

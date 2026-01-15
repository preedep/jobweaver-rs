-- Complete Control-M Database Schema with All Attributes
-- This schema includes ALL attributes from Control-M XML Schema

-- Drop existing tables if recreating
-- DROP TABLE IF EXISTS job_metadata;
-- DROP TABLE IF EXISTS job_auto_edits;
-- DROP TABLE IF EXISTS job_variables;
-- DROP TABLE IF EXISTS quantitative_resources;
-- DROP TABLE IF EXISTS control_resources;
-- DROP TABLE IF EXISTS do_actions;
-- DROP TABLE IF EXISTS on_conditions;
-- DROP TABLE IF EXISTS out_conditions;
-- DROP TABLE IF EXISTS in_conditions;
-- DROP TABLE IF EXISTS job_scheduling;
-- DROP TABLE IF EXISTS jobs;
-- DROP TABLE IF EXISTS folders;

-- ============================================================================
-- FOLDERS TABLE - Complete with all Control-M folder attributes
-- ============================================================================
CREATE TABLE IF NOT EXISTS folders (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    
    -- Core folder attributes
    folder_name TEXT NOT NULL,
    folder_type TEXT NOT NULL,
    datacenter TEXT,
    application TEXT,
    description TEXT,
    owner TEXT,
    
    -- Additional folder metadata from Control-M XML
    version TEXT,
    platform TEXT,
    table_name TEXT,
    folder_dsn TEXT,
    table_dsn TEXT,
    modified INTEGER,  -- boolean: 0 or 1
    last_upload TEXT,
    folder_order_method TEXT,  -- ⭐ Important for job ordering
    table_userdaily TEXT,
    real_folder_id INTEGER,
    real_tableid INTEGER,
    type_code INTEGER,
    used_by TEXT,
    used_by_code INTEGER,
    enforce_validation TEXT,
    site_standard_name TEXT,
    
    -- System metadata
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    UNIQUE(folder_name, datacenter)
);

-- ============================================================================
-- JOBS TABLE - Complete with all Control-M job attributes
-- ============================================================================
CREATE TABLE IF NOT EXISTS jobs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    
    -- Core job attributes
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
    
    -- Additional job metadata from Control-M XML
    jobisn INTEGER,  -- ⭐ Job ISN (unique identifier)
    job_group TEXT,  -- ⭐ Job group
    memname TEXT,
    author TEXT,
    doclib TEXT,
    docmem TEXT,
    job_interval TEXT,
    override_path TEXT,
    overlib TEXT,
    memlib TEXT,
    confirm TEXT,
    retro TEXT,
    maxwait INTEGER,
    maxrerun INTEGER,
    autoarch TEXT,
    maxdays INTEGER,
    maxruns INTEGER,
    days TEXT,
    weekdays TEXT,
    
    -- Monthly scheduling (12 columns)
    jan TEXT,
    feb TEXT,
    mar TEXT,
    apr TEXT,
    may TEXT,
    jun TEXT,
    jul TEXT,
    aug TEXT,
    sep TEXT,
    oct TEXT,
    nov TEXT,
    dec TEXT,
    
    date TEXT,
    rerunmem TEXT,
    days_and_or TEXT,
    category TEXT,
    shift TEXT,
    shiftnum TEXT,
    pdsname TEXT,
    minimum TEXT,
    preventnct2 TEXT,
    option_field TEXT,  -- 'option' is SQL keyword
    from_field TEXT,    -- 'from' is SQL keyword
    par TEXT,
    sysdb TEXT,
    due_out TEXT,
    reten_days TEXT,
    reten_gen TEXT,
    task_class TEXT,
    prev_day TEXT,
    adjust_cond TEXT,
    jobs_in_group TEXT,
    large_size TEXT,
    ind_cyclic TEXT,
    
    -- Audit fields
    creation_user TEXT,
    creation_time TEXT,
    created_by TEXT,
    creation_date TEXT,
    change_userid TEXT,
    change_date TEXT,
    change_time TEXT,
    
    -- Version control
    job_version TEXT,  -- ⭐ Job version
    version_opcode TEXT,
    is_current_version TEXT,
    version_serial INTEGER,
    version_host TEXT,
    
    -- Advanced features
    rule_based_calendar_relationship TEXT,
    tag_relationship TEXT,
    timezone TEXT,  -- ⭐ Timezone
    appl_form TEXT,
    cm_ver TEXT,
    multy_agent TEXT,
    active_from TEXT,  -- ⭐ Active from date
    active_till TEXT,  -- ⭐ Active till date
    scheduling_environment TEXT,  -- ⭐ Scheduling environment
    system_affinity TEXT,
    request_nje_node TEXT,
    stat_cal TEXT,
    instream_jcl TEXT,
    use_instream_jcl TEXT,
    due_out_daysoffset TEXT,
    from_daysoffset TEXT,
    to_daysoffset TEXT,
    
    -- Cyclic job attributes
    cyclic_interval_sequence TEXT,
    cyclic_times_sequence TEXT,
    cyclic_tolerance INTEGER,
    cyclic_type TEXT,
    
    -- Hierarchy
    parent_folder TEXT,  -- ⭐ Parent folder
    parent_table TEXT,   -- ⭐ Parent table
    end_folder TEXT,
    odate TEXT,
    fprocs TEXT,
    tpgms TEXT,
    tprocs TEXT,
    
    -- System metadata
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    UNIQUE(job_name, folder_name)
);

-- ============================================================================
-- JOB SCHEDULING TABLE - Complete scheduling information
-- ============================================================================
CREATE TABLE IF NOT EXISTS job_scheduling (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    job_id INTEGER NOT NULL,
    
    -- Time windows
    time_from TEXT,
    time_to TEXT,
    
    -- Calendars
    days_calendar TEXT,
    weeks_calendar TEXT,
    conf_calendar TEXT,
    stat_cal TEXT,
    
    -- Cyclic execution
    cyclic_interval TEXT,
    cyclic_times TEXT,
    
    -- Limits
    max_wait INTEGER,
    max_rerun INTEGER,
    maxdays INTEGER,
    maxruns INTEGER,
    
    -- Scheduling constraints
    days TEXT,
    weekdays TEXT,
    date TEXT,
    days_and_or TEXT,
    shift TEXT,
    shift_num TEXT,
    retro TEXT,
    autoarch TEXT,
    confirm TEXT,
    
    -- Timezone and active period
    timezone TEXT,
    active_from TEXT,
    active_till TEXT,
    
    -- Due out
    due_out TEXT,
    due_out_daysoffset TEXT,
    from_daysoffset TEXT,
    to_daysoffset TEXT,
    
    -- Other
    prev_day TEXT,
    adjust_cond TEXT,
    
    FOREIGN KEY (job_id) REFERENCES jobs(id) ON DELETE CASCADE
);

-- ============================================================================
-- IN CONDITIONS TABLE
-- ============================================================================
CREATE TABLE IF NOT EXISTS in_conditions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    job_id INTEGER NOT NULL,
    condition_name TEXT NOT NULL,
    odate TEXT,
    and_or TEXT,
    op TEXT,
    FOREIGN KEY (job_id) REFERENCES jobs(id) ON DELETE CASCADE
);

-- ============================================================================
-- OUT CONDITIONS TABLE
-- ============================================================================
CREATE TABLE IF NOT EXISTS out_conditions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    job_id INTEGER NOT NULL,
    condition_name TEXT NOT NULL,
    odate TEXT,
    sign TEXT,
    FOREIGN KEY (job_id) REFERENCES jobs(id) ON DELETE CASCADE
);

-- ============================================================================
-- ON CONDITIONS TABLE
-- ============================================================================
CREATE TABLE IF NOT EXISTS on_conditions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    job_id INTEGER NOT NULL,
    stmt TEXT,
    code TEXT,
    pattern TEXT,
    pgms TEXT,
    procs TEXT,
    and_or TEXT,
    from_column TEXT,
    to_column TEXT,
    FOREIGN KEY (job_id) REFERENCES jobs(id) ON DELETE CASCADE
);

-- ============================================================================
-- DO ACTIONS TABLE (for on conditions)
-- ============================================================================
CREATE TABLE IF NOT EXISTS do_actions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    on_condition_id INTEGER NOT NULL,
    action_type TEXT NOT NULL,
    action_value TEXT,
    additional_data TEXT,
    FOREIGN KEY (on_condition_id) REFERENCES on_conditions(id) ON DELETE CASCADE
);

-- ============================================================================
-- CONTROL RESOURCES TABLE
-- ============================================================================
CREATE TABLE IF NOT EXISTS control_resources (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    job_id INTEGER NOT NULL,
    resource_name TEXT NOT NULL,
    resource_type TEXT,
    on_fail TEXT,
    FOREIGN KEY (job_id) REFERENCES jobs(id) ON DELETE CASCADE
);

-- ============================================================================
-- QUANTITATIVE RESOURCES TABLE
-- ============================================================================
CREATE TABLE IF NOT EXISTS quantitative_resources (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    job_id INTEGER NOT NULL,
    resource_name TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    on_fail TEXT,
    on_ok TEXT,
    FOREIGN KEY (job_id) REFERENCES jobs(id) ON DELETE CASCADE
);

-- ============================================================================
-- JOB VARIABLES TABLE
-- ============================================================================
CREATE TABLE IF NOT EXISTS job_variables (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    job_id INTEGER NOT NULL,
    variable_name TEXT NOT NULL,
    variable_value TEXT,
    FOREIGN KEY (job_id) REFERENCES jobs(id) ON DELETE CASCADE
);

-- ============================================================================
-- JOB AUTO EDITS TABLE
-- ============================================================================
CREATE TABLE IF NOT EXISTS job_auto_edits (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    job_id INTEGER NOT NULL,
    edit_name TEXT NOT NULL,
    edit_value TEXT,
    FOREIGN KEY (job_id) REFERENCES jobs(id) ON DELETE CASCADE
);

-- ============================================================================
-- JOB METADATA TABLE (for additional key-value metadata)
-- ============================================================================
CREATE TABLE IF NOT EXISTS job_metadata (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    job_id INTEGER NOT NULL,
    meta_key TEXT NOT NULL,
    meta_value TEXT,
    FOREIGN KEY (job_id) REFERENCES jobs(id) ON DELETE CASCADE
);

-- ============================================================================
-- INDEXES FOR PERFORMANCE
-- ============================================================================

-- Folder indexes
CREATE INDEX IF NOT EXISTS idx_folders_datacenter ON folders(datacenter);
CREATE INDEX IF NOT EXISTS idx_folders_application ON folders(application);
CREATE INDEX IF NOT EXISTS idx_folders_type ON folders(folder_type);
CREATE INDEX IF NOT EXISTS idx_folders_order_method ON folders(folder_order_method);

-- Job indexes - Single column
CREATE INDEX IF NOT EXISTS idx_jobs_folder ON jobs(folder_name);
CREATE INDEX IF NOT EXISTS idx_jobs_application ON jobs(application);
CREATE INDEX IF NOT EXISTS idx_jobs_critical ON jobs(critical);
CREATE INDEX IF NOT EXISTS idx_jobs_appl_type ON jobs(appl_type);
CREATE INDEX IF NOT EXISTS idx_jobs_appl_ver ON jobs(appl_ver);
CREATE INDEX IF NOT EXISTS idx_jobs_task_type ON jobs(task_type);
CREATE INDEX IF NOT EXISTS idx_jobs_owner ON jobs(owner);
CREATE INDEX IF NOT EXISTS idx_jobs_jobisn ON jobs(jobisn);
CREATE INDEX IF NOT EXISTS idx_jobs_group ON jobs(job_group);
CREATE INDEX IF NOT EXISTS idx_jobs_timezone ON jobs(timezone);
CREATE INDEX IF NOT EXISTS idx_jobs_parent_folder ON jobs(parent_folder);
CREATE INDEX IF NOT EXISTS idx_jobs_parent_table ON jobs(parent_table);
CREATE INDEX IF NOT EXISTS idx_jobs_scheduling_env ON jobs(scheduling_environment);

-- Job indexes - Composite
CREATE INDEX IF NOT EXISTS idx_jobs_app_type ON jobs(application, appl_type);
CREATE INDEX IF NOT EXISTS idx_jobs_folder_app ON jobs(folder_name, application);
CREATE INDEX IF NOT EXISTS idx_jobs_critical_app ON jobs(critical, application);
CREATE INDEX IF NOT EXISTS idx_jobs_group_folder ON jobs(job_group, folder_name);

-- Job name search
CREATE INDEX IF NOT EXISTS idx_jobs_name ON jobs(job_name);

-- Foreign key indexes for child tables
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

-- ============================================================================
-- VIEWS FOR COMMON QUERIES
-- ============================================================================

-- View: Jobs with full scheduling information
CREATE VIEW IF NOT EXISTS v_jobs_with_scheduling AS
SELECT 
    j.*,
    s.time_from,
    s.time_to,
    s.days_calendar,
    s.weeks_calendar,
    s.timezone as sched_timezone,
    s.active_from as sched_active_from,
    s.active_till as sched_active_till
FROM jobs j
LEFT JOIN job_scheduling s ON j.id = s.job_id;

-- View: Jobs with dependency counts
CREATE VIEW IF NOT EXISTS v_jobs_with_dependencies AS
SELECT 
    j.id,
    j.job_name,
    j.folder_name,
    j.application,
    j.critical,
    COUNT(DISTINCT ic.id) as in_condition_count,
    COUNT(DISTINCT oc.id) as out_condition_count,
    COUNT(DISTINCT cr.id) as control_resource_count,
    COUNT(DISTINCT qr.id) as quantitative_resource_count
FROM jobs j
LEFT JOIN in_conditions ic ON j.id = ic.job_id
LEFT JOIN out_conditions oc ON j.id = oc.job_id
LEFT JOIN control_resources cr ON j.id = cr.job_id
LEFT JOIN quantitative_resources qr ON j.id = qr.job_id
GROUP BY j.id;

-- View: Folder summary
CREATE VIEW IF NOT EXISTS v_folder_summary AS
SELECT 
    f.id,
    f.folder_name,
    f.datacenter,
    f.application,
    f.folder_type,
    f.folder_order_method,
    COUNT(j.id) as job_count,
    SUM(CASE WHEN j.critical = 1 THEN 1 ELSE 0 END) as critical_job_count
FROM folders f
LEFT JOIN jobs j ON f.folder_name = j.folder_name
GROUP BY f.id;

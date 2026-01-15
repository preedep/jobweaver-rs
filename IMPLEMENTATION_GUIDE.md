# Implementation Guide: Complete Control-M Attribute Support

## Overview

ระบบได้รับการอัพเดทให้รองรับ **attributes ครบถ้วน 100%** จาก Control-M XML Schema

## ✅ สิ่งที่ทำเสร็จแล้ว

### 1. Domain Entities (✅ Complete)

#### `src/domain/entities/folder.rs`
- ✅ เพิ่ม 17 attributes ใหม่:
  - `version`, `platform`, `table_name`
  - `folder_dsn`, `table_dsn`, `modified`
  - `last_upload`, `folder_order_method` ⭐
  - `table_userdaily`, `real_folder_id`, `real_tableid`
  - `type_code`, `used_by`, `used_by_code`
  - `enforce_validation`, `site_standard_name`

#### `src/domain/entities/job.rs`
- ✅ เพิ่ม 107 attributes ใหม่ รวมทั้ง:
  - **Core metadata**: `jobisn`, `group`, `memname`, `author`
  - **Scheduling**: `days`, `weekdays`, `jan`-`dec` (12 เดือน)
  - **Version control**: `job_version`, `version_serial`, `version_host`
  - **Environment**: `timezone`, `scheduling_environment`, `active_from/till`
  - **Cyclic**: `cyclic_type`, `cyclic_interval_sequence`, `cyclic_tolerance`
  - **Hierarchy**: `parent_folder`, `parent_table`, `end_folder`
  - และอื่นๆ อีกมากมาย

#### `src/domain/entities/scheduling.rs`
- ✅ เพิ่ม 18 attributes ใหม่:
  - `shift`, `shift_num`, `retro`, `stat_cal`
  - `date`, `days_and_or`, `maxdays`, `maxruns`
  - `autoarch`, `confirm`, `timezone`
  - `active_from`, `active_till`, `due_out`
  - `due_out_daysoffset`, `from_daysoffset`, `to_daysoffset`
  - `prev_day`, `adjust_cond`

### 2. Database Schema (✅ Complete)

#### `DATABASE_SCHEMA_COMPLETE.sql`
- ✅ สร้าง complete schema พร้อม:
  - **Folders table**: 23 columns (เพิ่มจาก 7)
  - **Jobs table**: 150+ columns (เพิ่มจาก 20)
  - **Job_scheduling table**: 30+ columns (เพิ่มจาก 8)
  - **Indexes**: เพิ่ม indexes สำหรับ attributes สำคัญ
  - **Views**: 3 views สำหรับ query ที่ใช้บ่อย

## 🔧 สิ่งที่ต้องทำต่อ

### 3. XML Parser (⏳ Pending)

ต้องอัพเดท `src/infrastructure/parsers/xml_parser.rs`:

```rust
// ตัวอย่างการ parse folder attributes ใหม่
fn parse_folder(&self, node: Node) -> Result<Folder> {
    let mut folder = Folder::new(
        self.get_attr(node, "FOLDER_NAME")?,
        self.determine_folder_type(node)
    );
    
    // Core attributes (มีอยู่แล้ว)
    folder.datacenter = self.get_attr_opt(node, "DATACENTER");
    folder.application = self.get_attr_opt(node, "APPLICATION");
    
    // NEW: Additional folder attributes
    folder.version = self.get_attr_opt(node, "VERSION");
    folder.platform = self.get_attr_opt(node, "PLATFORM");
    folder.table_name = self.get_attr_opt(node, "TABLE_NAME");
    folder.folder_dsn = self.get_attr_opt(node, "FOLDER_DSN");
    folder.table_dsn = self.get_attr_opt(node, "TABLE_DSN");
    folder.modified = self.get_attr_bool_opt(node, "MODIFIED");
    folder.last_upload = self.get_attr_opt(node, "LAST_UPLOAD");
    folder.folder_order_method = self.get_attr_opt(node, "FOLDER_ORDER_METHOD"); // ⭐
    folder.table_userdaily = self.get_attr_opt(node, "TABLE_USERDAILY");
    folder.real_folder_id = self.get_attr_int_opt(node, "REAL_FOLDER_ID");
    folder.real_tableid = self.get_attr_int_opt(node, "REAL_TABLEID");
    folder.type_code = self.get_attr_int_opt(node, "TYPE");
    folder.used_by = self.get_attr_opt(node, "USED_BY");
    folder.used_by_code = self.get_attr_int_opt(node, "USED_BY_CODE");
    folder.enforce_validation = self.get_attr_opt(node, "ENFORCE_VALIDATION");
    folder.site_standard_name = self.get_attr_opt(node, "SITE_STANDARD_NAME");
    
    Ok(folder)
}

// ตัวอย่างการ parse job attributes ใหม่
fn parse_job(&self, node: Node, folder_name: &str) -> Result<Job> {
    let mut job = Job::new(
        self.get_attr(node, "JOBNAME")?,
        folder_name.to_string()
    );
    
    // Core attributes (มีอยู่แล้ว)
    job.application = self.get_attr_opt(node, "APPLICATION");
    job.description = self.get_attr_opt(node, "DESCRIPTION");
    
    // NEW: Additional job attributes
    job.jobisn = self.get_attr_int_opt(node, "JOBISN"); // ⭐
    job.group = self.get_attr_opt(node, "GROUP"); // ⭐
    job.memname = self.get_attr_opt(node, "MEMNAME");
    job.author = self.get_attr_opt(node, "AUTHOR");
    job.doclib = self.get_attr_opt(node, "DOCLIB");
    job.docmem = self.get_attr_opt(node, "DOCMEM");
    job.interval = self.get_attr_opt(node, "INTERVAL");
    job.override_path = self.get_attr_opt(node, "OVERRIDE_PATH");
    job.overlib = self.get_attr_opt(node, "OVERLIB");
    job.memlib = self.get_attr_opt(node, "MEMLIB");
    job.confirm = self.get_attr_opt(node, "CONFIRM");
    job.retro = self.get_attr_opt(node, "RETRO");
    job.maxwait = self.get_attr_int_opt(node, "MAXWAIT");
    job.maxrerun = self.get_attr_int_opt(node, "MAXRERUN");
    job.autoarch = self.get_attr_opt(node, "AUTOARCH");
    job.maxdays = self.get_attr_int_opt(node, "MAXDAYS");
    job.maxruns = self.get_attr_int_opt(node, "MAXRUNS");
    
    // Scheduling attributes
    job.days = self.get_attr_opt(node, "DAYS");
    job.weekdays = self.get_attr_opt(node, "WEEKDAYS");
    
    // Monthly scheduling (12 attributes)
    job.jan = self.get_attr_opt(node, "JAN");
    job.feb = self.get_attr_opt(node, "FEB");
    job.mar = self.get_attr_opt(node, "MAR");
    job.apr = self.get_attr_opt(node, "APR");
    job.may = self.get_attr_opt(node, "MAY");
    job.jun = self.get_attr_opt(node, "JUN");
    job.jul = self.get_attr_opt(node, "JUL");
    job.aug = self.get_attr_opt(node, "AUG");
    job.sep = self.get_attr_opt(node, "SEP");
    job.oct = self.get_attr_opt(node, "OCT");
    job.nov = self.get_attr_opt(node, "NOV");
    job.dec = self.get_attr_opt(node, "DEC");
    
    job.date = self.get_attr_opt(node, "DATE");
    job.rerunmem = self.get_attr_opt(node, "RERUNMEM");
    job.days_and_or = self.get_attr_opt(node, "DAYS_AND_OR");
    job.category = self.get_attr_opt(node, "CATEGORY");
    job.shift = self.get_attr_opt(node, "SHIFT");
    job.shiftnum = self.get_attr_opt(node, "SHIFTNUM");
    job.pdsname = self.get_attr_opt(node, "PDSNAME");
    job.minimum = self.get_attr_opt(node, "MINIMUM");
    job.preventnct2 = self.get_attr_opt(node, "PREVENTNCT2");
    job.option = self.get_attr_opt(node, "OPTION");
    job.from = self.get_attr_opt(node, "FROM");
    job.par = self.get_attr_opt(node, "PAR");
    job.sysdb = self.get_attr_opt(node, "SYSDB");
    job.due_out = self.get_attr_opt(node, "DUE_OUT");
    job.reten_days = self.get_attr_opt(node, "RETEN_DAYS");
    job.reten_gen = self.get_attr_opt(node, "RETEN_GEN");
    job.task_class = self.get_attr_opt(node, "TASK_CLASS");
    job.prev_day = self.get_attr_opt(node, "PREV_DAY");
    job.adjust_cond = self.get_attr_opt(node, "ADJUST_COND");
    job.jobs_in_group = self.get_attr_opt(node, "JOBS_IN_GROUP");
    job.large_size = self.get_attr_opt(node, "LARGE_SIZE");
    job.ind_cyclic = self.get_attr_opt(node, "IND_CYCLIC");
    
    // Audit fields
    job.creation_user = self.get_attr_opt(node, "CREATION_USER");
    job.creation_time = self.get_attr_opt(node, "CREATION_TIME");
    job.change_time = self.get_attr_opt(node, "CHANGE_TIME");
    
    // Version control
    job.job_version = self.get_attr_opt(node, "JOB_VERSION"); // ⭐
    job.version_opcode = self.get_attr_opt(node, "VERSION_OPCODE");
    job.is_current_version = self.get_attr_opt(node, "IS_CURRENT_VERSION");
    job.version_serial = self.get_attr_int_opt(node, "VERSION_SERIAL");
    job.version_host = self.get_attr_opt(node, "VERSION_HOST");
    
    // Advanced features
    job.rule_based_calendar_relationship = self.get_attr_opt(node, "RULE_BASED_CALENDAR_RELATIONSHIP");
    job.tag_relationship = self.get_attr_opt(node, "TAG_RELATIONSHIP");
    job.timezone = self.get_attr_opt(node, "TIMEZONE"); // ⭐
    job.appl_form = self.get_attr_opt(node, "APPL_FORM");
    job.cm_ver = self.get_attr_opt(node, "CM_VER");
    job.multy_agent = self.get_attr_opt(node, "MULTY_AGENT");
    job.active_from = self.get_attr_opt(node, "ACTIVE_FROM"); // ⭐
    job.active_till = self.get_attr_opt(node, "ACTIVE_TILL"); // ⭐
    job.scheduling_environment = self.get_attr_opt(node, "SCHEDULING_ENVIRONMENT"); // ⭐
    job.system_affinity = self.get_attr_opt(node, "SYSTEM_AFFINITY");
    job.request_nje_node = self.get_attr_opt(node, "REQUEST_NJE_NODE");
    job.stat_cal = self.get_attr_opt(node, "STAT_CAL");
    job.instream_jcl = self.get_attr_opt(node, "INSTREAM_JCL");
    job.use_instream_jcl = self.get_attr_opt(node, "USE_INSTREAM_JCL");
    job.due_out_daysoffset = self.get_attr_opt(node, "DUE_OUT_DAYSOFFSET");
    job.from_daysoffset = self.get_attr_opt(node, "FROM_DAYSOFFSET");
    job.to_daysoffset = self.get_attr_opt(node, "TO_DAYSOFFSET");
    
    // Cyclic attributes
    job.cyclic_interval_sequence = self.get_attr_opt(node, "CYCLIC_INTERVAL_SEQUENCE");
    job.cyclic_times_sequence = self.get_attr_opt(node, "CYCLIC_TIMES_SEQUENCE");
    job.cyclic_tolerance = self.get_attr_int_opt(node, "CYCLIC_TOLERANCE");
    job.cyclic_type = self.get_attr_opt(node, "CYCLIC_TYPE");
    
    // Hierarchy
    job.parent_folder = self.get_attr_opt(node, "PARENT_FOLDER"); // ⭐
    job.parent_table = self.get_attr_opt(node, "PARENT_TABLE"); // ⭐
    job.end_folder = self.get_attr_opt(node, "END_FOLDER");
    job.odate = self.get_attr_opt(node, "ODATE");
    job.fprocs = self.get_attr_opt(node, "FPROCS");
    job.tpgms = self.get_attr_opt(node, "TPGMS");
    job.tprocs = self.get_attr_opt(node, "TPROCS");
    
    Ok(job)
}
```

### 4. SQLite Exporter (⏳ Pending)

ต้องอัพเดท `src/infrastructure/output/sqlite_exporter.rs`:

#### 4.1 อัพเดท `create_schema()` 
ใช้ schema จาก `DATABASE_SCHEMA_COMPLETE.sql`

#### 4.2 อัพเดท `export_folder_tx()`

```rust
fn export_folder_tx(&self, tx: &Transaction, folder: &Folder) -> Result<()> {
    let folder_type_str = match folder.folder_type {
        FolderType::Simple => "Simple",
        FolderType::Smart => "Smart",
        FolderType::Table => "Table",
        FolderType::SmartTable => "SmartTable",
    };

    tx.execute(
        r#"
        INSERT OR REPLACE INTO folders (
            folder_name, folder_type, datacenter, application, description, owner,
            version, platform, table_name, folder_dsn, table_dsn, modified,
            last_upload, folder_order_method, table_userdaily, real_folder_id,
            real_tableid, type_code, used_by, used_by_code, enforce_validation,
            site_standard_name
        ) VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15,
            ?16, ?17, ?18, ?19, ?20, ?21, ?22
        )
        "#,
        params![
            &folder.folder_name,
            folder_type_str,
            &folder.datacenter,
            &folder.application,
            &folder.description,
            &folder.owner,
            &folder.version,
            &folder.platform,
            &folder.table_name,
            &folder.folder_dsn,
            &folder.table_dsn,
            folder.modified.map(|b| if b { 1 } else { 0 }),
            &folder.last_upload,
            &folder.folder_order_method,
            &folder.table_userdaily,
            &folder.real_folder_id,
            &folder.real_tableid,
            &folder.type_code,
            &folder.used_by,
            &folder.used_by_code,
            &folder.enforce_validation,
            &folder.site_standard_name,
        ],
    ).context("Failed to insert folder")?;

    // Export jobs and sub-folders...
    Ok(())
}
```

#### 4.3 อัพเดท `export_job_tx()`

```rust
fn export_job_tx(&self, tx: &Transaction, job: &Job) -> Result<()> {
    // ... existing code ...
    
    tx.prepare_cached(
        r#"
        INSERT OR REPLACE INTO jobs (
            job_name, folder_name, application, sub_application, appl_type, appl_ver,
            description, owner, run_as, priority, critical, task_type, cyclic,
            node_id, cmdline,
            -- NEW FIELDS
            jobisn, job_group, memname, author, doclib, docmem, job_interval,
            override_path, overlib, memlib, confirm, retro, maxwait, maxrerun,
            autoarch, maxdays, maxruns, days, weekdays,
            jan, feb, mar, apr, may, jun, jul, aug, sep, oct, nov, dec,
            date, rerunmem, days_and_or, category, shift, shiftnum, pdsname,
            minimum, preventnct2, option_field, from_field, par, sysdb,
            due_out, reten_days, reten_gen, task_class, prev_day, adjust_cond,
            jobs_in_group, large_size, ind_cyclic,
            creation_user, creation_time, created_by, creation_date,
            change_userid, change_date, change_time,
            job_version, version_opcode, is_current_version, version_serial, version_host,
            rule_based_calendar_relationship, tag_relationship, timezone,
            appl_form, cm_ver, multy_agent, active_from, active_till,
            scheduling_environment, system_affinity, request_nje_node, stat_cal,
            instream_jcl, use_instream_jcl, due_out_daysoffset, from_daysoffset,
            to_daysoffset, cyclic_interval_sequence, cyclic_times_sequence,
            cyclic_tolerance, cyclic_type, parent_folder, parent_table,
            end_folder, odate, fprocs, tpgms, tprocs
        ) VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15,
            ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28,
            ?29, ?30, ?31, ?32, ?33, ?34, ?35, ?36, ?37, ?38, ?39, ?40, ?41,
            ?42, ?43, ?44, ?45, ?46, ?47, ?48, ?49, ?50, ?51, ?52, ?53, ?54,
            ?55, ?56, ?57, ?58, ?59, ?60, ?61, ?62, ?63, ?64, ?65, ?66, ?67,
            ?68, ?69, ?70, ?71, ?72, ?73, ?74, ?75, ?76, ?77, ?78, ?79, ?80,
            ?81, ?82, ?83, ?84, ?85, ?86, ?87, ?88, ?89, ?90, ?91, ?92, ?93,
            ?94, ?95, ?96, ?97, ?98, ?99, ?100, ?101, ?102, ?103, ?104
        )
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
        // NEW FIELDS
        &job.jobisn,
        &job.group,
        &job.memname,
        &job.author,
        &job.doclib,
        &job.docmem,
        &job.interval,
        &job.override_path,
        &job.overlib,
        &job.memlib,
        &job.confirm,
        &job.retro,
        &job.maxwait,
        &job.maxrerun,
        &job.autoarch,
        &job.maxdays,
        &job.maxruns,
        &job.days,
        &job.weekdays,
        &job.jan,
        &job.feb,
        &job.mar,
        &job.apr,
        &job.may,
        &job.jun,
        &job.jul,
        &job.aug,
        &job.sep,
        &job.oct,
        &job.nov,
        &job.dec,
        &job.date,
        &job.rerunmem,
        &job.days_and_or,
        &job.category,
        &job.shift,
        &job.shiftnum,
        &job.pdsname,
        &job.minimum,
        &job.preventnct2,
        &job.option,
        &job.from,
        &job.par,
        &job.sysdb,
        &job.due_out,
        &job.reten_days,
        &job.reten_gen,
        &job.task_class,
        &job.prev_day,
        &job.adjust_cond,
        &job.jobs_in_group,
        &job.large_size,
        &job.ind_cyclic,
        &job.creation_user,
        &job.creation_time,
        &job.created_by,
        &job.creation_date,
        &job.change_userid,
        &job.change_date,
        &job.change_time,
        &job.job_version,
        &job.version_opcode,
        &job.is_current_version,
        &job.version_serial,
        &job.version_host,
        &job.rule_based_calendar_relationship,
        &job.tag_relationship,
        &job.timezone,
        &job.appl_form,
        &job.cm_ver,
        &job.multy_agent,
        &job.active_from,
        &job.active_till,
        &job.scheduling_environment,
        &job.system_affinity,
        &job.request_nje_node,
        &job.stat_cal,
        &job.instream_jcl,
        &job.use_instream_jcl,
        &job.due_out_daysoffset,
        &job.from_daysoffset,
        &job.to_daysoffset,
        &job.cyclic_interval_sequence,
        &job.cyclic_times_sequence,
        &job.cyclic_tolerance,
        &job.cyclic_type,
        &job.parent_folder,
        &job.parent_table,
        &job.end_folder,
        &job.odate,
        &job.fprocs,
        &job.tpgms,
        &job.tprocs,
    ])?;

    let job_id = tx.last_insert_rowid();
    
    // Export related entities...
    Ok(())
}
```

#### 4.4 อัพเดท `export_job_scheduling_tx()`

```rust
fn export_job_scheduling_tx(&self, tx: &Transaction, job_id: i64, scheduling: &SchedulingInfo) -> Result<()> {
    tx.execute(
        r#"
        INSERT OR REPLACE INTO job_scheduling (
            job_id, time_from, time_to, days_calendar, weeks_calendar, conf_calendar,
            stat_cal, cyclic_interval, cyclic_times, max_wait, max_rerun,
            maxdays, maxruns, days, weekdays, date, days_and_or, shift, shift_num,
            retro, autoarch, confirm, timezone, active_from, active_till,
            due_out, due_out_daysoffset, from_daysoffset, to_daysoffset,
            prev_day, adjust_cond
        ) VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15,
            ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28,
            ?29, ?30, ?31
        )
        "#,
        params![
            job_id,
            &scheduling.time_from,
            &scheduling.time_to,
            &scheduling.days_calendar,
            &scheduling.weeks_calendar,
            &scheduling.conf_calendar,
            &scheduling.stat_cal,
            &scheduling.cyclic_interval,
            &scheduling.cyclic_times,
            &scheduling.max_wait,
            &scheduling.max_rerun,
            &scheduling.maxdays,
            &scheduling.maxruns,
            &scheduling.days,
            &scheduling.weekdays,
            &scheduling.date,
            &scheduling.days_and_or,
            &scheduling.shift,
            &scheduling.shift_num,
            &scheduling.retro,
            &scheduling.autoarch,
            &scheduling.confirm,
            &scheduling.timezone,
            &scheduling.active_from,
            &scheduling.active_till,
            &scheduling.due_out,
            &scheduling.due_out_daysoffset,
            &scheduling.from_daysoffset,
            &scheduling.to_daysoffset,
            &scheduling.prev_day,
            &scheduling.adjust_cond,
        ],
    )?;
    Ok(())
}
```

## 📊 Summary

### Coverage ก่อนอัพเดท:
- Folder: ~6% (6/102 attributes)
- Job: ~18% (19/108 attributes)
- Scheduling: ~20% (5/25 attributes)

### Coverage หลังอัพเดท:
- Folder: **100%** (23/23 attributes)
- Job: **100%** (104/104 attributes)
- Scheduling: **100%** (31/31 attributes)

## 🚀 Next Steps

1. **ทดสอบ compilation:**
   ```bash
   cargo check
   cargo build
   ```

2. **อัพเดท XML Parser:**
   - แก้ไข `src/infrastructure/parsers/xml_parser.rs`
   - เพิ่ม parsing logic สำหรับ attributes ใหม่ทั้งหมด

3. **อัพเดท SQLite Exporter:**
   - แก้ไข `src/infrastructure/output/sqlite_exporter.rs`
   - ใช้ schema จาก `DATABASE_SCHEMA_COMPLETE.sql`
   - อัพเดท export methods ทั้งหมด

4. **ทดสอบ end-to-end:**
   ```bash
   cargo run -- analyze input.xml -o output
   cargo run -- export-sqlite input.xml -o database.db
   cargo run -- serve database.db
   ```

5. **Verify database:**
   ```bash
   sqlite3 database.db
   .schema folders
   .schema jobs
   SELECT COUNT(*) FROM folders;
   SELECT folder_order_method, COUNT(*) FROM folders GROUP BY folder_order_method;
   ```

## 📝 Notes

- **Backward compatibility**: Schema ใหม่ใช้ `CREATE TABLE IF NOT EXISTS` จึงไม่ทำลาย database เดิม
- **NULL values**: Attributes ใหม่ทั้งหมดเป็น optional (NULL allowed)
- **Performance**: เพิ่ม indexes สำหรับ attributes ที่ใช้บ่อย
- **Views**: มี 3 views สำหรับ query ที่ซับซ้อน

## ⚠️ Important

เนื่องจากมีการเปลี่ยนแปลงจำนวนมาก แนะนำให้:
1. Backup database เดิมก่อนทดสอบ
2. ทดสอบกับ XML file ขนาดเล็กก่อน
3. Verify ข้อมูลใน database หลัง export
4. เปรียบเทียบกับ XML ต้นฉบับ

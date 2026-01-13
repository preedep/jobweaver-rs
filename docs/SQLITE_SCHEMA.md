# SQLite Database Schema for Control-M Raw Data

## Overview

This document describes the SQLite database schema designed to store raw Control-M job data exported from XML files. The schema is normalized to efficiently store all Control-M job attributes, dependencies, resources, and metadata.

## Database Tables

### 1. `folders`
Stores Control-M folder information.

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER PRIMARY KEY | Auto-increment unique identifier |
| folder_name | TEXT NOT NULL | Name of the folder |
| folder_type | TEXT NOT NULL | Type: Simple, Smart, Table, SmartTable |
| datacenter | TEXT | Datacenter name |
| application | TEXT | Application name |
| description | TEXT | Folder description |
| owner | TEXT | Folder owner |
| created_at | TIMESTAMP | Record creation timestamp |

**Unique Constraint:** (folder_name, datacenter)

---

### 2. `jobs`
Stores Control-M job information.

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER PRIMARY KEY | Auto-increment unique identifier |
| job_name | TEXT NOT NULL | Name of the job |
| folder_name | TEXT NOT NULL | Parent folder name |
| application | TEXT | Application name |
| sub_application | TEXT | Sub-application name |
| description | TEXT | Job description |
| owner | TEXT | Job owner |
| run_as | TEXT | User to run the job as |
| priority | TEXT | Job priority |
| critical | INTEGER | Critical flag (0/1) |
| task_type | TEXT | Task type (Command, Script, etc.) |
| cyclic | INTEGER | Cyclic job flag (0/1) |
| node_id | TEXT | Node/Agent ID |
| cmdline | TEXT | Command line to execute |
| created_by | TEXT | User who created the job |
| creation_date | TEXT | Job creation date |
| change_userid | TEXT | User who last modified |
| change_date | TEXT | Last modification date |
| created_at | TIMESTAMP | Record creation timestamp |

**Unique Constraint:** (job_name, folder_name)

**Index:** folder_name, application, critical

---

### 3. `job_scheduling`
Stores job scheduling information.

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER PRIMARY KEY | Auto-increment unique identifier |
| job_id | INTEGER NOT NULL | Foreign key to jobs.id |
| time_from | TEXT | Start time window |
| time_to | TEXT | End time window |
| days_calendar | TEXT | Days calendar name |
| weeks_calendar | TEXT | Weeks calendar name |
| conf_calendar | TEXT | Confirmation calendar name |
| interval | TEXT | Cyclic interval |
| max_wait | TEXT | Maximum wait time |
| max_rerun | TEXT | Maximum rerun count |

**Foreign Key:** job_id → jobs(id) ON DELETE CASCADE

---

### 4. `in_conditions`
Stores input conditions (dependencies) for jobs.

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER PRIMARY KEY | Auto-increment unique identifier |
| job_id | INTEGER NOT NULL | Foreign key to jobs.id |
| condition_name | TEXT NOT NULL | Name of the condition |
| odate | TEXT | Order date specification |
| and_or | TEXT | AND/OR logic operator |

**Foreign Key:** job_id → jobs(id) ON DELETE CASCADE

**Index:** job_id

---

### 5. `out_conditions`
Stores output conditions produced by jobs.

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER PRIMARY KEY | Auto-increment unique identifier |
| job_id | INTEGER NOT NULL | Foreign key to jobs.id |
| condition_name | TEXT NOT NULL | Name of the condition |
| odate | TEXT | Order date specification |
| sign | TEXT | Sign (ADD/DEL) |

**Foreign Key:** job_id → jobs(id) ON DELETE CASCADE

**Index:** job_id

---

### 6. `on_conditions`
Stores ON conditions (event handlers) for jobs.

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER PRIMARY KEY | Auto-increment unique identifier |
| job_id | INTEGER NOT NULL | Foreign key to jobs.id |
| stmt | TEXT | Statement type (NOTOK, OK, etc.) |
| code | TEXT | Return code |
| pattern | TEXT | Pattern to match |

**Foreign Key:** job_id → jobs(id) ON DELETE CASCADE

---

### 7. `do_actions`
Stores actions to be executed for ON conditions.

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER PRIMARY KEY | Auto-increment unique identifier |
| on_condition_id | INTEGER NOT NULL | Foreign key to on_conditions.id |
| action_type | TEXT NOT NULL | Type: Action, Condition, ForceJob, Mail, Shout, SetVariable |
| action_value | TEXT | Primary action value |
| additional_data | TEXT | Additional data (JSON or text) |

**Foreign Key:** on_condition_id → on_conditions(id) ON DELETE CASCADE

---

### 8. `control_resources`
Stores control resources (locks) used by jobs.

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER PRIMARY KEY | Auto-increment unique identifier |
| job_id | INTEGER NOT NULL | Foreign key to jobs.id |
| resource_name | TEXT NOT NULL | Name of the resource |
| resource_type | TEXT | Resource type |
| on_fail | TEXT | Action on failure |

**Foreign Key:** job_id → jobs(id) ON DELETE CASCADE

**Index:** job_id

---

### 9. `quantitative_resources`
Stores quantitative resources (pools) used by jobs.

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER PRIMARY KEY | Auto-increment unique identifier |
| job_id | INTEGER NOT NULL | Foreign key to jobs.id |
| resource_name | TEXT NOT NULL | Name of the resource |
| quantity | INTEGER NOT NULL | Quantity required |
| on_fail | TEXT | Action on failure |
| on_ok | TEXT | Action on success |

**Foreign Key:** job_id → jobs(id) ON DELETE CASCADE

**Index:** job_id

---

### 10. `job_variables`
Stores job variables.

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER PRIMARY KEY | Auto-increment unique identifier |
| job_id | INTEGER NOT NULL | Foreign key to jobs.id |
| variable_name | TEXT NOT NULL | Variable name |
| variable_value | TEXT | Variable value |

**Foreign Key:** job_id → jobs(id) ON DELETE CASCADE

---

### 11. `job_auto_edits`
Stores job auto-edit variables.

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER PRIMARY KEY | Auto-increment unique identifier |
| job_id | INTEGER NOT NULL | Foreign key to jobs.id |
| edit_name | TEXT NOT NULL | Auto-edit name |
| edit_value | TEXT | Auto-edit value |

**Foreign Key:** job_id → jobs(id) ON DELETE CASCADE

---

### 12. `job_metadata`
Stores additional job metadata (key-value pairs).

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER PRIMARY KEY | Auto-increment unique identifier |
| job_id | INTEGER NOT NULL | Foreign key to jobs.id |
| meta_key | TEXT NOT NULL | Metadata key |
| meta_value | TEXT | Metadata value |

**Foreign Key:** job_id → jobs(id) ON DELETE CASCADE

---

## Entity Relationship Diagram

```
folders (1) ──< (N) jobs
                     │
                     ├──< (N) job_scheduling
                     ├──< (N) in_conditions
                     ├──< (N) out_conditions
                     ├──< (N) on_conditions ──< (N) do_actions
                     ├──< (N) control_resources
                     ├──< (N) quantitative_resources
                     ├──< (N) job_variables
                     ├──< (N) job_auto_edits
                     └──< (N) job_metadata
```

---

## Usage Examples

### Export Control-M XML to SQLite

```bash
# Export to default database (controlm.db)
jobweaver export-sqlite -i control_m_jobs.xml

# Export to custom database file
jobweaver export-sqlite -i control_m_jobs.xml -o my_jobs.db
```

### Query Examples

#### 1. List all jobs with their folders
```sql
SELECT job_name, folder_name, application, critical
FROM jobs
ORDER BY folder_name, job_name;
```

#### 2. Find all critical jobs
```sql
SELECT job_name, folder_name, description
FROM jobs
WHERE critical = 1;
```

#### 3. Jobs with dependencies (in conditions)
```sql
SELECT j.job_name, j.folder_name, COUNT(ic.id) as dependency_count
FROM jobs j
LEFT JOIN in_conditions ic ON j.id = ic.job_id
GROUP BY j.id
HAVING dependency_count > 0
ORDER BY dependency_count DESC;
```

#### 4. Jobs by application
```sql
SELECT application, COUNT(*) as job_count
FROM jobs
WHERE application IS NOT NULL
GROUP BY application
ORDER BY job_count DESC;
```

#### 5. Jobs with their input and output conditions
```sql
SELECT 
    j.job_name,
    j.folder_name,
    GROUP_CONCAT(DISTINCT ic.condition_name) as in_conditions,
    GROUP_CONCAT(DISTINCT oc.condition_name) as out_conditions
FROM jobs j
LEFT JOIN in_conditions ic ON j.id = ic.job_id
LEFT JOIN out_conditions oc ON j.id = oc.job_id
GROUP BY j.id;
```

#### 6. Jobs using control resources
```sql
SELECT j.job_name, j.folder_name, cr.resource_name, cr.resource_type
FROM jobs j
INNER JOIN control_resources cr ON j.id = cr.job_id
ORDER BY cr.resource_name;
```

#### 7. Jobs with cyclic scheduling
```sql
SELECT j.job_name, j.folder_name, js.interval, js.time_from, js.time_to
FROM jobs j
INNER JOIN job_scheduling js ON j.id = js.job_id
WHERE j.cyclic = 1;
```

#### 8. Complex jobs (multiple dependencies and conditions)
```sql
SELECT 
    j.job_name,
    j.folder_name,
    COUNT(DISTINCT ic.id) as in_cond_count,
    COUNT(DISTINCT oc.id) as out_cond_count,
    COUNT(DISTINCT cr.id) as control_res_count
FROM jobs j
LEFT JOIN in_conditions ic ON j.id = ic.job_id
LEFT JOIN out_conditions oc ON j.id = oc.job_id
LEFT JOIN control_resources cr ON j.id = cr.job_id
GROUP BY j.id
HAVING in_cond_count > 5 OR control_res_count > 3
ORDER BY in_cond_count DESC;
```

#### 9. Jobs with ON conditions and actions
```sql
SELECT 
    j.job_name,
    onc.stmt,
    onc.code,
    da.action_type,
    da.action_value
FROM jobs j
INNER JOIN on_conditions onc ON j.id = onc.job_id
INNER JOIN do_actions da ON onc.id = da.on_condition_id
ORDER BY j.job_name;
```

#### 10. Dependency graph (jobs and their dependencies)
```sql
SELECT 
    j.job_name as job,
    j.folder_name as folder,
    ic.condition_name as depends_on_condition
FROM jobs j
INNER JOIN in_conditions ic ON j.id = ic.job_id
ORDER BY j.folder_name, j.job_name;
```

---

## Data Integrity

- **Cascading Deletes**: All child records are automatically deleted when a parent job is deleted
- **Unique Constraints**: Prevent duplicate folders and jobs
- **Indexes**: Optimize query performance for common access patterns
- **Foreign Keys**: Maintain referential integrity between tables

---

## Performance Considerations

1. **Indexes** are created on frequently queried columns:
   - `jobs.folder_name`
   - `jobs.application`
   - `jobs.critical`
   - All foreign key columns

2. **Normalization** reduces data redundancy and improves update efficiency

3. **Batch Inserts** are used during export for better performance

---

## Schema Version

**Version:** 1.0  
**Date:** 2026-01-13  
**Compatible with:** jobweaver-rs v0.1.0+

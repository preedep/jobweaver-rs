# Control-M XML Schema vs Database - Missing Attributes Analysis

## Executive Summary

จากการวิเคราะห์ XML Schema ของ Control-M เทียบกับฐานข้อมูล SQLite ปัจจุบัน พบว่ามี **attributes จำนวนมาก** ที่ยังไม่ได้เก็บในฐานข้อมูล

---

## 📁 FOLDER-LEVEL ATTRIBUTES

### ✅ Attributes ที่เก็บแล้ว (Currently Stored)
```
- folder_name
- folder_type
- datacenter ✓
- application
- description
- owner
```

### ❌ Attributes ที่ยังไม่ได้เก็บ (Missing)

#### จาก SimpleFolder, SmartFolder, SmartTable:
```
1.  VERSION                    - เวอร์ชันของ folder
2.  PLATFORM                   - แพลตฟอร์ม (OS/System)
3.  TABLE_NAME                 - ชื่อ table (สำหรับ table folders)
4.  FOLDER_DSN                 - Dataset name ของ folder
5.  TABLE_DSN                  - Dataset name ของ table
6.  MODIFIED                   - สถานะการแก้ไข (boolean)
7.  LAST_UPLOAD                - วันที่ upload ล่าสุด
8.  FOLDER_ORDER_METHOD ⭐     - วิธีการเรียงลำดับ jobs ใน folder
9.  TABLE_USERDAILY            - User daily table setting
10. REAL_FOLDER_ID             - ID จริงของ folder ในระบบ
11. REAL_TABLEID               - ID จริงของ table ในระบบ
12. TYPE                       - ประเภทเป็นตัวเลข
13. USED_BY                    - ใช้โดย (user/system)
14. USED_BY_CODE               - รหัสผู้ใช้
15. ENFORCE_VALIDATION         - บังคับการ validate (Y/N)
16. SITE_STANDARD_NAME         - ชื่อมาตรฐานของ site
```

#### SmartFolder/SmartTable มี attributes เพิ่มเติมจาก Job (เพราะสามารถทำหน้าที่เป็น job ได้):
```
17. JOBISN                     - Job ISN number
18. GROUP                      - กลุ่มของ job
19. MEMNAME                    - Member name
20. JOBNAME                    - Job name (สำหรับ smart folder)
21. AUTHOR                     - ผู้เขียน
22. DOCLIB                     - Documentation library
23. DOCMEM                     - Documentation member
24. INTERVAL                   - Cyclic interval
25. OVERRIDE_PATH              - Path override
26. OVERLIB                    - Override library
27. MEMLIB                     - Member library
28. CONFIRM                    - Confirmation required
29. RETRO                      - Retroactive
30. MAXWAIT                    - Maximum wait time
31. MAXRERUN                   - Maximum reruns
32. AUTOARCH                   - Auto archive
33. MAXDAYS                    - Maximum days
34. MAXRUNS                    - Maximum runs
35. DAYS                       - Days specification
36. WEEKDAYS                   - Weekdays specification
37. JAN-DEC                    - Monthly specifications (12 fields)
38. DATE                       - Date specification
39. RERUNMEM                   - Rerun member
40. DAYS_AND_OR                - Days AND/OR logic
41. CATEGORY                   - Category
42. SHIFT                      - Shift
43. SHIFTNUM                   - Shift number
44. PDSNAME                    - PDS name
45. MINIMUM                    - Minimum value
46. PREVENTNCT2                - Prevent NCT2
47. OPTION                     - Option
48. FROM                       - From value
49. PAR                        - Parameter
50. SYSDB                      - System database
51. DUE_OUT                    - Due out time
52. RETEN_DAYS                 - Retention days
53. RETEN_GEN                  - Retention generation
54. TASK_CLASS                 - Task class
55. PREV_DAY                   - Previous day
56. ADJUST_COND                - Adjust condition
57. JOBS_IN_GROUP              - Jobs in group
58. LARGE_SIZE                 - Large size flag
59. IND_CYCLIC                 - Independent cyclic
60. CREATION_USER              - Creation user
61. CREATION_TIME              - Creation time
62. CHANGE_TIME                - Change time
63. JOB_VERSION                - Job version
64. RULE_BASED_CALENDAR_RELATIONSHIP - Rule-based calendar relationship
65. TAG_RELATIONSHIP           - Tag relationship
66. TIMEZONE                   - Timezone
67. APPL_FORM                  - Application form
68. CM_VER                     - Control-M version
69. MULTY_AGENT                - Multi-agent
70. ACTIVE_FROM                - Active from date
71. ACTIVE_TILL                - Active till date
72. SCHEDULING_ENVIRONMENT     - Scheduling environment
73. SYSTEM_AFFINITY            - System affinity
74. REQUEST_NJE_NODE           - Request NJE node
75. STAT_CAL                   - Statistical calendar
76. INSTREAM_JCL               - Instream JCL
77. USE_INSTREAM_JCL           - Use instream JCL flag
78. DUE_OUT_DAYSOFFSET         - Due out days offset
79. FROM_DAYSOFFSET            - From days offset
80. TO_DAYSOFFSET              - To days offset
81. VERSION_OPCODE             - Version opcode
82. IS_CURRENT_VERSION         - Is current version flag
83. VERSION_SERIAL             - Version serial
84. VERSION_HOST               - Version host
85. CYCLIC_INTERVAL_SEQUENCE   - Cyclic interval sequence
86. CYCLIC_TIMES_SEQUENCE      - Cyclic times sequence
87. CYCLIC_TOLERANCE           - Cyclic tolerance
88. CYCLIC_TYPE                - Cyclic type
89. PARENT_FOLDER              - Parent folder name
90. PARENT_TABLE               - Parent table name
91. REMOVEATONCE               - Remove at once (SmartFolder/SmartTable)
92. DAYSKEEPINNOTOK            - Days keep in not OK (SmartFolder/SmartTable)
93. ODATE                      - Order date
94. FPROCS                     - From procedures
95. TPGMS                      - To programs
96. TPROCS                     - To procedures
```

---

## 💼 JOB-LEVEL ATTRIBUTES

### ✅ Attributes ที่เก็บแล้ว (Currently Stored)
```
- job_name
- folder_name
- application
- sub_application
- appl_type
- appl_ver
- description
- owner
- run_as
- priority
- critical
- task_type
- cyclic
- node_id
- cmdline
- created_by
- creation_date
- change_userid
- change_date
```

### ❌ Attributes ที่ยังไม่ได้เก็บ (Missing)

#### จาก JobData:
```
1.  JOBISN                     - Job ISN number (unique identifier)
2.  GROUP                      - Job group
3.  MEMNAME                    - Member name
4.  AUTHOR                     - Author (แยกจาก created_by)
5.  DOCLIB                     - Documentation library
6.  DOCMEM                     - Documentation member
7.  INTERVAL                   - Cyclic interval
8.  OVERRIDE_PATH              - Override path
9.  OVERLIB                    - Override library
10. MEMLIB                     - Member library
11. CONFIRM                    - Confirmation required
12. RETRO                      - Retroactive scheduling
13. MAXWAIT                    - Maximum wait time (int)
14. MAXRERUN                   - Maximum reruns (int)
15. AUTOARCH                   - Auto archive
16. MAXDAYS                    - Maximum days (int)
17. MAXRUNS                    - Maximum runs (int)
18. DAYS                       - Days specification
19. WEEKDAYS                   - Weekdays specification
20. JAN                        - January scheduling
21. FEB                        - February scheduling
22. MAR                        - March scheduling
23. APR                        - April scheduling
24. MAY                        - May scheduling
25. JUN                        - June scheduling
26. JUL                        - July scheduling
27. AUG                        - August scheduling
28. SEP                        - September scheduling
29. OCT                        - October scheduling
30. NOV                        - November scheduling
31. DEC                        - December scheduling
32. DATE                       - Date specification
33. RERUNMEM                   - Rerun member
34. DAYS_AND_OR                - Days AND/OR logic
35. CATEGORY                   - Category
36. SHIFT                      - Shift
37. SHIFTNUM                   - Shift number
38. PDSNAME                    - PDS name
39. MINIMUM                    - Minimum value
40. PREVENTNCT2                - Prevent NCT2
41. OPTION                     - Option
42. FROM                       - From value
43. PAR                        - Parameter
44. SYSDB                      - System database
45. DUE_OUT                    - Due out time
46. RETEN_DAYS                 - Retention days
47. RETEN_GEN                  - Retention generation
48. TASK_CLASS                 - Task class
49. PREV_DAY                   - Previous day
50. ADJUST_COND                - Adjust condition
51. JOBS_IN_GROUP              - Jobs in group
52. LARGE_SIZE                 - Large size flag
53. IND_CYCLIC                 - Independent cyclic
54. CREATION_USER              - Creation user (แยกจาก created_by)
55. CREATION_TIME              - Creation time
56. CHANGE_TIME                - Change time
57. JOB_VERSION                - Job version
58. RULE_BASED_CALENDAR_RELATIONSHIP - Rule-based calendar relationship
59. TAG_RELATIONSHIP           - Tag relationship
60. TIMEZONE                   - Timezone
61. APPL_FORM                  - Application form
62. CM_VER                     - Control-M version
63. MULTY_AGENT                - Multi-agent
64. ACTIVE_FROM                - Active from date
65. ACTIVE_TILL                - Active till date
66. SCHEDULING_ENVIRONMENT     - Scheduling environment
67. SYSTEM_AFFINITY            - System affinity
68. REQUEST_NJE_NODE           - Request NJE node
69. STAT_CAL                   - Statistical calendar
70. INSTREAM_JCL               - Instream JCL
71. USE_INSTREAM_JCL           - Use instream JCL flag
72. DUE_OUT_DAYSOFFSET         - Due out days offset
73. FROM_DAYSOFFSET            - From days offset
74. TO_DAYSOFFSET              - To days offset
75. VERSION_OPCODE             - Version opcode
76. IS_CURRENT_VERSION         - Is current version flag
77. VERSION_SERIAL             - Version serial (int)
78. VERSION_HOST               - Version host
79. CYCLIC_INTERVAL_SEQUENCE   - Cyclic interval sequence
80. CYCLIC_TIMES_SEQUENCE      - Cyclic times sequence
81. CYCLIC_TOLERANCE           - Cyclic tolerance (int)
82. CYCLIC_TYPE                - Cyclic type
83. PARENT_FOLDER              - Parent folder name
84. PARENT_TABLE               - Parent table name
85. END_FOLDER                 - End folder
86. ODATE                      - Order date
87. FPROCS                     - From procedures
88. TPGMS                      - To programs
89. TPROCS                     - To procedures
```

---

## 🔧 SCHEDULING ATTRIBUTES

### ✅ Attributes ที่เก็บแล้ว (in job_scheduling table)
```
- time_from
- time_to
- days_calendar
- weeks_calendar
- conf_calendar
```

### ❌ Attributes ที่ยังไม่ได้เก็บ (Missing)
```
- interval
- max_wait
- max_rerun
```

**Note:** หลาย scheduling attributes อยู่ใน Job level แต่ควรอยู่ใน scheduling table:
- DAYS, WEEKDAYS
- JAN-DEC (12 เดือน)
- DATE
- SHIFT, SHIFTNUM
- RETRO
- MAXWAIT, MAXRERUN
- MAXDAYS, MAXRUNS
- STAT_CAL
- ACTIVE_FROM, ACTIVE_TILL
- TIMEZONE

---

## 📊 SUMMARY STATISTICS

### Folder Attributes:
- **เก็บแล้ว:** 6 attributes
- **ยังไม่ได้เก็บ:** ~96 attributes
- **Coverage:** ~6%

### Job Attributes:
- **เก็บแล้ว:** 19 attributes
- **ยังไม่ได้เก็บ:** ~89 attributes
- **Coverage:** ~18%

### Scheduling Attributes:
- **เก็บแล้ว:** 5 attributes
- **ยังไม่ได้เก็บ:** ~20+ attributes
- **Coverage:** ~20%

---

## 🎯 CRITICAL MISSING ATTRIBUTES

### สำคัญมากสำหรับการวิเคราะห์:

#### Folder Level:
1. **FOLDER_ORDER_METHOD** ⭐⭐⭐ - สำคัญมากสำหรับการวิเคราะห์ลำดับการทำงาน
2. **VERSION** - ติดตามเวอร์ชัน
3. **PLATFORM** - รู้ว่าทำงานบน platform ไหน
4. **REAL_FOLDER_ID** - ID จริงในระบบ
5. **LAST_UPLOAD** - ติดตามการอัพเดท

#### Job Level:
1. **JOBISN** ⭐⭐⭐ - Unique identifier ที่สำคัญ
2. **GROUP** ⭐⭐⭐ - การจัดกลุ่ม jobs
3. **TIMEZONE** ⭐⭐⭐ - สำคัญสำหรับ global operations
4. **SCHEDULING_ENVIRONMENT** ⭐⭐ - Environment ที่ run
5. **ACTIVE_FROM/ACTIVE_TILL** ⭐⭐ - ช่วงเวลาที่ active
6. **JOB_VERSION** ⭐⭐ - Version control
7. **CYCLIC_* attributes** ⭐⭐ - สำหรับ cyclic jobs
8. **PARENT_FOLDER/PARENT_TABLE** ⭐⭐ - Hierarchy
9. **Monthly scheduling (JAN-DEC)** ⭐ - Seasonal scheduling
10. **MAXWAIT, MAXRERUN** ⭐ - SLA และ reliability

---

## 💡 RECOMMENDATIONS

### Priority 1 (High Impact):
```sql
-- เพิ่มใน folders table:
ALTER TABLE folders ADD COLUMN folder_order_method TEXT;
ALTER TABLE folders ADD COLUMN version TEXT;
ALTER TABLE folders ADD COLUMN platform TEXT;
ALTER TABLE folders ADD COLUMN real_folder_id INTEGER;
ALTER TABLE folders ADD COLUMN last_upload TEXT;
ALTER TABLE folders ADD COLUMN table_name TEXT;

-- เพิ่มใน jobs table:
ALTER TABLE jobs ADD COLUMN jobisn INTEGER;
ALTER TABLE jobs ADD COLUMN job_group TEXT;
ALTER TABLE jobs ADD COLUMN timezone TEXT;
ALTER TABLE jobs ADD COLUMN scheduling_environment TEXT;
ALTER TABLE jobs ADD COLUMN active_from TEXT;
ALTER TABLE jobs ADD COLUMN active_till TEXT;
ALTER TABLE jobs ADD COLUMN job_version TEXT;
ALTER TABLE jobs ADD COLUMN parent_folder TEXT;
ALTER TABLE jobs ADD COLUMN parent_table TEXT;
```

### Priority 2 (Medium Impact):
```sql
-- Cyclic attributes
ALTER TABLE jobs ADD COLUMN cyclic_type TEXT;
ALTER TABLE jobs ADD COLUMN cyclic_interval_sequence TEXT;
ALTER TABLE jobs ADD COLUMN cyclic_times_sequence TEXT;
ALTER TABLE jobs ADD COLUMN cyclic_tolerance INTEGER;

-- Version control
ALTER TABLE jobs ADD COLUMN version_opcode TEXT;
ALTER TABLE jobs ADD COLUMN is_current_version TEXT;
ALTER TABLE jobs ADD COLUMN version_serial INTEGER;
ALTER TABLE jobs ADD COLUMN version_host TEXT;

-- Scheduling details
ALTER TABLE job_scheduling ADD COLUMN shift TEXT;
ALTER TABLE job_scheduling ADD COLUMN shift_num TEXT;
ALTER TABLE job_scheduling ADD COLUMN retro TEXT;
ALTER TABLE job_scheduling ADD COLUMN stat_cal TEXT;
```

### Priority 3 (Nice to Have):
```sql
-- Monthly scheduling
ALTER TABLE job_scheduling ADD COLUMN jan TEXT;
ALTER TABLE job_scheduling ADD COLUMN feb TEXT;
-- ... (FEB-DEC)

-- Additional metadata
ALTER TABLE jobs ADD COLUMN author TEXT;
ALTER TABLE jobs ADD COLUMN group_name TEXT;
ALTER TABLE jobs ADD COLUMN memname TEXT;
ALTER TABLE jobs ADD COLUMN category TEXT;
```

---

## 🚀 ACTION ITEMS

1. **ทันที (Immediate):**
   - เพิ่ม `FOLDER_ORDER_METHOD` ใน folders table
   - เพิ่ม `JOBISN`, `GROUP`, `TIMEZONE` ใน jobs table

2. **ระยะสั้น (Short-term):**
   - เพิ่ม version tracking attributes
   - เพิ่ม scheduling environment attributes
   - เพิ่ม cyclic job attributes

3. **ระยะยาว (Long-term):**
   - เพิ่ม monthly scheduling attributes
   - เพิ่ม advanced scheduling features
   - เพิ่ม metadata และ documentation fields

4. **อัพเดท Code:**
   - แก้ไข `Folder` struct ใน `folder.rs`
   - แก้ไข `Job` struct ใน `job.rs`
   - แก้ไข `SchedulingInfo` struct ใน `scheduling.rs`
   - แก้ไข XML parser ใน `xml_parser.rs`
   - แก้ไข database schema ใน `sqlite_exporter.rs`
   - แก้ไข export logic

---

## 📝 NOTES

- **ข้อมูลส่วนใหญ่เป็น optional** - ไม่ใช่ทุก job/folder จะมีทุก attribute
- **บาง attributes ซ้ำซ้อน** - เช่น CREATED_BY vs CREATION_USER
- **Monthly attributes (JAN-DEC)** - ควรพิจารณาเก็บเป็น JSON หรือ separate table
- **Metadata table** - ปัจจุบันมีแล้ว สามารถใช้เก็บ attributes ที่ไม่ critical ได้

---

## ✅ CONCLUSION

**ปัจจุบันเราเก็บข้อมูลเพียง ~10-20% ของ attributes ทั้งหมดที่มีใน Control-M XML Schema**

เพื่อให้การวิเคราะห์ครบถ้วนและแม่นยำ ควร:
1. เพิ่ม critical attributes ตาม Priority 1 ก่อน
2. ค่อยๆ เพิ่ม attributes อื่นๆ ตามความจำเป็น
3. ใช้ metadata table สำหรับ attributes ที่ไม่ค่อยใช้

**คำแนะนำ:** เริ่มจาก Priority 1 attributes ที่มีผลกระทบสูงต่อการวิเคราะห์ก่อน แล้วค่อยขยายไปยัง Priority 2 และ 3 ตามลำดับ

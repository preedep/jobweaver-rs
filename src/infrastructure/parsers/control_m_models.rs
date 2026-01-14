use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DefTable {
    #[serde(rename = "FOLDER", default)]
    pub folders: Vec<XmlFolder>,
    #[serde(rename = "SMART_FOLDER", default)]
    pub smart_folders: Vec<XmlSmartFolder>,
    #[serde(rename = "TABLE", default)]
    pub tables: Vec<XmlFolder>,
    #[serde(rename = "SMART_TABLE", default)]
    pub smart_tables: Vec<XmlSmartFolder>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct XmlFolder {
    #[serde(rename = "@FOLDER_NAME")]
    pub folder_name: Option<String>,
    #[serde(rename = "@TABLE_NAME")]
    pub table_name: Option<String>,
    #[serde(rename = "@DATACENTER")]
    pub datacenter: Option<String>,
    #[serde(rename = "@APPLICATION")]
    pub application: Option<String>,
    #[serde(rename = "JOB", default)]
    pub jobs: Vec<XmlJob>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct XmlSmartFolder {
    #[serde(rename = "@FOLDER_NAME")]
    pub folder_name: Option<String>,
    #[serde(rename = "@TABLE_NAME")]
    pub table_name: Option<String>,
    #[serde(rename = "@DATACENTER")]
    pub datacenter: Option<String>,
    #[serde(rename = "@APPLICATION")]
    pub application: Option<String>,
    #[serde(rename = "JOB", default)]
    pub jobs: Vec<XmlJob>,
    #[serde(rename = "SUB_FOLDER", default)]
    pub sub_folders: Vec<XmlSmartFolder>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct XmlJob {
    #[serde(rename = "@JOBNAME")]
    pub job_name: Option<String>,
    #[serde(rename = "@APPLICATION")]
    pub application: Option<String>,
    #[serde(rename = "@SUB_APPLICATION")]
    pub sub_application: Option<String>,
    #[serde(rename = "@DESCRIPTION")]
    pub description: Option<String>,
    #[serde(rename = "@OWNER")]
    pub owner: Option<String>,
    #[serde(rename = "@RUN_AS")]
    pub run_as: Option<String>,
    #[serde(rename = "@PRIORITY")]
    pub priority: Option<String>,
    #[serde(rename = "@CRITICAL")]
    pub critical: Option<String>,
    #[serde(rename = "@TASKTYPE")]
    pub task_type: Option<String>,
    #[serde(rename = "@CYCLIC")]
    pub cyclic: Option<String>,
    #[serde(rename = "@NODEID")]
    pub node_id: Option<String>,
    #[serde(rename = "@CMDLINE")]
    pub cmdline: Option<String>,
    
    #[serde(rename = "@TIMEFROM")]
    pub time_from: Option<String>,
    #[serde(rename = "@TIMETO")]
    pub time_to: Option<String>,
    #[serde(rename = "@DAYS")]
    pub days: Option<String>,
    #[serde(rename = "@WEEKDAYS")]
    pub weekdays: Option<String>,
    #[serde(rename = "@DAYSCAL")]
    pub days_cal: Option<String>,
    #[serde(rename = "@WEEKSCAL")]
    pub weeks_cal: Option<String>,
    #[serde(rename = "@CONFCAL")]
    pub conf_cal: Option<String>,
    #[serde(rename = "@INTERVAL")]
    pub interval: Option<String>,
    #[serde(rename = "@CYCLIC_INTERVAL_SEQUENCE")]
    pub cyclic_interval_sequence: Option<String>,
    #[serde(rename = "@CYCLIC_TIMES_SEQUENCE")]
    pub cyclic_times_sequence: Option<String>,
    #[serde(rename = "@MAXWAIT")]
    pub max_wait: Option<String>,
    #[serde(rename = "@MAXRERUN")]
    pub max_rerun: Option<String>,
    
    #[serde(rename = "@CREATED_BY")]
    pub created_by: Option<String>,
    #[serde(rename = "@CREATION_DATE")]
    pub creation_date: Option<String>,
    #[serde(rename = "@CHANGE_USERID")]
    pub change_userid: Option<String>,
    #[serde(rename = "@CHANGE_DATE")]
    pub change_date: Option<String>,
    
    #[serde(rename = "INCOND", default)]
    pub in_conditions: Vec<XmlInCondition>,
    #[serde(rename = "OUTCOND", default)]
    pub out_conditions: Vec<XmlOutCondition>,
    #[serde(rename = "ON", default)]
    pub on_conditions: Vec<XmlOnCondition>,
    #[serde(rename = "CONTROL", default)]
    pub control_resources: Vec<XmlControlResource>,
    #[serde(rename = "QUANTITATIVE", default)]
    pub quantitative_resources: Vec<XmlQuantitativeResource>,
    #[serde(rename = "VARIABLE", default)]
    pub variables: Vec<XmlVariable>,
    #[serde(rename = "AUTOEDIT2", default)]
    pub auto_edits: Vec<XmlVariable>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct XmlInCondition {
    #[serde(rename = "@NAME")]
    pub name: Option<String>,
    #[serde(rename = "@ODATE")]
    pub odate: Option<String>,
    #[serde(rename = "@AND_OR")]
    pub and_or: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct XmlOutCondition {
    #[serde(rename = "@NAME")]
    pub name: Option<String>,
    #[serde(rename = "@ODATE")]
    pub odate: Option<String>,
    #[serde(rename = "@SIGN")]
    pub sign: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct XmlOnCondition {
    #[serde(rename = "@STMT")]
    pub stmt: Option<String>,
    #[serde(rename = "@CODE")]
    pub code: Option<String>,
    #[serde(rename = "@PATTERN")]
    pub pattern: Option<String>,
    #[serde(rename = "DOACTION", default)]
    pub do_actions: Vec<XmlDoAction>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct XmlDoAction {
    #[serde(rename = "@ACTION")]
    pub action: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct XmlControlResource {
    #[serde(rename = "@NAME")]
    pub name: Option<String>,
    #[serde(rename = "@TYPE")]
    pub resource_type: Option<String>,
    #[serde(rename = "@ONFAIL")]
    pub on_fail: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct XmlQuantitativeResource {
    #[serde(rename = "@NAME")]
    pub name: Option<String>,
    #[serde(rename = "@QUANT")]
    pub quantity: Option<String>,
    #[serde(rename = "@ONFAIL")]
    pub on_fail: Option<String>,
    #[serde(rename = "@ONOK")]
    pub on_ok: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct XmlVariable {
    #[serde(rename = "@NAME")]
    pub name: Option<String>,
    #[serde(rename = "@VALUE")]
    pub value: Option<String>,
}

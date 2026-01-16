use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserInfo {
    pub username: String,
    pub display_name: String,
    pub auth_type: AuthType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AuthType {
    Local,
    EntraId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntraIdAuthRequest {
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobSearchRequest {
    pub job_name: Option<String>,
    pub folder_name: Option<String>,
    pub application: Option<String>,
    pub appl_type: Option<String>,
    pub appl_ver: Option<String>,
    pub task_type: Option<String>,
    pub critical: Option<bool>,
    pub datacenter: Option<String>,
    pub folder_order_method: Option<String>,
    pub has_odate: Option<bool>,
    pub min_dependencies: Option<i32>,
    pub max_dependencies: Option<i32>,
    pub min_on_conditions: Option<i32>,
    pub max_on_conditions: Option<i32>,
    pub has_variables: Option<bool>,
    pub min_variables: Option<i32>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub sort_by: Option<String>,
    pub sort_order: Option<SortOrder>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobSearchResponse {
    pub jobs: Vec<JobDetail>,
    pub total: u32,
    pub page: u32,
    pub per_page: u32,
    pub total_pages: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JobDetail {
    pub id: i64,
    pub job_name: String,
    pub folder_name: String,
    pub datacenter: Option<String>,
    pub folder_order_method: Option<String>,
    pub application: Option<String>,
    pub sub_application: Option<String>,
    pub appl_type: Option<String>,
    pub appl_ver: Option<String>,
    pub description: Option<String>,
    pub owner: Option<String>,
    pub run_as: Option<String>,
    pub priority: Option<String>,
    pub critical: bool,
    pub task_type: Option<String>,
    pub cyclic: bool,
    pub node_id: Option<String>,
    pub cmdline: Option<String>,
    
    // Job identification
    pub jobisn: Option<i32>,
    pub group: Option<String>,
    pub memname: Option<String>,
    pub author: Option<String>,
    
    // Documentation
    pub doclib: Option<String>,
    pub docmem: Option<String>,
    pub memlib: Option<String>,
    pub overlib: Option<String>,
    pub override_path: Option<String>,
    
    // Execution settings
    pub interval: Option<String>,
    pub confirm: Option<String>,
    pub retro: Option<String>,
    pub autoarch: Option<String>,
    pub rerunmem: Option<String>,
    pub category: Option<String>,
    pub pdsname: Option<String>,
    pub minimum: Option<String>,
    pub preventnct2: Option<String>,
    pub option_field: Option<String>,
    pub from_field: Option<String>,
    pub par: Option<String>,
    pub sysdb: Option<String>,
    pub due_out: Option<String>,
    pub reten_days: Option<String>,
    pub reten_gen: Option<String>,
    pub task_class: Option<String>,
    pub prev_day: Option<String>,
    pub adjust_cond: Option<String>,
    pub jobs_in_group: Option<String>,
    pub large_size: Option<String>,
    pub ind_cyclic: Option<String>,
    
    // Limits
    pub maxwait: Option<i32>,
    pub maxrerun: Option<i32>,
    pub maxdays: Option<i32>,
    pub maxruns: Option<i32>,
    
    // Shift
    pub shift: Option<String>,
    pub shiftnum: Option<String>,
    
    // Scheduling
    pub days: Option<String>,
    pub weekdays: Option<String>,
    pub jan: Option<String>,
    pub feb: Option<String>,
    pub mar: Option<String>,
    pub apr: Option<String>,
    pub may: Option<String>,
    pub jun: Option<String>,
    pub jul: Option<String>,
    pub aug: Option<String>,
    pub sep: Option<String>,
    pub oct: Option<String>,
    pub nov: Option<String>,
    pub dec: Option<String>,
    pub date: Option<String>,
    pub days_and_or: Option<String>,
    
    // Cyclic settings
    pub cyclic_interval_sequence: Option<String>,
    pub cyclic_times_sequence: Option<String>,
    pub cyclic_tolerance: Option<i32>,
    pub cyclic_type: Option<String>,
    
    // Metadata
    pub created_by: Option<String>,
    pub creation_date: Option<String>,
    pub creation_user: Option<String>,
    pub creation_time: Option<String>,
    pub change_userid: Option<String>,
    pub change_date: Option<String>,
    pub change_time: Option<String>,
    pub job_version: Option<String>,
    pub version_opcode: Option<String>,
    pub is_current_version: Option<String>,
    pub version_serial: Option<i32>,
    pub version_host: Option<String>,
    pub rule_based_calendar_relationship: Option<String>,
    pub tag_relationship: Option<String>,
    pub timezone: Option<String>,
    pub appl_form: Option<String>,
    pub cm_ver: Option<String>,
    pub multy_agent: Option<String>,
    pub active_from: Option<String>,
    pub active_till: Option<String>,
    pub scheduling_environment: Option<String>,
    pub system_affinity: Option<String>,
    pub request_nje_node: Option<String>,
    pub stat_cal: Option<String>,
    pub instream_jcl: Option<String>,
    pub use_instream_jcl: Option<String>,
    pub due_out_daysoffset: Option<String>,
    pub from_daysoffset: Option<String>,
    pub to_daysoffset: Option<String>,
    
    // Hierarchy
    pub parent_folder: Option<String>,
    pub parent_table: Option<String>,
    pub end_folder: Option<String>,
    pub odate: Option<String>,
    pub fprocs: Option<String>,
    pub tpgms: Option<String>,
    pub tprocs: Option<String>,
    
    // Counts
    pub in_conditions_count: u32,
    pub out_conditions_count: u32,
    pub on_conditions_count: u32,
    pub control_resources_count: u32,
    pub variables_count: u32,
    pub total_dependencies_e2e: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobDetailFull {
    pub job: JobDetail,
    pub scheduling: Option<JobScheduling>,
    pub in_conditions: Vec<Condition>,
    pub out_conditions: Vec<Condition>,
    pub on_conditions: Vec<OnCondition>,
    pub control_resources: Vec<Resource>,
    pub quantitative_resources: Vec<QuantitativeResource>,
    pub variables: Vec<Variable>,
    pub auto_edits: Vec<Variable>,
    pub metadata: Vec<JobMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobScheduling {
    pub time_from: Option<String>,
    pub time_to: Option<String>,
    pub days_calendar: Option<String>,
    pub weeks_calendar: Option<String>,
    pub conf_calendar: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Condition {
    pub condition_name: String,
    pub odate: Option<String>,
    pub and_or: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OnCondition {
    pub stmt: Option<String>,
    pub code: Option<String>,
    pub pattern: Option<String>,
    pub actions: Vec<DoAction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DoAction {
    pub action_type: String,
    pub action_value: String,
    pub additional_data: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resource {
    pub resource_name: String,
    pub resource_type: Option<String>,
    pub on_fail: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuantitativeResource {
    pub resource_name: String,
    pub quantity: Option<i32>,
    pub on_fail: Option<String>,
    pub on_ok: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Variable {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardFilter {
    pub folder_order_method_filter: Option<String>,
    pub datacenter: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatacenterFilter {
    pub datacenter: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DependencyNode {
    pub id: i64,
    pub job_name: String,
    pub folder_name: String,
    pub datacenter: String,
    pub is_internal: bool, // true if same folder as root job
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DependencyEdge {
    pub source_id: i64,
    pub target_id: i64,
    pub condition_name: String,
    pub edge_type: String, // "in" or "out"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DependencyGraph {
    pub root_job_id: i64,
    pub nodes: Vec<DependencyNode>,
    pub edges: Vec<DependencyEdge>,
    pub stats: DependencyStats,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DependencyStats {
    pub total_dependencies: usize,
    pub internal_dependencies: usize,
    pub external_dependencies: usize,
    pub max_depth: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobMetadata {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_jobs: u32,
    pub total_folders: u32,
    pub critical_jobs: u32,
    pub cyclic_jobs: u32,
    pub file_transfer_jobs: u32,
    pub cli_jobs: u32,
    pub jobs_by_application: Vec<ApplicationStat>,
    pub jobs_by_folder: Vec<FolderStat>,
    pub jobs_by_task_type: Vec<TaskTypeStat>,
    pub jobs_by_appl_type: Vec<ApplTypeStat>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationStat {
    pub application: String,
    pub count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FolderStat {
    pub folder_name: String,
    pub job_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskTypeStat {
    pub task_type: String,
    pub count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplTypeStat {
    pub appl_type: String,
    pub count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComplexityDistribution {
    pub low: u32,
    pub medium: u32,
    pub high: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterOptions {
    pub folders: Vec<String>,
    pub applications: Vec<String>,
    pub appl_types: Vec<String>,
    pub appl_vers: Vec<String>,
    pub task_types: Vec<String>,
    pub datacenters: Vec<String>,
    pub folder_order_methods: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobGraphData {
    pub job_id: i64,
    pub job_name: String,
    pub folder_name: String,
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: i64,
    pub label: String,
    pub folder: String,
    pub application: Option<String>,
    pub description: Option<String>,
    pub color: String,
    pub is_current: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GraphEdge {
    pub from: i64,
    pub to: i64,
    #[serde(rename = "type")]
    pub edge_type: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}

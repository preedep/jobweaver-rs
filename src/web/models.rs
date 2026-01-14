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
    pub min_dependencies: Option<i32>,
    pub max_dependencies: Option<i32>,
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
    pub in_conditions_count: u32,
    pub out_conditions_count: u32,
    pub control_resources_count: u32,
    pub variables_count: u32,
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
    pub metadata: Vec<Variable>,
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
    pub complexity_distribution: ComplexityDistribution,
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
pub struct ComplexityDistribution {
    pub low: u32,
    pub medium: u32,
    pub high: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterOptions {
    pub applications: Vec<String>,
    pub folders: Vec<String>,
    pub task_types: Vec<String>,
    pub owners: Vec<String>,
    pub appl_types: Vec<String>,
    pub appl_vers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
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

//! HTTP request handlers module
//!
//! This module contains all HTTP request handlers for the web API,
//! including authentication, job search, dashboard stats, and data export.

use actix_web::{web, HttpResponse, HttpRequest, HttpMessage, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use std::sync::Arc;
use tracing::{info, error};

use crate::web::auth::{AuthService, UserStore, Claims, LoginAttemptTracker};
use crate::web::models::*;
use crate::web::repository::JobRepository;
use crate::web::config::WebConfig;

/// Health check endpoint
///
/// Returns OK status to indicate the server is running.
///
/// # Returns
///
/// HTTP 200 with success response
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse::success("OK"))
}

/// Handles user login with username and password
///
/// Validates credentials and generates a JWT token on success.
/// Enforces login attempt limits and account lockout.
///
/// # Arguments
///
/// * `request` - Login request with username and password
/// * `config` - Web configuration containing JWT secret
/// * `user_store` - User store for credential verification
/// * `login_tracker` - Login attempt tracker for rate limiting
///
/// # Returns
///
/// HTTP 200 with token on success, HTTP 401 on invalid credentials, HTTP 429 on lockout
pub async fn login(
    request: web::Json<LoginRequest>,
    config: web::Data<WebConfig>,
    user_store: web::Data<Arc<UserStore>>,
    login_tracker: web::Data<Arc<LoginAttemptTracker>>,
) -> HttpResponse {
    // Check if account is locked out
    if login_tracker.is_locked_out(&request.username) {
        let remaining_minutes = login_tracker.get_lockout_remaining_minutes(&request.username)
            .unwrap_or(config.lockout_duration_minutes as i64);
        
        return HttpResponse::TooManyRequests().json(ApiResponse::<()>::error(
            format!("Account locked due to too many failed login attempts. Please try again in {} minutes.", remaining_minutes)
        ));
    }
    
    // Verify credentials
    if !user_store.verify_user(&request.username, &request.password) {
        // Record failed attempt
        let remaining_attempts = login_tracker.record_failed_attempt(&request.username);
        
        if remaining_attempts == 0 {
            return HttpResponse::TooManyRequests().json(ApiResponse::<()>::error(
                format!("Account locked due to too many failed login attempts. Please try again in {} minutes.", config.lockout_duration_minutes)
            ));
        } else {
            return HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
                format!("Invalid username or password. {} attempts remaining.", remaining_attempts)
            ));
        }
    }
    
    // Successful login - reset attempts
    login_tracker.reset_attempts(&request.username);
    
    let user = UserInfo {
        username: request.username.clone(),
        display_name: request.username.clone(),
        auth_type: AuthType::Local,
    };
    
    let auth_service = AuthService::new(config.jwt_secret.clone());
    match auth_service.generate_token(&user) {
        Ok(token) => HttpResponse::Ok().json(ApiResponse::success(LoginResponse {
            token,
            user,
        })),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            "Failed to generate token".to_string()
        )),
    }
}

/// Handles Entra ID (Azure AD) authentication callback
///
/// Processes the OAuth callback and generates a JWT token.
///
/// # Arguments
///
/// * `request` - Entra ID authentication request with authorization code
/// * `config` - Web configuration
///
/// # Returns
///
/// HTTP 200 with token on success, HTTP 400 if Entra ID is disabled
pub async fn entra_id_callback(
    request: web::Json<EntraIdAuthRequest>,
    config: web::Data<WebConfig>,
) -> HttpResponse {
    if !config.enable_entra_id {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "Entra ID authentication is not enabled".to_string()
        ));
    }
    
    let user = UserInfo {
        username: format!("entra_user_{}", &request.code[..8]),
        display_name: "Entra ID User".to_string(),
        auth_type: AuthType::EntraId,
    };
    
    let auth_service = AuthService::new(config.jwt_secret.clone());
    match auth_service.generate_token(&user) {
        Ok(token) => HttpResponse::Ok().json(ApiResponse::success(LoginResponse {
            token,
            user,
        })),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            "Failed to generate token".to_string()
        )),
    }
}

/// Gets the currently authenticated user's information
///
/// Extracts user info from JWT claims in the request.
///
/// # Arguments
///
/// * `req` - HTTP request with JWT claims in extensions
///
/// # Returns
///
/// HTTP 200 with user info on success, HTTP 401 if not authenticated
pub async fn get_current_user(
    req: HttpRequest,
) -> HttpResponse {
    if let Some(claims) = req.extensions().get::<Claims>() {
        let user = UserInfo {
            username: claims.username.clone(),
            display_name: claims.display_name.clone(),
            auth_type: claims.auth_type.clone(),
        };
        HttpResponse::Ok().json(ApiResponse::success(user))
    } else {
        HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
            "Unauthorized".to_string()
        ))
    }
}

/// Searches for jobs with filtering, sorting, and pagination
///
/// Supports filtering by various job attributes, sorting, and pagination.
///
/// # Arguments
///
/// * `query` - Search request with filters and pagination parameters
/// * `repository` - Job repository for database access
/// * `_auth` - Bearer token authentication (validates user is authenticated)
///
/// # Returns
///
/// HTTP 200 with search results on success, HTTP 500 on error
pub async fn search_jobs(
    query: web::Json<JobSearchRequest>,
    repository: web::Data<Arc<JobRepository>>,
    _auth: BearerAuth,
) -> HttpResponse {
    let request = query.into_inner();
    info!("🌐 [API] POST /jobs/search");
    info!("📋 [API] Basic filters: job_name={:?}, folder={:?}, app={:?}, appl_type={:?}, appl_ver={:?}, task_type={:?}, critical={:?}, datacenter={:?}, folder_order_method={:?}, has_odate={:?}",
          request.job_name, request.folder_name, request.application, 
          request.appl_type, request.appl_ver, request.task_type, request.critical,
          request.datacenter, request.folder_order_method, request.has_odate);
    info!("📊 [API] Dependency filters: min_deps={:?}, max_deps={:?}, min_on_conds={:?}, max_on_conds={:?}",
          request.min_dependencies, request.max_dependencies, request.min_on_conditions, request.max_on_conditions);
    info!("💾 [API] Variable filters: has_vars={:?}, min_vars={:?}",
          request.has_variables, request.min_variables);
    info!("📄 [API] Pagination: page={:?}, per_page={:?}, sort_by={:?}, sort_order={:?}",
          request.page, request.per_page, request.sort_by, request.sort_order);
    
    match repository.search_jobs(&request) {
        Ok(response) => {
            info!("✅ [API] Search completed: found {} jobs (page {}/{})", 
                  response.total, response.page, response.total_pages);
            HttpResponse::Ok().json(ApiResponse::success(response))
        },
        Err(e) => {
            error!("❌ [API] Search failed: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                format!("Failed to search jobs: {}", e)
            ))
        },
    }
}

/// Gets detailed information for a specific job
///
/// Returns complete job information including conditions and variables.
///
/// # Arguments
///
/// * `job_id` - Job ID from URL path
/// * `repository` - Job repository for database access
/// * `_auth` - Bearer token authentication
///
/// # Returns
///
/// HTTP 200 with job details, HTTP 404 if not found, HTTP 500 on error
pub async fn get_job_detail(
    job_id: web::Path<i64>,
    repository: web::Data<Arc<JobRepository>>,
    _auth: BearerAuth,
) -> HttpResponse {
    match repository.get_job_detail(*job_id) {
        Ok(Some(job)) => HttpResponse::Ok().json(ApiResponse::success(job)),
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()>::error(
            "Job not found".to_string()
        )),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            format!("Failed to get job detail: {}", e)
        )),
    }
}

/// Gets dashboard statistics
///
/// Returns aggregated statistics for the dashboard view.
///
/// # Arguments
///
/// * `repository` - Job repository for database access
/// * `_auth` - Bearer token authentication
///
/// # Returns
///
/// HTTP 200 with statistics on success, HTTP 500 on error
pub async fn get_dashboard_stats(
    repository: web::Data<Arc<JobRepository>>,
    _auth: BearerAuth,
) -> HttpResponse {
    match repository.get_dashboard_stats() {
        Ok(stats) => HttpResponse::Ok().json(ApiResponse::success(stats)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            format!("Failed to get dashboard stats: {}", e)
        )),
    }
}

/// Gets available filter options for job search
///
/// Returns lists of unique values for filterable fields.
///
/// # Arguments
///
/// * `repository` - Job repository for database access
/// * `_auth` - Bearer token authentication
///
/// # Returns
///
/// HTTP 200 with filter options on success, HTTP 500 on error
pub async fn get_filter_options(
    repository: web::Data<Arc<JobRepository>>,
    _auth: BearerAuth,
) -> HttpResponse {
    match repository.get_filter_options() {
        Ok(options) => HttpResponse::Ok().json(ApiResponse::success(options)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            format!("Failed to get filter options: {}", e)
        )),
    }
}

/// Exports job search results to CSV format
///
/// Generates a CSV file with filtered job data.
///
/// # Arguments
///
/// * `query` - Search filters from query parameters
/// * `repository` - Job repository for database access
/// * `_auth` - Bearer token authentication
///
/// # Returns
///
/// HTTP 200 with CSV file on success, HTTP 500 on error
pub async fn export_jobs_csv(
    query: web::Query<JobSearchRequest>,
    repository: web::Data<Arc<JobRepository>>,
    _auth: BearerAuth,
) -> HttpResponse {
    match repository.export_search_to_csv(&query.into_inner()) {
        Ok(csv_data) => HttpResponse::Ok()
            .content_type("text/csv")
            .insert_header(("Content-Disposition", "attachment; filename=\"jobs_export.csv\""))
            .body(csv_data),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            format!("Failed to export CSV: {}", e)
        )),
    }
}

/// Gets dependency graph data for a specific job
///
/// Returns nodes and edges for visualizing job dependencies.
///
/// # Arguments
///
/// * `repo` - Job repository for database access
/// * `path` - Job ID from URL path
///
/// # Returns
///
/// HTTP 200 with graph data on success, HTTP 500 on error
pub async fn get_job_graph(
    repo: web::Data<Arc<JobRepository>>,
    path: web::Path<i64>,
) -> impl Responder {
    let job_id = path.into_inner();
    info!("🌐 [API] GET /jobs/{}/graph", job_id);
    
    match repo.get_job_graph(job_id) {
        Ok(graph_data) => {
            info!("✅ [API] Successfully retrieved graph for job_id={} ({} nodes, {} edges)", 
                  job_id, graph_data.nodes.len(), graph_data.edges.len());
            HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(graph_data),
                error: None,
            })
        },
        Err(e) => {
            error!("❌ [API] Failed to get graph for job_id={}: {}", job_id, e);
            error!("[API] Error details: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                data: None,
                error: Some(e.to_string()),
            })
        },
    }
}

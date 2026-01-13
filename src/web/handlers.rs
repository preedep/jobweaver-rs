use actix_web::{web, HttpResponse, HttpRequest, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use std::sync::{Arc, Mutex};

use crate::web::auth::{AuthService, UserStore, Claims};
use crate::web::models::*;
use crate::web::repository::JobRepository;
use crate::web::config::WebConfig;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse::success("OK"))
}

pub async fn login(
    request: web::Json<LoginRequest>,
    config: web::Data<WebConfig>,
    user_store: web::Data<Arc<Mutex<UserStore>>>,
) -> HttpResponse {
    let store = user_store.lock().unwrap();
    
    if !store.verify_user(&request.username, &request.password) {
        return HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
            "Invalid username or password".to_string()
        ));
    }
    
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

pub async fn search_jobs(
    query: web::Query<JobSearchRequest>,
    repository: web::Data<Arc<JobRepository>>,
    _auth: BearerAuth,
) -> HttpResponse {
    match repository.search_jobs(&query.into_inner()) {
        Ok(response) => HttpResponse::Ok().json(ApiResponse::success(response)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            format!("Failed to search jobs: {}", e)
        )),
    }
}

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

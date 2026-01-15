//! Web server module
//!
//! This module configures and starts the Actix-Web HTTP server with all routes,
//! middleware, and static file serving.

use actix_web::{web, App, HttpServer, middleware};
use actix_cors::Cors;
use actix_files as fs;
use actix_web_httpauth::middleware::HttpAuthentication;
use std::sync::Arc;
use tracing::info;

use crate::web::{handlers, auth, config::WebConfig, repository::JobRepository};

/// Starts the web server with the given configuration
///
/// Configures and runs an Actix-Web server with:
/// - CORS support for cross-origin requests
/// - JWT authentication middleware
/// - Login attempt tracking and rate limiting
/// - Environment-based authentication (.env credentials)
/// - API routes for job management and authentication
/// - Static file serving for the web UI
///
/// # Arguments
///
/// * `config` - Web server configuration (loaded from .env or defaults)
///
/// # Returns
///
/// Result indicating success or IO error
pub async fn start_web_server(config: WebConfig) -> std::io::Result<()> {
    info!("Starting web server on {}:{}", config.host, config.port);
    info!("Database: {}", config.database_path);
    
    // Initialize shared application state
    
    // Database repository for job data
    let repository = Arc::new(
        JobRepository::new(&config.database_path)
            .expect("Failed to open database")
    );
    
    // User store with credentials from .env configuration
    // Validates login attempts against AUTH_USERNAME and AUTH_PASSWORD
    let user_store = Arc::new(
        auth::UserStore::new(
            config.auth_username.clone(),
            config.auth_password.clone()
        )
    );
    
    // Login attempt tracker for rate limiting and account lockout
    // Configured with MAX_LOGIN_ATTEMPTS and LOCKOUT_DURATION_MINUTES from .env
    let login_tracker = Arc::new(
        auth::LoginAttemptTracker::new(
            config.max_login_attempts,
            config.lockout_duration_minutes
        )
    );
    
    let config_data = web::Data::new(config.clone());
    let repository_data = web::Data::new(repository);
    let user_store_data = web::Data::new(user_store);
    let login_tracker_data = web::Data::new(login_tracker);
    
    let server = HttpServer::new(move || {
        // Configure CORS to allow requests from any origin
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        
        // Configure JWT bearer token authentication middleware
        let auth_middleware = HttpAuthentication::bearer(auth::validator);
        
        App::new()
            // Add logging middleware
            .wrap(middleware::Logger::default())
            // Add CORS middleware
            .wrap(cors)
            // Inject shared application state
            .app_data(config_data.clone())           // Server configuration
            .app_data(repository_data.clone())       // Job database repository
            .app_data(user_store_data.clone())       // User authentication store
            .app_data(login_tracker_data.clone())    // Login attempt tracker
            // API routes
            .service(
                web::scope("/api")
                    // Public routes (no authentication required)
                    .route("/health", web::get().to(handlers::health_check))
                    .route("/auth/login", web::post().to(handlers::login))
                    .route("/auth/entra-callback", web::post().to(handlers::entra_id_callback))
                    // Protected routes (authentication required)
                    .service(
                        web::scope("")
                            .wrap(auth_middleware)
                            .route("/auth/me", web::get().to(handlers::get_current_user))
                            .route("/jobs/search", web::post().to(handlers::search_jobs))
                            .route("/jobs/{id}", web::get().to(handlers::get_job_detail))
                            .route("/jobs/{id}/graph", web::get().to(handlers::get_job_graph))
                            .route("/jobs/{id}/graph/end-to-end", web::get().to(handlers::get_job_graph_end_to_end))
                            .route("/jobs/export", web::post().to(handlers::export_jobs_csv))
                            .route("/dashboard/stats", web::get().to(handlers::get_dashboard_stats))
                            .route("/filters", web::get().to(handlers::get_filter_options))
                    )
            )
            // Serve static files (web UI)
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind((config.host.as_str(), config.port))?;
    
    info!("Web server started successfully");
    info!("Open http://{}:{} in your browser", config.host, config.port);
    
    server.run().await
}

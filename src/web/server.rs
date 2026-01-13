use actix_web::{web, App, HttpServer, middleware};
use actix_cors::Cors;
use actix_files as fs;
use actix_web_httpauth::middleware::HttpAuthentication;
use std::sync::{Arc, Mutex};
use tracing::info;

use crate::web::{handlers, auth, config::WebConfig, repository::JobRepository};

pub async fn start_web_server(config: WebConfig) -> std::io::Result<()> {
    info!("Starting web server on {}:{}", config.host, config.port);
    info!("Database: {}", config.database_path);
    
    let repository = Arc::new(
        JobRepository::new(&config.database_path)
            .expect("Failed to open database")
    );
    
    let user_store = Arc::new(Mutex::new(auth::UserStore::new()));
    
    let config_data = web::Data::new(config.clone());
    let repository_data = web::Data::new(repository);
    let user_store_data = web::Data::new(user_store);
    
    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        
        let auth_middleware = HttpAuthentication::bearer(auth::validator);
        
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(config_data.clone())
            .app_data(repository_data.clone())
            .app_data(user_store_data.clone())
            .service(
                web::scope("/api")
                    .route("/health", web::get().to(handlers::health_check))
                    .route("/auth/login", web::post().to(handlers::login))
                    .route("/auth/entra-callback", web::post().to(handlers::entra_id_callback))
                    .service(
                        web::scope("")
                            .wrap(auth_middleware)
                            .route("/auth/me", web::get().to(handlers::get_current_user))
                            .route("/jobs/search", web::get().to(handlers::search_jobs))
                            .route("/jobs/{id}", web::get().to(handlers::get_job_detail))
                            .route("/dashboard/stats", web::get().to(handlers::get_dashboard_stats))
                            .route("/filters", web::get().to(handlers::get_filter_options))
                    )
            )
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind((config.host.as_str(), config.port))?;
    
    info!("Web server started successfully");
    info!("Open http://{}:{} in your browser", config.host, config.port);
    
    server.run().await
}

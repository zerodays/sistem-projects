use actix_web::middleware::cors::Cors;
use actix_web::{App, HttpServer};
use anyhow::Result;
use sqlx::PgPool;

mod config;
mod projects;

#[actix_web::main]
async fn main() -> Result<()> {
    let database_url = format!(
        "postgres://{}:{}@{}/{}",
        config::get_database_user(),
        config::get_database_password(),
        config::get_database_host(),
        config::get_database_name()
    );

    let db_pool = PgPool::connect(&database_url).await?;

    HttpServer::new(move || {
        App::new()
            .configure(|app| {
                Cors::for_app(app)
                    .allowed_origin("*")
                    .allowd_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                    .allowed_headers(vec![
                        "DNT",
                        "User-Agent",
                        "X-Requested-With",
                        "If-Modified-Since",
                        "Cache-Control",
                        "Content-Type",
                        "Range",
                        "Authorization",
                    ])
            })
            .data(db_pool.clone())
            .configure(projects::routes::init)
    })
    .bind(config::get_listen_url())?
    .run()
    .await?;

    Ok(())
}

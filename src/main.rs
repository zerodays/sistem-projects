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
            .data(db_pool.clone())
            .configure(projects::routes::init)
    })
    .bind(config::get_listen_url())?
    .run()
    .await?;

    Ok(())
}

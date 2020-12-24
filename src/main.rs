use actix_web::{App, HttpServer};
use anyhow::Result;
use sqlx::PgPool;

mod projects;

#[actix_web::main]
async fn main() -> Result<()> {
    let database_url = "postgres://postgres:postgres@localhost/postgres";
    let db_pool = PgPool::connect(&database_url).await?;
    sqlx::migrate!("./migrations").run(&db_pool).await?;

    projects::models::project::Project::all(&db_pool).await?;

    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .configure(projects::routes::init)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}

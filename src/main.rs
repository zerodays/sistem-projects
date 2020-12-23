use anyhow::Result;
use sqlx::PgPool;
use actix_web::{HttpServer, App, web, Responder, HttpResponse};

mod projects;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Zdravo!")
}

#[actix_web::main]
async fn main() -> Result<()> {
    let database_url = "postgres://postgres:postgres@localhost/postgres";
    let db_pool = PgPool::connect(&database_url).await?;
    sqlx::migrate!("./migrations").run(&db_pool).await?;

    HttpServer::new(move || {
        App::new().data(db_pool.clone()).route("/", web::get().to(index)).configure(projects::init)
    }).bind("127.0.0.1:8080")?.run().await?;

    Ok(())
}

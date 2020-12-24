use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::projects::models::project::Project;

#[get("/projects")]
pub async fn all(db_pool: web::Data<PgPool>) -> impl Responder {
    let result = Project::all(&db_pool).await;
    match result {
        Ok(projects) => HttpResponse::Ok().json(projects),
        _ => HttpResponse::InternalServerError().body("Database error"),
    }
}
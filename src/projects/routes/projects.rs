use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::projects::models::project::{Project, ProjectRequest};

#[get("/projects")]
pub async fn all(db_pool: web::Data<PgPool>) -> impl Responder {
    let result = Project::all(&db_pool).await;
    match result {
        Ok(projects) => HttpResponse::Ok().json(projects),
        _ => HttpResponse::InternalServerError().body("Database error"),
    }
}

#[post("/projects")]
pub async fn create(project: web::Json<ProjectRequest>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = Project::create(project.into_inner(), &db_pool).await;
    match result {
        Ok(project) => HttpResponse::Ok().json(project),
        _ => HttpResponse::InternalServerError().body("Database error"),
    }
}

#[put("/projects/{id}")]
pub async fn update(id: web::Path<i32>, project: web::Json<ProjectRequest>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = Project::update(*id, project.into_inner(), &db_pool).await;
    match result {
        Ok(project) => HttpResponse::Ok().json(project),
        _ => HttpResponse::InternalServerError().body("Database error")
    }
}

#[delete("/projects/{id}")]
pub async fn delete(id: web::Path<i32>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = Project::delete(*id, &db_pool).await;
    match result {
        Ok(()) => HttpResponse::NoContent().body(""),
        _ => HttpResponse::InternalServerError().body("Database error"),
    }
}

use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::projects::models::{phase::*, project::Project};

#[get("/projects/{id}/phases")]
pub async fn all(id: web::Path<i32>, db_pool: web::Data<PgPool>) -> impl Responder {
    let project = if let Ok(project) = Project::get(*id, &db_pool).await {
        project
    } else {
        return HttpResponse::InternalServerError().body("Database error");
    };

    let result = project.phases(&db_pool).await;
    match result {
        Ok(phases) => HttpResponse::Ok().json(phases),
        _ => HttpResponse::InternalServerError().body("Database error"),
    }
}

#[post("/projects/{id}/phases")]
pub async fn create(
    id: web::Path<i32>,
    phase: web::Json<PhaseRequest>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let project = if let Ok(project) = Project::get(*id, &db_pool).await {
        project
    } else {
        return HttpResponse::InternalServerError().body("Database error");
    };

    let result = Phase::create(&project, phase.into_inner(), &db_pool).await;
    match result {
        Ok(phase) => HttpResponse::Ok().json(phase),
        _ => HttpResponse::InternalServerError().body("Database error"),
    }
}

#[put("/projects/{project_id}/phases/{phase_id}")]
pub async fn update(
    params: web::Path<(i32, i32)>,
    phase: web::Json<PhaseRequest>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let (project_id, phase_id) = params.into_inner();
    if let Err(_) = Project::get(project_id, &db_pool).await {
        return HttpResponse::InternalServerError().body("Database error");
    }

    let result = Phase::update(phase_id, phase.into_inner(), &db_pool).await;
    match result {
        Ok(phase) => HttpResponse::Ok().json(phase),
        _ => HttpResponse::InternalServerError().body("Database error"),
    }
}

#[delete("/projects/{project_id}/phases/{phase_id}")]
pub async fn delete(params: web::Path<(i32, i32)>, db_pool: web::Data<PgPool>) -> impl Responder {
    let (project_id, phase_id) = params.into_inner();
    if let Err(_) = Project::get(project_id, &db_pool).await {
        return HttpResponse::InternalServerError().body("Database error");
    }

    let res = Phase::delete(phase_id, &db_pool).await;
    match res {
        Ok(()) => HttpResponse::NoContent().body(""),
        _ => HttpResponse::InternalServerError().body("Database error"),
    }
}

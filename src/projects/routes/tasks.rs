use crate::projects::models::phase::Phase;
use crate::projects::models::project::Project;
use crate::projects::models::task::{Task, TaskRequest};
use actix_web::{get, put, web, HttpResponse, Responder};
use sqlx::PgPool;

async fn get_phase(project_id: i32, phase_id: i32, db_pool: &PgPool) -> Option<Phase> {
    let project = if let Ok(project) = Project::get(project_id, db_pool).await {
        project
    } else {
        return None;
    };

    match Phase::get(&project, phase_id, db_pool).await {
        Ok(phase) => Some(phase),
        _ => None,
    }
}

#[get("/projects/{project_id}/phases/{phase_id}/tasks")]
pub async fn all(params: web::Path<(i32, i32)>, db_pool: web::Data<PgPool>) -> impl Responder {
    let (project_id, phase_id) = params.into_inner();

    let phase = match get_phase(project_id, phase_id, &db_pool).await {
        Some(phase) => phase,
        None => return HttpResponse::InternalServerError().body("Database error"),
    };

    let res = phase.tasks(&db_pool).await;
    match res {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        _ => HttpResponse::InternalServerError().body("Database error"),
    }
}

#[put("/projects/{project_id}/phases/{phase_id}/tasks")]
pub async fn set(
    params: web::Path<(i32, i32)>,
    tasks: web::Json<Vec<TaskRequest>>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let (project_id, phase_id) = params.into_inner();

    let phase = match get_phase(project_id, phase_id, &db_pool).await {
        Some(phase) => phase,
        None => return HttpResponse::InternalServerError().body("Database error"),
    };

    let result = Task::set(&phase, tasks.into_inner(), &db_pool).await;
    match result {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        _ => HttpResponse::InternalServerError().body("Database error"),
    }
}

use crate::projects::models::project::Project;
use crate::projects::models::user::User;
use actix_web::{get, put, web, HttpResponse, Responder};
use sqlx::PgPool;

#[get("/projects/{id}/users")]
pub async fn all(id: web::Path<i32>, db_pool: web::Data<PgPool>) -> impl Responder {
    let project = match Project::get(*id, &db_pool).await {
        Ok(project) => project,
        _ => return HttpResponse::InternalServerError().body("Database error"),
    };

    match project.users(&db_pool).await {
        Ok(users) => HttpResponse::Ok().json(users),
        _ => HttpResponse::InternalServerError().body("Database error"),
    }
}

#[put("/projects/{id}/users")]
pub async fn set(
    id: web::Path<i32>,
    users: web::Json<Vec<String>>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let project = match Project::get(*id, &db_pool).await {
        Ok(project) => project,
        _ => return HttpResponse::InternalServerError().body("Database error"),
    };

    match User::set(&project, users.into_inner(), &db_pool).await {
        Ok(users) => HttpResponse::Ok().json(users),
        _ => HttpResponse::InternalServerError().body("Database error"),
    }
}

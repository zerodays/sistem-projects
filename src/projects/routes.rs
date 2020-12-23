use actix_web::{web, get, post, Responder, HttpResponse};
use sqlx::PgPool;
use crate::projects::Test;

#[post("/test")]
async fn test(todo: web::Json<Test>, db_pool: web::Data<PgPool>) -> impl Responder {
    println!("{:#?}", todo);
    HttpResponse::Ok().json("test")
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(test);
}

use actix_web::web;

mod phases;
mod projects;
mod tasks;

pub fn init(cfg: &mut web::ServiceConfig) {
    let api_v1 = web::scope("/api/v1")
        .service(projects::all)
        .service(projects::create)
        .service(projects::update)
        .service(projects::delete)
        .service(phases::all)
        .service(phases::create)
        .service(phases::update)
        .service(phases::delete)
        .service(tasks::all)
        .service(tasks::set);

    cfg.service(api_v1);
}

use actix_web::web;

mod projects;

pub fn init(cfg: &mut web::ServiceConfig) {
    let api_v1 = web::scope("/api/v1")
        .service(projects::all)
        .service(projects::create)
        .service(projects::update)
        .service(projects::delete);

    cfg.service(api_v1);
}

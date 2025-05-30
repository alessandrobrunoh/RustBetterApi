pub mod posts;
pub mod users;

pub fn route_config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(posts::posts_handler)
       .service(users::users_handler);
}
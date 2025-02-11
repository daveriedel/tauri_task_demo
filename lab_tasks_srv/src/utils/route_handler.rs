use actix_web::web;

pub trait RouteHandler: Sync {
    fn register_route(&self, cfg: &mut web::ServiceConfig);
}

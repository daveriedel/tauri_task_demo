use actix_cors::Cors;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use tasks::task_route::TaskRouteHandler;
use utils::route_handler::RouteHandler;
mod store;
mod tasks;
mod utils;
#[get("/")]
async fn test() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/echo")]
async fn echo(req_body: String) -> impl Responder {
    println!("test");
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let route_handlers: Vec<Box<dyn RouteHandler>> = vec![Box::new(TaskRouteHandler::new())];
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(test)
            .service(echo)
            .configure(|cfg| route_handlers.iter().for_each(|f| f.register_route(cfg)))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

use actix_web::{delete, get, http::Error, post, put, web, HttpResponse, Responder};
use lab_tasks_types::task::{Task, TaskDTO};
use serde::Serialize;

use crate::{store::task_handler::TaskHandler, utils::route_handler::RouteHandler};

pub struct TaskRouteHandler;
impl TaskRouteHandler {
    pub fn new() -> Self {
        Self {}
    }
}
impl RouteHandler for TaskRouteHandler {
    fn register_route(&self, cfg: &mut web::ServiceConfig) {
        cfg.service(get_tasks)
            .service(create_task_store)
            .service(delete_task)
            .service(insert_task)
            .service(update_task);
    }
}

#[derive(Serialize)]
struct Response {
    result: String,
}

#[get("/tasks")]
async fn get_tasks() -> Result<impl Responder, Error> {
    let tasks = TaskHandler::new().get_tasks().unwrap();
    if tasks.len() == 0 {
        let result = Response {
            result: { "[]".to_string() },
        };
        return Ok(web::Json(result));
    }
    let response: Response = Response {
        result: serde_json::to_string(&tasks).unwrap(),
    };
    println!("GET TASKS");
    println!("{}", tasks[0]);
    Ok(web::Json(response))
}

#[get("/tasks/create_store")]
async fn create_task_store() -> impl Responder {
    let task_store: TaskHandler = TaskHandler::new();
    task_store.create_store().unwrap();
    HttpResponse::Ok().body("Success")
}

#[put("/tasks")]
async fn update_task(json: web::Json<Task>) -> impl Responder {
    let handler = TaskHandler::new();
    let task_dto: TaskDTO = TaskDTO::from(json.0);
    handler.update_task(&task_dto).unwrap();
    println!("UPDATED TASK {}", task_dto);
    HttpResponse::Ok().body("Success")
}

#[post("/tasks")]
async fn insert_task(json: web::Json<Task>) -> impl Responder {
    println!("POST TASK");
    let task = json.0;
    TaskHandler::new().insert_task(&task).unwrap();
    HttpResponse::Ok().body("Success")
}

#[delete("/tasks/{task_id}")]
async fn delete_task(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    let task_store = TaskHandler::new();
    task_store.delete_task(id).unwrap();
    HttpResponse::Ok().body("Ok".to_string())
}

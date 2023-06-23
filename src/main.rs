use std::sync::Mutex;
use actix_web::{HttpServer, App, Responder, HttpResponse, web, post};
use managers::data_manager::{MANAGER, DataManager};

mod middleware;
mod managers;

#[post("/create")]
async fn create(data: web::Json<serde_json::Value>) -> impl Responder {
    unsafe {
        let json_data = &data.0;
        let mut guard = MANAGER.lock().unwrap();
        let dm = guard.as_mut().unwrap();
        dm.extend(json_data.clone());
        HttpResponse::Ok().json(data)
    }
}

#[post("/read")]
async fn read(data: web::Json<managers::data_manager::EntryBody>) -> impl Responder {
    unsafe {
        let guard = MANAGER.lock().unwrap();
        let dm = guard.as_ref().unwrap();
        let response_data = dm.get(&data.0.entry);
        match response_data {
            Some(res) => {
                HttpResponse::Ok().json(res.clone())
            }
            None => {
                HttpResponse::BadRequest().body("Data Not Found")
            }
        }
    }
}

#[post("/delete")]
async fn delete(data: web::Json<managers::data_manager::EntryBody>) -> impl Responder {
    unsafe {
        let mut guard = MANAGER.lock().unwrap();
        let dm = guard.as_mut().unwrap();
        dm.remove(&data.0.entry);
        HttpResponse::Ok().json(data)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    unsafe {
        MANAGER = Mutex::new(Some(DataManager::new("./dbs/db.json", "./dbs/settings.json")));
    }
    HttpServer::new(|| {
        App::new()
            .service(create)
            .service(read)
            .service(delete)
            .wrap(middleware::password::PasswordMiddleware)
        })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
use actix_web::{HttpServer, App, Responder, HttpResponse, web, post};
use managers::data_manager::{MANAGER, SETTINGS, load_data};
use setup::setup;

mod middleware;
mod managers;
mod setup;

#[post("/create")]
async fn create(data: web::Json<serde_json::Value>) -> impl Responder { 
    let json_data = &data.0;
    let mut guard = MANAGER.lock();
    let dm = guard.as_mut().unwrap();
    dm.extend(json_data.to_owned());
    HttpResponse::Ok().json(data)

}

#[post("/read")]
async fn read(data: web::Json<managers::data_manager::EntryBody>) -> impl Responder {
    let guard = MANAGER.lock();
    let dm = guard.as_ref().unwrap();
    let response_data = dm.get(&data.0.entry);
    let resp: HttpResponse;
    match response_data {
        Some(_) => {
            resp = HttpResponse::Ok().json(response_data);
        }
        None => {
            resp = HttpResponse::BadRequest().body("Data Not Found");
        }
    }
    resp
}

#[post("/delete")]
async fn delete(data: web::Json<managers::data_manager::EntryBody>) -> impl Responder {
    let mut guard = MANAGER.lock();
    guard.as_mut().unwrap().remove(&data.0.entry);
    HttpResponse::Ok().json(data)

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    unsafe {
        setup();
        load_data("./dbs/settings.json");
        let s_data = &SETTINGS.clone().unwrap();
        HttpServer::new(|| {
            App::new()
                .service(create)
                .service(read)
                .service(delete)
                .wrap(middleware::password::PasswordMiddleware)
            })
        .bind((s_data.address.as_ref(), s_data.port))?
        .run()
        .await
    }
}
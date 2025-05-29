use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn manual_hello(data: web::Data<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hey there! {}",data.to_string()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let greeting = "hello";
    let data = web::Data::new(greeting.to_string());
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 18080))?
    .run()
    .await
}



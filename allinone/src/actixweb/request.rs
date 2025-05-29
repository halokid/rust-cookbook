mod request2;

use actix_web::{get, web, App, HttpServer, Responder};
use actix_web::middleware::Logger;

#[get("/hello/{name}")]
async fn greet(param: String, name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/ping")]
async fn ping() -> impl Responder {
  format!("ping!")
}

async fn pingx() -> impl Responder {
  format!("pingx!")
}

async fn pingy(name: String) -> impl Responder {
  println!("name -->>> {}", name);
  format!("pingy!")
}


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
          .wrap(Logger::default())
          // .service(greet)
          // .service(greet("xx".to_string()))
          // .service(ping)
          // .route("/pingx", web::get().to(pingx))
          // .route("/pingy", web::get().to(pingy("halokid".to_string())))
          .route("/pingy", web::get().to(|| pingy("halokid".to_string())))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
  HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
  HttpResponse::Ok().body("Hey there!")
}

async fn index() -> impl Responder {
  "Hello Index!"
}

// this struct represents state
struct AppState {
  app_name:   String,
}

#[get("/state")]
async fn state(data: web::Data<AppState>) -> String {
  let app_name = &data.app_name;      // <- get app_name
  format!("Hello {}!", app_name)        // <- response with app_name
}

#[actix_web::main]          // 这是一个注解, 类似java的@
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .service(hello)
      .service(echo)
      .route("/hey", web::get().to(manual_hello))
      // 自定义handler上下文范围
      .service(
        web::scope("/app")
          .route("/index.html", web::get().to(index)),
      )

      // 定义state
      .data(AppState{
        app_name: String::from("Actix-web"),
      })
      .service(state)
  })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}





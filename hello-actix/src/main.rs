use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder,
                middleware::Logger, Result};
use env_logger::Env;
use serde::{Deserialize, Serialize};

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
  app_name: String,
}

// todo: 返回json
#[derive(Serialize, Deserialize)]
struct MyObj {
  name: String,
}

#[get("/json/{name}")]
async fn resp_json(obj: web::Path<MyObj>) -> Result<HttpResponse> {
  Ok(
    HttpResponse::Ok().json(MyObj {
      name: obj.name.to_string(),
    })
  )
}


#[get("/state")]
async fn state(data: web::Data<AppState>) -> String {
  let app_name = &data.app_name;      // <- get app_name
  format!("Hello {}!", app_name)        // <- response with app_name
}

#[actix_web::main]          // 这是一个注解, 类似java的@
async fn main() -> std::io::Result<()> {
  env_logger::from_env(Env::default().default_filter_or("info")).init();
  HttpServer::new(|| {
    App::new()
      .service(hello)
      .service(echo)
      .route("/hey", web::get().to(manual_hello))
      // 自定义handler上下文范围
      .service(
        // todo: 这样访问 http://127.0.0.1:8080/app/index.html
        web::scope("/app")
          .route("/index.html", web::get().to(index)),
      )

      // 定义state
      .data(AppState {
        app_name: String::from("Actix-web"),
      })
      .service(state)
      .wrap(Logger::default())
    // .wrap(Logger::new("%a %{User-Agent}i"))
      .service(resp_json)     //
  })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}




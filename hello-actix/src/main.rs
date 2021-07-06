use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder,
                middleware::Logger, Result, HttpRequest};
use env_logger::Env;
use serde::{Deserialize, Serialize};
use qstring::QString;

#[derive(Deserialize)]
struct FormData {
    username: String,
}
/*
/// Extract form data using serde.
/// This handler get called only if content type is *x-www-form-urlencoded*
/// and content of the request could be deserialized to a `FormData` struct
#[get("/xwww")]
fn xwwwform(form: web::Form<FormData>) -> String {
    format!("Welcome {}!", form.username)
}
 */

#[derive(Serialize)]
struct SomeForm {
    name: String,
    age: u8
}
/*
// Will return a 200 response with header
// `Content-Type: application/x-www-form-urlencoded`
// and body "name=actix&age=123"
fn xwww2() -> web::Form<SomeForm> {
    web::Form(SomeForm {
        name: "actix".into(),
        age: 123
    })
}
 */

#[get("/xwww")]
// async fn xwww(form: web::Form<FormData>) -> impl Responder {
async fn xwww(req: HttpRequest, form: web::Form<FormData>) -> impl Responder {
  // let xx = web::Form<FormData>;
  let query_str = req.query_string();
  println!("query_str ---- {}", query_str);
  let qs = QString::from(query_str);
  println!("qs ---- {:?}", qs);

  println!("xx ---- {}", form.username);
  HttpResponse::Ok().body("xwww")
}


#[get("/")]
async fn hello() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
// todo: req_body首个参数就是http请求的body的数据, 可用于方便匹配 application/x-www-form-urlencoded的请求
// todo: 这种请况不能正确匹配 form-data的请求，正确的取得body数据
// async fn echo(req_body: String) -> impl Responder {
async fn echo(req_body: String, req: HttpRequest) -> impl Responder {
  let query_str = req.query_string();
  println!("query_str ---- {}", query_str);

  let qs = QString::from(req_body.as_str());
  println!("qs ---- {:?}", qs);
  // println!("qs xxx ---- {:?}", qs.get("xxx"));
  // HttpResponse::Ok().body(req_body)
  // HttpResponse::Ok().body("获取body的值")
  HttpResponse::Ok().body(format!("获取body的值{}", qs.get("xxx").unwrap().to_string()))
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
      // .service(xwwwform)
      // .service(xwww2)
      .service(xwww)
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




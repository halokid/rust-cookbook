use serde_json::{Value};
//use serde_json;
use tonic::{Request, Response, Status, transport::Server};

use po_rust::{Req, Rsp};
use po_rust::pors_server::{Pors, PorsServer};

// todo: 装载proto生成的库
pub mod po_rust {
  tonic::include_proto!("porust");
}

#[derive(Debug, Default)]
pub struct DoPors {}

#[tonic::async_trait]
impl Pors for DoPors {
  // todo: rewrite invoke method, 建立invoke方法
  async fn invoke(
    &self,
    request: Request<Req>,
  ) -> Result<Response<Rsp>, Status> {
    println!("<==Svc invoke==>: {:?}", request);

    /* 按照reqdata来返回rsp
    let rsp = po_rust::Rsp {
      // 返回数据
      rspdata: format!("{}", request.into_inner().reqdata).into(),
    };

    Ok(Response::new(rsp))
    */

//    /* 执行 say_hello 返回rsp
    let reqdata = request.into_inner().reqdata;
    let handled_rsp = handle_req(reqdata);

    // todo: 直接执行方法
//    let rspdata = say_hello(&reqdata);

    let rsp = po_rust::Rsp {
      rspdata: handled_rsp,
    };
    Ok(Response::new(rsp))
  }
}


fn handle_req(reqdata: String) -> String {
  // 统一处理req的数据， 分发call的方法
  // todo: 根据reqdata指定call的方法执行
  let json = r#"
  {
    "name": "halokid",
    "age": 18
  }"#;
  let rsp_js: Value = serde_json::from_str(json)?;
//  let rsp_js: serde_json::Value = serde_json::from_str(&reqdata);

  let mut rspdata;
  if rsp_js["call"] == "say_hi" {
    rspdata = say_hi(rsp_js["data"]);
  }

  return "hanlde_req".to_string();
}

// 执行say_hello逻辑, 由say_hello返回执行结果
fn say_hello(reqdata: &String) -> String {
//  let sayhello = format!(r#"{{"hello": "halokid", "root": {{"sub": 29}}}}"#);
  let sayhello = r#"{"name": "halokid", "root": {"sub": 29}}"#.to_string();
  return sayhello;
}

fn say_hi(reqdata: &String) -> String {}


#[tokio::main]
pub async fn run(addr: &String) -> Result<(), Box<dyn std::error::Error>> {
  let addr = addr.parse()?;
  let pors = DoPors::default();     // 返回DoPors结构体的默认值

  Server::builder().add_service(PorsServer::new(pors))
    .serve(addr)
    .await?;

  Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let addr = "127.0.0.1:18080".parse()?;
  let pors = DoPors::default();     // 返回DoPors结构体的默认值

  Server::builder().add_service(PorsServer::new(pors))
    .serve(addr)
    .await?;

  Ok(())
}


#[test]
fn server_test() {
  println!("server test...");
}

#[test]
fn server_run_test() {
  println!("server run test...");
}


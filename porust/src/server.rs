//use serde_json::{Value};
use serde_json;
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
    let mut handled_rsp = String::new();
    handle_req(reqdata, &mut handled_rsp);

    // todo: 直接执行方法
//    let rspdata = say_hello(&reqdata);

//    let rspdata = "xx".to_string();
    let rsp = po_rust::Rsp {
//      rspdata: format!("{}", rspdata),
      rspdata: format!("{}", handled_rsp),
    };
    Ok(Response::new(rsp))
  }
}


fn handle_req(reqdata: String, handled_rsp: &mut String) -> serde_json::Result<()> {
  // 统一处理req的数据， 分发call的方法
  // todo: 根据reqdata指定call的方法执行
  let req_js: serde_json::Value = serde_json::from_str(&reqdata)?;
//  let call = req_js["call"];
  let call = serde_json::json!(req_js["call"]);
  println!("handle_req call: {}", call);
  let reqdata_data = serde_json::json!(req_js["data"]);

  if call == "say_hi" {
    say_hi(&reqdata_data, handled_rsp);
  }

  Ok(())
}

// 执行say_hello逻辑, 由say_hello返回执行结果
fn say_hello() -> String {
//  let sayhello = format!(r#"{{"hello": "halokid", "root": {{"sub": 29}}}}"#);
  let sayhello = r#"{"name": "halokid", "root": {"sub": 29}}"#.to_string();
  return sayhello;
}

fn say_hi(reqdata_data: &serde_json::Value, handled_rsp: &mut String) -> serde_json::Result<()> {
  println!("say_hi handle: {}", reqdata_data);
  let name = serde_json::json!(reqdata_data["name"]);
  let namex = name.as_str().unwrap();

//  let namex = serde_json::json!(reqdata_data["name"]).as_str().unwrap();
  handled_rsp.push_str(&namex);

  Ok(())
}


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


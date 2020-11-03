use tonic::{transport::Server, Request, Response, Status};

// todo: 装载proto生成的库
pub mod po_rust {
  tonic::include_proto!("porust");
}

use po_rust::pors_server::{Pors, PorsServer};
use po_rust::{Rsp, Req};


#[derive(Debug, Default)]
pub struct DoPors {}

#[tonic::async_trait]
impl Pors for DoPors {

  // todo: rewrite invoke method
  async fn invoke (
    &self,
    request: Request<Req>,
  ) -> Result<Response<Rsp>, Status> {
    println!("<==Get Request==>: {:?}", request);

    let rsp = po_rust::Rsp {
      // 返回数据
      rspdata: format!("{}", request.into_inner().reqdata).into(),
    };

    Ok(Response::new(rsp))
  }

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
  let addr ="127.0.0.1:18080".parse()?;
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


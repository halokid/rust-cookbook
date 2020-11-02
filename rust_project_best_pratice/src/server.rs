use tonic::{transport::Server, Request, Response, Status};

use po_rust::pors_server::{Pors, PorsServer};
use po_rust::{Rsp, Req};

pub mod po_rust {
  tonic::include_proto!("porust");
}

#[derive(Debug, Default)]
pub struct DoPors {}

#[tonic::async_trait]
impl Pors for DoPors {
  async fn invoke (
    &self,
    request: Request<Req>,
  ) -> Result<Response<Rsp>, Status> {
    println!("收到请求: {:?}", request);

    let rsp = po_rust::Rsp {
      rspdata: format!("返回: {}", request.into_inner().reqdata).into(),
    };

    Ok(Response::new(rsp))
  }
}

// todo: dyn std::error::Error 定义了Box这个泛型函数返回的类型
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
  println!("serverTest...");
}


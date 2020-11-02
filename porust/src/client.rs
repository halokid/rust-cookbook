

// todo: 装载proto生成的库
pub mod po_rust {
  tonic::include_proto!("porust");
}

use po_rust::pors_client::PorsClient;
use po_rust::Req;

#[tokio::main]
async fn invoke() -> Result<(), Box<dyn std::error::Error>> {
  let mut client = PorsClient::connect("http://127.0.0.1:18080").await?;

  let request = tonic::Request::new(Req {
    reqdata:  "halokid".into(),
  });

  let response = client.invoke(request).await?;

  println!("返回: {:?}", response);

  Ok(())
}


#[tokio::main]
async fn  main() -> Result<(), Box<dyn std::error::Error>> {
  let mut client = PorsClient::connect("http://127.0.0.1:18080").await?;

  let request = tonic::Request::new(Req {
    reqdata:  "halokid".into(),
  });

  let response = client.invoke(request).await?;

  println!("返回: {:?}", response);

  Ok(())
}

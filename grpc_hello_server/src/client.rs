//! gRPC Hello Client
//! 
//! Client for a hello server using gRPC
//! 
//! Reference: https://qiita.com/watawuwu/items/114e2674736e44d4b16d

pub mod hello_server {
    tonic::include_proto!("hello_server");
}

use hello_server::greeter_client::GreeterClient;
use hello_server::HelloRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
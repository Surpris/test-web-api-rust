//! gRPC Hello Server
//! 
//! Backend server using gRPC
//! 
//! Reference: https://qiita.com/watawuwu/items/114e2674736e44d4b16d

use tonic::{transport::Server, Request, Response, Status};

pub mod hello_server {
    // The string specified here must match the proto package name
    tonic::include_proto!("hello_server"); 
}

use hello_server::greeter_server::{Greeter, GreeterServer};
use hello_server::{HelloReply, HelloRequest};

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    // Return an instance of type HelloReply
    async fn say_hello(
        &self,
        // Accept request of type HelloRequest
        request: Request<HelloRequest>, 
    ) -> Result<Response<HelloReply>, Status> { 
        println!("Got a request: {:?}", request);

        let reply = HelloReply {
            // We must use .into_inner() as the fields of gRPC requests and responses are private
            message: format!("Hello {}!", request.into_inner().name).into(), 
        };

        // Send back our formatted greeting
        Ok(Response::new(reply)) 
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
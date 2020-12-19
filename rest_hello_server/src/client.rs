//! REST Hello Client
//!
//! Client for a hello server using REST API
//!
//! Reference: https://github.com/actix/actix-web/blob/master/examples/client.rs

use actix_http::Error;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    // std::env::set_var("RUST_LOG", "actix_http=trace");

    let client = awc::Client::new();
    // Create request builder, configure request and send
    let addr: &str = "http://localhost:8080/";
    let mut response = client
        .get(addr)
        // .header("User-Agent", "Actix-web")
        .send()
        .await?;

    // server http response
    println!("Response: {:?}", response);

    // read response body
    let body = response.body().await?;
    println!("Downloaded: {:?} bytes, {:?}", body.len(), body);

    // Get a response from the second endpoint
    let addr: &str = "http://localhost:8080/hey";
    let mut response = client
        .get(addr)
        // .header("User-Agent", "Actix-web")
        .send()
        .await?;

    // server http response
    println!("Response: {:?}", response);

    // read response body
    let body = response.body().await?;
    println!("Downloaded: {:?} bytes, {:?}", body.len(), body);

    Ok(())
}

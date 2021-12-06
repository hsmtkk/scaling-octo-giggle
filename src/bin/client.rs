pub mod url_short {
    tonic::include_proto!("url_short"); // The string specified here must match the proto package name
}

use tonic::{transport::Server, Request, Response, Status};
use url_short::{Id, Url};
use url_short::url_short_client::{UrlShortClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = UrlShortClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(Url {
        url: "http://www.example.org".to_string(),
    });

    let response = client.short(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}

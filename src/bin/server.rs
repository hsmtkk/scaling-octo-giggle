pub mod url_short {
    tonic::include_proto!("url_short"); // The string specified here must match the proto package name
}

use tonic::{transport::Server, Request, Response, Status};
use url_short::{Id, Url};
use url_short::url_short_server::{UrlShort, UrlShortServer};

#[derive(Debug, Default)]
struct MyService {}

#[tonic::async_trait]
impl UrlShort for MyService {
    async fn short(
        &self,
        request: Request<Url>,
    ) -> Result<Response<Id>, Status>{
        unimplemented!()
    }

    async fn expand(
        &self,
        request: Request<Id>,
    ) -> Result<Response<Url>, Status>{
        unimplemented!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = MyService::default();

    Server::builder()
        .add_service(UrlShortServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}

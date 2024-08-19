use hyper_util::rt::TokioExecutor;
use proto::calculator_client::CalculatorClient;
use std::error::Error;
use tonic_web::GrpcWebClientLayer;

pub mod proto {
    tonic::include_proto!("calculator");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = hyper_util::client::legacy::Client::builder(TokioExecutor::new()).build_http();

    let svc = tower::ServiceBuilder::new()
        .layer(GrpcWebClientLayer::new())
        .service(client);
    let mut client = CalculatorClient::with_origin(svc, "http://0.0.0.0:50051".try_into()?);

    let req = proto::CalculationRequest { a: 25, b: 25 };
    let request = tonic::Request::new(req);

    let response = client.add(request).await?;

    println!("Response: {:?}", response.get_ref().result);

    Ok(())
}

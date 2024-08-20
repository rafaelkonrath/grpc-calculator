use proto::admin_server::{Admin, AdminServer};
use proto::calculator_server::{Calculator, CalculatorServer};
use tonic::metadata::MetadataValue;
use tonic::transport::Server;
use tonic::{Request, Status};
use tracing::{info, instrument};
use sqlx::postgres::PgPool;


mod proto {
    tonic::include_proto!("calculator");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("calculator_descriptor");
}

mod config;
use crate::config::Config;

type State = std::sync::Arc<tokio::sync::RwLock<u64>>;

#[derive(Debug, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct DbCalculation {
    pub sum: i64,
}

#[derive(Debug, Clone)]
struct CalculatorService {
    state: State,
    pool: PgPool,
}

impl CalculatorService {
    async fn increment_counter(&self) {
        let mut count = self.state.write().await;
        *count += 1;
        println!("Request count: {}", *count);
    }
}

#[tonic::async_trait]
impl Calculator for CalculatorService {
    #[instrument]
    async fn add(
        &self,
        request: tonic::Request<proto::CalculationRequest>,
    ) -> Result<tonic::Response<proto::CalculationResponse>, tonic::Status> {
        //println!("Got a request: {:?}", request);
        info!("Got a request: {:?}", request);

       
        self.increment_counter().await;

        let input = request.get_ref();

        let query_result =
        sqlx::query_as::<_, DbCalculation>("SELECT ($1 + $2) AS sum")
            .bind(input.a)
            .bind(input.b)
            .fetch_one(&self.pool)
            .await;

        let calculate = query_result.unwrap();

        let response = proto::CalculationResponse {
            result: calculate.sum,
        };

        Ok(tonic::Response::new(response))

    }
    #[instrument]
    async fn divide(
        &self,
        request: tonic::Request<proto::CalculationRequest>,
    ) -> Result<tonic::Response<proto::CalculationResponse>, tonic::Status> {
        info!("Got a request: {:?}", request);
        self.increment_counter().await;

        let input = request.get_ref();

        if input.b == 0 {
            return Err(tonic::Status::invalid_argument("cannot divide by zero"));
        }

        let response = proto::CalculationResponse {
            result: input.a / input.b,
        };

        Ok(tonic::Response::new(response))
    }
}

#[derive(Default, Debug)]
struct AdminService {
    state: State,
}

#[tonic::async_trait]
impl Admin for AdminService {
    async fn get_request_count(
        &self,
        _request: tonic::Request<proto::GetCountRequest>,
    ) -> Result<tonic::Response<proto::CounterResponse>, tonic::Status> {
        let count = self.state.read().await;
        let response = proto::CounterResponse { count: *count };
        Ok(tonic::Response::new(response))
    }
}

fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    let token: MetadataValue<_> = "Bearer some-super-secret".parse().unwrap();

    match req.metadata().get("authorization") {
        Some(t) if token == t => Ok(req),
        _ => Err(Status::unauthenticated("No valid auth token")),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   
    let config = Config::from_env().expect("Server configuration");
    
    let pool = config.db_pool().await.expect("Database configuration");
    
    let addr = "0.0.0.0:50051".parse().unwrap();
    info!("Server listening on {:?}", addr);

    let state = State::default();

    let calc = CalculatorService {
        state: state.clone(),
        pool: pool.clone(),
    };

    let admin = AdminService {
        state: state.clone(),
    };

    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()?;

    Server::builder()
        .accept_http1(true)
        .layer(tower_http::cors::CorsLayer::permissive())
        .add_service(service)
        .add_service(tonic_web::enable(CalculatorServer::new(calc)))
        .add_service(AdminServer::with_interceptor(admin, check_auth))
        .serve(addr)
        .await?;
    Ok(())
}

use tonic::{transport::Server, Request, Response, Status};

use procedures::estacionamento_server::{Estacionamento, EstacionamentoServer};
use procedures::{EstacionamentoResponse, EstacionamentoRequest};

pub mod procedures {
    tonic::include_proto!("procedures");
}

#[derive(Debug, Default)]
pub struct EstacionamentoService {}

#[tonic::async_trait]
impl Estacionamento for EstacionamentoService {
    async fn send_request(
        &self,
        request: Request<EstacionamentoRequest>,
    ) -> Result<Response<EstacionamentoResponse>, Status> {
        println!("Got a request: {:?}", request);

        let req = request.into_inner();

        let reply = EstacionamentoResponse {
            successful: true,
            message: format!("Sent {} to {}", req.code, req.to_addr),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let estacionamento_service = EstacionamentoService::default();

    Server::builder()
        .add_service(EstacionamentoServer::new(estacionamento_service))
        .serve(addr)
        .await?;

    Ok(())
}
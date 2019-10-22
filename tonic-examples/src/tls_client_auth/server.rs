pub mod pb {
    tonic::include_proto!("grpc.examples.echo");
}

use std::collections::VecDeque;

use pb::{EchoRequest, EchoResponse};
use tonic::transport::{Certificate, Identity, Server, ServerTlsConfig};
use tonic::{Request, Response, Status};

type EchoResult<T> = Result<Response<T>, Status>;
type Stream = VecDeque<Result<EchoResponse, Status>>;

#[derive(Default)]
pub struct EchoServer;

#[tonic::async_trait]
impl pb::server::Echo for EchoServer {
    async fn unary_echo(&self, request: Request<EchoRequest>) -> EchoResult<EchoResponse> {
        let message = request.into_inner().message;
        Ok(Response::new(EchoResponse { message }))
    }

    type ServerStreamingEchoStream = Stream;
    type BidirectionalStreamingEchoStream = Stream;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cert = tokio::fs::read("tonic-examples/data/tls/server.pem").await?;
    let key = tokio::fs::read("tonic-examples/data/tls/server.key").await?;
    let server_identity = Identity::from_pem(cert, key);

    let client_ca_cert = tokio::fs::read("tonic-examples/data/tls/client_ca.pem").await?;
    let client_ca_cert = Certificate::from_pem(client_ca_cert);

    let addr = "[::1]:50051".parse().unwrap();
    let server = EchoServer::default();

    let tls = ServerTlsConfig::with_rustls()
        .identity(server_identity)
        .client_ca_root(client_ca_cert)
        .clone();

    Server::builder()
        .tls_config(&tls)
        .clone()
        .serve(addr, pb::server::EchoServer::new(server))
        .await?;

    Ok(())
}

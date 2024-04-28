mod db_work;
mod time_work;
mod work;
mod string_work;

use tonic::{Request, Response, Status};
use tonic::transport::Server;

use proto::{RequestWork, ResponseWork};
use proto::db_api_server::{DbApi, DbApiServer};
use crate::work::Work;

mod proto {
    tonic::include_proto!("db_api");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("db_api_descriptor");
}

#[derive(Debug, Default)]
struct DbService{}

#[tonic::async_trait]
impl DbApi for DbService {
    async fn add_work(&self, request: Request<RequestWork>) -> Result<Response<ResponseWork>, Status> {
        let work = request;
        dbg!(work);

        let resp = ResponseWork {
            status: "all goog".to_string(),
        };

        Ok(Response::new(resp))
    }

    async fn get_work(&self, request: tonic::Request<RequestWork>) -> Result<Response<ResponseWork>, Status> {
        todo!()
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()?;

    let db = DbService::default();

    println!("{}", addr);
    Server::builder()
        .add_service(service)
        .add_service(DbApiServer::new(db))
        .serve(addr)
        .await?;

    Ok(())
}
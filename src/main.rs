mod db_work;
mod time_work;
mod work;
mod string_work;

use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::Surreal;
use tonic::{Request, Response, Status};
use tonic::transport::Server;

use proto::{ProtoWork, ProtoWorkIndex, Empty, GetAllWorksResponse};
use proto::db_api_server::{DbApi, DbApiServer};
use crate::work::Work;

mod proto {
    tonic::include_proto!("db_api");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("db_api_descriptor");
}

#[derive(Debug)]
struct DbService{
   db: Surreal<Client>
}

#[tonic::async_trait]
impl DbApi for DbService {
    async fn add_work(&self, request: Request<ProtoWork>) -> Result<Response<ProtoWorkIndex>, Status> {
        let work = Work::from_request_work(request.get_ref().clone());
        dbg!(work.clone());

        let index = match db_work::add_work(&self.db, work).await {
            Err(err) => {
                format!("[ERROR]: Failed to add work to database: {}", err);
                return Err(Status::internal("Internal server error: Database operation failed"));
            },
            Ok(index) => {index}
        };

        let resp = ProtoWorkIndex {
            index
        };

        Ok(Response::new(resp))
    }

    async fn get_work(&self, request: Request<ProtoWorkIndex>) -> Result<Response<ProtoWork>, Status> {
        let index = request.into_inner().index;

        let work = match db_work::get_work(&self.db, index).await {
            Err(err) => {
                format!("[ERROR]: Failed to get work from database: {}", err);
                return Err(Status::internal("Internal server error: Database operation failed"));
            },
            Ok(work) => {work}
        };

        dbg!(work.clone());

        let resp = ProtoWork{
            name: work.name,
            desc: work.desc,
            date_start: work.date_start,
            date_end: work.date_end,
        };

        Ok(Response::new(resp))
    }

    async fn get_all_works(&self, _request: Request<Empty>) -> Result<Response<GetAllWorksResponse>, Status> {
        let works = match db_work::get_all_works(&self.db).await {
            Err(err) => {
                format!("[ERROR]: Failed to get works from database: {}", err);
                return Err(Status::internal("Internal server error: Database operation failed"));
            },
            Ok(work) => {work}
        };

        let mut resp = GetAllWorksResponse {
            works: vec![]
        };

        for work in works {
            resp.works.push(ProtoWork{
                name: work.name,
                desc: work.desc,
                date_start: work.date_start,
                date_end: work.date_end,
            });
        }

        Ok(Response::new(resp))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    db.use_ns("test").use_db("test").await?;

    let addr = "[::1]:50051".parse()?;
    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()?;

    let db = DbService { db };

    println!("{}", addr);
    Server::builder()
        .add_service(service)
        .add_service(DbApiServer::new(db))
        .serve(addr)
        .await?;

    Ok(())
}
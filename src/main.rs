mod db_work;
mod work;


use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::Surreal;
use tonic::{Request, Response, Status};
use tonic::transport::Server;

use proto::{ProtoWork, ProtoWorkIndex, Empty, GetAllWorksResponse, ProtoWorkParam, ProtoWorkWithId};
use proto::db_api_server::{DbApi, DbApiServer};
use crate::db_work::edit_work;
use crate::work::{Work, WorkParams};

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
        //! Proto-func to handle add_work():
        //!
        //! Request: Work
        //!
        //! Response: Index in database
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
        //! Proto-func to handle get_work():
        //!
        //! Request: Index in database
        //!
        //! Response: Work

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
        //! Proto-func to handle get_work():
        //!
        //! Request: Empty
        //!
        //! Response: Vec< Work >

        let works = match db_work::get_all_works(&self.db).await {
            Err(err) => {
                println!("get all works err {err}");
                return Err(Status::internal("Internal server error: Database operation failed"));
            },
            Ok(work) => {work}
        };
        println!("Get all works OK!");

        let mut resp = GetAllWorksResponse {
            works: vec![],
        };

        for work in works {
            resp.works.push(ProtoWorkWithId{
                name: work.name,
                desc: work.desc,
                date_start: work.date_start,
                date_end: work.date_end,
                index: work.id
            });
        }

        Ok(Response::new(resp))
    }

    async fn edit_work(&self, request: Request<ProtoWorkParam>) -> Result<Response<ProtoWork>, Status> {
        //! Proto-func to handle edit_work():
        //!
        //! Request: Index in database, WorkParam, Value
        //!
        //! Response: Work
        
        let req = request.get_ref().clone();
        let index = req.index;
        let enudm = req.r#enum.to_string();
        let value = req.value;

        let a = match enudm.as_str() {
            "0" => {
                edit_work(&self.db, index, WorkParams::Name(value)).await.expect("fail name")
            },
            "1" => {
                edit_work(&self.db, index, WorkParams::Desc(value)).await.expect("fail desc")
            },
            "2" => {
                edit_work(&self.db, index, WorkParams::DateStart(value.parse().unwrap())).await.expect("fail date_start")
            },
            "3" => {
                edit_work(&self.db, index, WorkParams::DateEnd(value.parse().unwrap())).await.expect("fail date_end")
            },
            _ => {Work::new()},
        };

        Ok(Response::new(ProtoWork{ name: a.name, desc: a.desc, date_start: a.date_start, date_end: a.date_end }))
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
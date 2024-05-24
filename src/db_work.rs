use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::{Thing};
use surrealdb::{Surreal};

use crate::work::{Work, WorkParams};

#[derive(Deserialize, Serialize, Debug)]
struct UserData {
    id: Thing,
}
impl UserData {
    pub fn get_id(&self) -> String {
        self.id.id.to_string()
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WorkWithId {
    pub name: String,
    pub desc: String,
    pub date_start: i64,
    pub date_end: i64,
    pub index: String,
}


const WORK_DB_NAME: &str = "work";

pub async fn get_all_works(db: &Surreal<Client>) -> surrealdb::Result<Vec<WorkWithId>> {
    let resp: Vec<WorkWithId> = db.select(WORK_DB_NAME).await?;
    Ok(resp)
}

pub async fn add_work(db: &Surreal<Client>, work: Work) -> surrealdb::Result<String> {
    let created: Vec<UserData> = db.create(WORK_DB_NAME).content(work).await?;
    dbg!(created.first().clone());

    let id = created.first().unwrap().get_id();

    Ok(id)
}

pub async fn edit_work(db: &Surreal<Client>, index: String, param: WorkParams) ->  surrealdb::Result<Work> {
    let mut work = get_work(db, index.clone()).await.expect("Failed to get work");
    work.edit(param);
    let work_upd: Option<Work> = db.update((WORK_DB_NAME, index))
        .content(work)
        .await?;
    
    Ok(work_upd.expect("Fail"))
}

pub async fn get_work(db: &Surreal<Client>, index: String) -> surrealdb::Result<Work> {
    let acquired_work: Option<Work> = db.select((WORK_DB_NAME, index.clone())).await?;

    match acquired_work {
        None => {
            panic!("[ERROR]: Failed to get work by index: {index} from the db.")
        }
        Some(work) => {
            dbg!(work.clone());
            Ok(work)
        }
    }

}
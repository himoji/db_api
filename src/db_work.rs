use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::{Thing};
use surrealdb::{Surreal};
use surrealdb::sql::Uuid;

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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WorkWithId {
    pub name: String,
    pub desc: String,
    pub date_start: i64,
    pub date_end: i64,
    pub id: Thing,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WorkWithoutId {
    pub name: String,
    pub desc: String,
    pub date_start: i64,
    pub date_end: i64,
}

const WORK_DB_NAME: &str = "work";

pub async fn get_all_works(db: &Surreal<Client>) -> surrealdb::Result<Vec<Work>> {
    println!("get all works call db");

    let raw_data: Vec<WorkWithId> = db.select(WORK_DB_NAME).await.expect("vgdggd");
    let mut works: Vec<Work> = Vec::new();

    for val in raw_data {
        works.push(Work::from(val.name, val.desc, val.date_start, val.date_end, val.id.id.to_string()))
    }

    Ok(works)
}

pub async fn add_work(db: &Surreal<Client>, mut work: Work) -> surrealdb::Result<String> {

    let new_work = WorkWithoutId{
        name: work.name,
        desc: work.desc,
        date_start: work.date_start,
        date_end: work.date_end,
    };

   let created: Vec<UserData> = db.create(WORK_DB_NAME).content(new_work).await.expect("Failed to crate the work");

   if let Some(created_work) = created.first() {
       work.id = created_work.get_id();
   } else {
       panic!("No work created");
   }
   Ok(work.id)
}


pub async fn edit_work(db: &Surreal<Client>, index: String, param: WorkParams) ->  surrealdb::Result<Work> {
    let mut work = get_work(db, index.clone()).await.expect("Failed to get work");

    match param {
        WorkParams::Name(name) => {
            work.name = name;
        }
        WorkParams::Desc(desc) => {
            work.desc = desc;
        }
        WorkParams::DateStart(date_start) => {
            work.date_start = date_start;
        }
        WorkParams::DateEnd(date_end) => {
            work.date_end = date_end;
        }
    }
    let work_upd: Option<WorkWithId> = db.update((WORK_DB_NAME, index))
        .content(work.clone())
        .await.expect("Edit [update] fail on db");

    Ok(Work::from(work.name, work.desc, work.date_start, work.date_end, work.id.id.to_string()))
}

pub async fn get_work(db: &Surreal<Client>, index: String) -> surrealdb::Result<WorkWithId> {
    let acquired_work: Option<WorkWithId> = db.select((WORK_DB_NAME, index.clone())).await?;

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
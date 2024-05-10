use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::{Thing};
use surrealdb::{Surreal};

use crate::work::Work;

#[derive(Deserialize, Serialize, Debug)]
struct UserData {
    id: Thing,
}
impl UserData {
    pub fn get_id(&self) -> String {
        self.id.id.to_string()
    }
}

const WORK_DB_NAME: &str = "work";

// pub async fn add_filter_works_vec(db: &Surreal<Client>, vec: Vec<Work>) -> surrealdb::Result<()> {
//     let a = Work::remove_duplicates(get_all_works(db).await?, vec);
//     dbg!(&a);
//
//     add_works_vec(db, a).await?;
//
//     Ok(())
// }
//
pub async fn get_all_works(db: &Surreal<Client>) -> surrealdb::Result<Vec<Work>> {
    let resp: Vec<Work> = db.select(WORK_DB_NAME).await?;
    Ok(resp)
}
//
// pub async fn add_works_vec(db: &Surreal<Client>, vec: Vec<Work>) -> surrealdb::Result<()> {
//     for work in vec {
//         let created: Vec<UserData> = db.create("work").content(work).await?;
//         dbg!(created);
//     }
//     Ok(())
// }

pub async fn add_work(db: &Surreal<Client>, work: Work) -> surrealdb::Result<String> {
    let created: Vec<UserData> = db.create(WORK_DB_NAME).content(work).await?;
    dbg!(created.first().clone());

    let id = created.first().unwrap().get_id();

    Ok(id)
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
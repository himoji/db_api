// use serde::{Deserialize, Serialize};
// use surrealdb::engine::remote::ws::Client;
// use surrealdb::sql::Thing;
// use surrealdb::Surreal;
// use crate::work::Work;
//
// #[derive(Deserialize, Serialize, Debug)]
// struct UserData {
//     id: Thing,
// }
//
// // pub async fn add_filter_works_vec(db: &Surreal<Client>, vec: Vec<Work>) -> surrealdb::Result<()> {
// //     let a = Work::remove_duplicates(get_all_works(db).await?, vec);
// //     dbg!(&a);
// //
// //     add_works_vec(db, a).await?;
// //
// //     Ok(())
// // }
// //
// // pub async fn get_all_works(db: &Surreal<Client>) -> surrealdb::Result<Vec<Work>> {
// //     let mut resp = db.query("select * from work").await?;
// //     let old_vec: Vec<Work> = resp.take(0)?;
// //     Ok(old_vec)
// // }
// //
// // pub async fn add_works_vec(db: &Surreal<Client>, vec: Vec<Work>) -> surrealdb::Result<()> {
// //     for work in vec {
// //         let created: Vec<UserData> = db.create("work").content(work).await?;
// //         dbg!(created);
// //     }
// //     Ok(())
// // }
//
// pub async fn add_work(db: &Surreal<Client>, work: Work) -> surrealdb::Result<()> {
//     let created: Vec<UserData> = db.create("work").content(work).await?;
//     dbg!(created);
//     Ok(())
// }
use std::path::PathBuf;
use scan_folder::lib::*;
use scan_folder::scanning::db::{DirInfoInMongo, get_collection, get_database, get_mongo_client, insert_doc};

#[tokio::main]
async fn main() -> Result<(), ServiceError> {
    println!("Hello, world!");

    let path = PathBuf::from("./src");
    let absolute_path = path.clone().canonicalize().unwrap();
    let folder_info = scan_folder(absolute_path.to_str().unwrap().clone(), true);
    let client = get_mongo_client("mongodb://admin:admin@localhost:27017").await?;
    let db = get_database(client, "rust-scan").await;
    let scan_collection = get_collection(db, "scans").await;
    insert_doc(scan_collection, DirInfoInMongo{
        dir_name: absolute_path.clone().to_str().unwrap().to_string(),
        info: folder_info
    }).await;

    Ok(())
}


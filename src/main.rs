use std::path::PathBuf;

use scan_folder::lib::*;
use scan_folder::scanning::db::{get_collection, get_database, get_mongo_client, insert_doc};

#[tokio::main]
async fn main() -> Result<(), ServiceError> {
    println!("Hello, world!");

    let path = PathBuf::from("./src/utils");
    let absolute_path = path.clone().canonicalize().unwrap();
    let folder_info = scan_folder(absolute_path.to_str().unwrap().clone(), true);
    let client = get_mongo_client("mongodb://admin:admin@localhost:27017").await?;
    let db = get_database(client, "rust-scan").await;
    let scan_collection = get_collection(db.clone(), "scans").await;
    let dir_info = DirInfo {
        dir_name: absolute_path.clone().to_str().unwrap().to_string(),
        info: folder_info.clone(),
    };
    let dir_info2 = DirInfo {
        dir_name: absolute_path.clone().to_str().unwrap().to_string(),
        info: folder_info.clone(),
    };
    let res1 = insert_doc(scan_collection.clone(), dir_info).await;
    // let res2 = insert_doc(scan_collection.clone(), dir_info2,).await;

    Ok(())
}


use std::ptr::slice_from_raw_parts;

use mongodb::{Client, ClientSession, Collection, Database};
use mongodb::bson::doc;
use mongodb::options::{FindOneOptions, FindOptions};
use serde::{Deserialize, Serialize};

use crate::lib::*;
use crate::scanning::models::{DirInfo, DirInfoInMongo};

pub async fn get_mongo_client(uri: &str) -> Result<Client, ServiceError> {
    match Client::with_uri_str(uri).await {
        Ok(client) => Ok(client),
        Err(e) => Err(ServiceError::FailedToCreateDB),
    }
}

pub async fn get_database(client: Client, db_name: &str) -> Database {
    client.database(db_name)
}


pub async fn get_collection(db: Database, collection_name: &str) -> Collection<DirInfoInMongo> {
    db.collection::<DirInfoInMongo>(collection_name)
}

pub async fn insert_doc(coll: Collection<DirInfoInMongo>, doc: DirInfo) -> Result<(),
    ServiceError> {
    let filter = doc! {"dir_info.dir_name": doc.clone().dir_name};
    let find_options = FindOneOptions::builder().sort(doc! {"dir_info.dir_name": -1}).build();
    let document_with_last_version = coll.find_one(filter, find_options).await;

    let last_version = match document_with_last_version {
        Ok(doc) => {
            match doc {
                Some(d) => { d.version }
                None => 0
            }
        }
        Err(_e) => 0
    };

    let dir_info_in_mongo = DirInfoInMongo {
        dir_info: doc,
        version: (last_version + 1) as u64,
    };

    match coll.clone_with_type::<DirInfoInMongo>().insert_one(dir_info_in_mongo, None).await {
        Ok(coll) => Ok(()),
        _ => Err(ServiceError::FailedToFoundCollection),
    }
}
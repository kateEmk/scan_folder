use std::ptr::slice_from_raw_parts;
use mongodb::{Client, ClientSession, Collection, Database};
use serde::{Serialize, Deserialize};
use crate::lib::*;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirInfoInMongo {
    pub dir_info: DirInfo,
    pub version: u64,
}

impl DirInfoInMongo {
    pub fn new(dir_info: DirInfo, version: u64) -> Self {
        DirInfoInMongo {
            dir_info,
            version,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirInfo {
    pub dir_name: String,
    pub info: Vec<DirObjInfo>,
}

impl DirInfo {
    pub fn new(dir_name: String, info: Vec<DirObjInfo>) -> Self {
        DirInfo {
            dir_name,
            info,
        }
    }
}

pub async fn get_mongo_client(uri: &str) -> Result<Client, ServiceError> {
    match Client::with_uri_str(uri).await {
        Ok(client) => Ok(client),
        Err(e) => Err(ServiceError::FailedToCreateDB),
    }
}

pub async fn get_database(client: Client, db_name: &str) -> Database {
    client.database(db_name)
}


pub async fn get_collection(db: Database, collection_name: &str) -> Collection<DirInfoInMongo>{
    db.collection::<DirInfoInMongo>(collection_name)
}

pub async fn insert_doc(coll: Collection<DirInfoInMongo>, doc: DirInfo, db: Database) -> Result<(),
    ServiceError> {
    let collection_reference = coll.clone();

    let collection_exists = match coll.list_index_names().await {
        Ok(collections) => collections.contains(&doc.dir_name.to_owned()),
        Err(_) => false,
    };

    let mut dir_info_in_mongo: DirInfoInMongo;

    let list_collections_names = match db.list_collection_names(None).await {
        Ok(list) => list,
        _ => vec![],
    };

    let collection_count = list_collections_names
        .iter()
        .filter(|&collection_name| collection_name == &doc.dir_name)
        .count();

    if collection_exists {
        dir_info_in_mongo = DirInfoInMongo {
            dir_info: doc,
            version: (collection_count + 1) as u64
        }
    } else {
        dir_info_in_mongo = DirInfoInMongo {
            dir_info: doc,
            version: 0
        }
    }

    match collection_reference.clone_with_type::<DirInfoInMongo>().insert_one(dir_info_in_mongo, None).await {
        Ok(coll) => Ok(()),
        _ => Err(ServiceError::FailedToFoundCollection),
    }
}
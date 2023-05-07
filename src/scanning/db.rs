use mongodb::{Client, Collection, Database};
use mongodb::bson::doc;
use mongodb::options::FindOneOptions;

use crate::lib::*;
use crate::scanning::models::{DirInfo, DirInfoInMongo};

pub async fn get_mongo_client(uri: &str) -> Result<Client, ServiceError> {
    match Client::with_uri_str(uri).await {
        Ok(client) => Ok(client),
        Err(_e) => Err(ServiceError::FailedToCreateDB),
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
    let find_options = FindOneOptions::builder().sort(doc! {"version": -1}).build();
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
        Ok(_coll) => Ok(()),
        _ => Err(ServiceError::FailedToFoundCollection),
    }
}

pub async fn check_if_exists(coll: Collection<DirInfoInMongo>, dir_name: &str) -> bool {
    let filter = doc! {"dir_info.dir_name": dir_name};
    match coll.find_one(filter, FindOneOptions::default()).await {
        Ok(dir) => match dir {
            Some(_d) => true,
            None => false
        },
        _ => false
    }
}

pub async fn get_by_version(coll: Collection<DirInfoInMongo>, dir_name: &str, version: u64) -> Option<DirInfoInMongo> {
    let filter = doc! {"dir_info.dir_name": dir_name, "version": version as f32};
    let document = coll.find_one(filter, FindOneOptions::default()).await;

    match document {
        Ok(doc) => doc,
        _ => None,
    }
}
use mongodb::{Client, ClientSession, Collection, Database};
use serde::{Serialize, Deserialize};
use crate::lib::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirInfoInMongo {
    pub dir_name: String,
    pub info: Vec<DirObjInfo>,
    pub version: u64
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

pub async fn insert_doc(coll: Collection<DirInfoInMongo>, doc: DirInfoInMongo) {
    let collection_reference = coll.clone();
    collection_reference.clone_with_type::<DirInfoInMongo>().insert_one(doc, None).await.expect("");
}
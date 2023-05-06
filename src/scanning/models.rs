use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirObjInfo {
    pub obj_name: String,
    pub is_folder: bool,
    pub size_bytes: u64,
    pub created_at: u64,
    pub last_modified_at: u64,
    pub objects: Vec<DirObjInfo>,
}

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
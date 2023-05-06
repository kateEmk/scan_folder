use std::borrow::Borrow;
use std::ffi::OsStr;
use std::fs;
use std::fs::{DirEntry, File};
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::lib::*;
use crate::scanning::models::DirObjInfo;

pub fn scan_folder(path: &str, is_recursive: bool) -> Vec<DirObjInfo> {
    pub fn _scan(path: &str, is_recursive: bool) -> Vec<DirObjInfo> {
        let mut dir_objects: Vec<DirObjInfo> = vec![];
        for obj_result in fs::read_dir(path.to_string()).unwrap() {
            let obj = obj_result.unwrap();
            let metadata = obj.metadata().unwrap();
            let mut objects: Vec<DirObjInfo> = vec![];
            let obj_file_name = obj.file_name();
            if metadata.is_dir() && is_recursive {
                let dir = format!("{}/{}", path, obj_file_name.to_str().unwrap());
                objects = _scan(dir.as_str(), true);
            }
            let dir_object = DirObjInfo {
                obj_name: obj_file_name.into_string().unwrap(),
                is_folder: metadata.is_dir(),
                size_bytes: metadata.len(),
                created_at: metadata.created().unwrap().duration_since(UNIX_EPOCH).expect("Time went \
            backwards").as_secs(),
                last_modified_at: metadata.modified().unwrap().duration_since(UNIX_EPOCH).expect("Time went \
            backwards").as_secs(),
                objects,
            };
            dir_objects.push(dir_object);
        }
        return dir_objects;
    }
    _scan(path, is_recursive)
}


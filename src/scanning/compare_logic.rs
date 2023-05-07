use std::collections::HashSet;
use std::fmt::Display;
use std::ops::Deref;

use mongodb::Collection;

use crate::lib::*;

pub async fn compare_versions(coll: Collection<DirInfoInMongo>, dir_name: &str, v1: u64, v2: u64) {
    if v1 > v2 {
        println!("First version must be lower than second");
        return;
    }
    if !check_if_exists(coll.clone(), dir_name).await {
        println!("Object with this name doesn't exist.");
        return;
    }

    let v1_doc = match get_by_version(coll.clone(), dir_name.clone(), v1).await {
        Some(doc) => doc,
        _ => {
            println!("Object with this name {} doesn't have {} version doesn't exist.", dir_name.clone(), v1.clone());
            return;
        }
    };

    let v2_doc = match get_by_version(coll.clone(), dir_name, v2).await {
        Some(doc) => doc,
        _ => {
            println!("Object with this name {} doesn't have {} version doesn't exist.", dir_name.clone(), v2.clone());
            return;
        }
    };

    let dir_objects_1 = v1_doc.clone().dir_info.info;
    let dir_objects_2 = v2_doc.clone().dir_info.info;

    compare_names(dir_objects_1, dir_objects_2);
}


fn show_diff_rec(obj: &DirObjInfo, mode: DiffMode, prefix: &str) {
    if obj.objects.len() == 0 {
        println!("{} {}", mode.to_str(), prefix)
    }
    for entry in &obj.objects {
        if entry.is_folder {
            let deep_prefix = format!("{}/{}", prefix, entry.obj_name);
            show_diff_rec(entry, mode.clone(), deep_prefix.as_str())
        } else {
            println!("{} {}/{}", mode.to_str(), prefix, entry.obj_name)
        }
    }
}

fn compare_names(dir_obj_info_1: Vec<DirObjInfo>, dir_obj_info_2: Vec<DirObjInfo>) {
    let dir_obj_names_1 = dir_obj_info_1.clone().into_iter().map(|item| item.obj_name).collect::<HashSet<String>>();
    let dir_obj_names_2 = dir_obj_info_2.clone().into_iter().map(|item| item.obj_name).collect::<HashSet<String>>();

    let mut new_objects = HashSet::new();
    let mut deleted_objects = HashSet::new();

    for obj_v1 in dir_obj_info_1.iter() {
        if !dir_obj_names_2.contains(obj_v1.obj_name.as_str()) {
            deleted_objects.insert(obj_v1);
        }
    }
    for obj_v2 in dir_obj_info_2.iter() {
        if !dir_obj_names_1.contains(obj_v2.obj_name.as_str()) {
            new_objects.insert(obj_v2);
        }
    }
    show_diff(&mut new_objects, DiffMode::New);
    show_diff(&mut deleted_objects, DiffMode::Deleted);
}

fn show_diff(objects: &mut HashSet<&DirObjInfo>, mode: DiffMode) {
    for obj in objects.iter() {
        let mut default_prefix = "";
        if obj.is_folder {
            default_prefix = obj.obj_name.as_str();
        }
        show_diff_rec(obj.deref(), mode.clone(), default_prefix)
    }
}
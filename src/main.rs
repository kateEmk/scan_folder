use std::env;
use std::path::PathBuf;
use std::str::FromStr;

use scan_folder::cli::*;
use scan_folder::lib::*;

#[tokio::main]
async fn main() -> Result<(), ServiceError> {
    let client = get_mongo_client("mongodb://admin:admin@localhost:27017").await.unwrap();
    let db = get_database(client, "rust-scan").await;

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return Err(ServiceError::WrongNumberOfArgs);
    }

    let path = PathBuf::from(&args[2]);
    let absolute_path = path.clone().canonicalize().unwrap();
    let scan_collection = get_collection(db.clone(), "scans").await;

    let command = Command::from_str(args[1].as_str());
    match command {
        Ok(comm) => match comm {
            Command::Scan => {
                if args.len() == 4 {
                    let is_recursive_arg = std::env::args().nth(3);
                    let mut is_recursive = false;
                    if let Some(recursive_arg) = is_recursive_arg {
                        if recursive_arg != "--recursive" {
                            return Err(ServiceError::InvalidArgument);
                        }
                        is_recursive = true;
                    }
                    let dir_object = scan_folder(absolute_path.to_str().unwrap().clone(), is_recursive);
                    let dir_info = DirInfo {
                        dir_name: absolute_path.clone().to_str().unwrap().to_string(),
                        info: dir_object.clone(),
                    };
                    insert_doc(scan_collection, dir_info).await;
                }
                Ok(())
            }
            Command::Compare => {
                if args.len() != 5 {
                    return Err(ServiceError::WrongNumberOfArgs);
                } else {
                    compare_versions(
                        scan_collection,
                        absolute_path.to_str().unwrap().clone(),
                        args[3].parse::<u64>().unwrap(),
                        args[4].parse::<u64>().unwrap(),
                    ).await;
                    Ok(())
                }
            }
        }
        _ => Err(ServiceError::FailedToParseCommand)
    }
}

use std::{env, fs, path::PathBuf};

fn main() -> std::io::Result<()> {
    let folder_names = vec![
        "images".to_string(),
        "songs".to_string(),
        "texts".to_string(),
        "csv".to_string(),
    ];

    let mut folders_found: Vec<String> = Vec::new();
    let mut files_found: Vec<String> = Vec::new();

    let parent_dir = env::current_dir()?;

    let dir = fs::read_dir(&parent_dir).expect("Failed to read directory");

    for entry in dir {
        let entry = entry.expect("Failed to read diretory entry");
        let path = entry.path();

        let path_file_name = path.file_name();

        match path_file_name {
            Some(f) => {
                let file_name_str = f.to_str().expect("File name is suppoed to be a string");
                if !file_name_str.starts_with(".") {
                    if path.is_dir() {
                        folders_found.push(file_name_str.to_string());
                    } else {
                        files_found.push(file_name_str.to_string());
                    }
                }
            }
            _ => {}
        }
    }

    println!("folders to create ({:?})", folder_names);
    println!("folders found ({:?})", folders_found);

    for folder_name in folder_names {
        if !folders_found.contains(&folder_name) {
            let folder_path = parent_dir.join(&folder_name);
            fs::create_dir(&folder_path)?;
            println!("Folder created: {:?}", folder_path);
        }
    }

    for file in files_found {
        let file_path: PathBuf = parent_dir.join(&file);
        let new_file_path: PathBuf;

        if file.ends_with("jpeg") || file.ends_with("jpg") || file.ends_with("png") {
            new_file_path = parent_dir.join("images").join(&file);
            fs::rename(file_path, new_file_path)?;
            println!("File moved successfully")
        } else if file.ends_with(".text") || file.ends_with("txt") {
            new_file_path = parent_dir.join("texts").join(&file);
            fs::rename(file_path, new_file_path)?;
        } else if file.ends_with(".csv") || file.ends_with(".xls") {
            new_file_path = parent_dir.join("csv").join(&file);
            fs::rename(file_path, new_file_path)?;
        } else if file.ends_with(".mp3") {
            new_file_path = parent_dir.join("songs").join(&file);
            fs::rename(file_path, new_file_path)?;
        }
    }

    Ok(())
}

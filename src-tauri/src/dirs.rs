use std::{fs, path};
use crate::{FileDataContainer, NamesData};

pub(crate) fn get_dir_files(path: String) -> FileDataContainer {
 
    let dir_files = get_file_from_current_dir(&path);
    /*    match result_name {
            Err(err)=> { data.err = err.to_string() }
            Ok(ok) => {data.names = ok}
        }*/
    let mut data = FileDataContainer {
        err: "undefined".to_string(),
        dir_files,
    };
    return data;
}


pub(crate) fn get_file_from_current_dir(path: &String) -> Vec<String> {
    let mut dir_list: Vec<String> = vec![];
    let filename = path.split(r"\").collect::<Vec<&str>>()[path.split(r"\").collect::<Vec<&str>>().len() - 1];
    let dirname = path.replace(filename, "");

    println!("{}", dirname.to_string());
    let target = path::PathBuf::from(dirname);

    for dir_entry in fs::read_dir(target).unwrap() {
        // dir_entry(Result<DirEntry, Error>型)をfile_path(PathBuf型)に変換する
        let entry = dir_entry.unwrap();
        if entry.file_type().unwrap().is_file() {
            let file_path: String = entry.file_name().into_string().unwrap();
            /*            let file_path : &str = file_path.unwrap();
            */
            let splited_name = file_path.split(r".").collect::<Vec<&str>>();
            if splited_name.len() > 1 {
                let extension = splited_name[splited_name.len() - 1];
                if extension == "woff2" ||
                    extension == "woff" ||
                    extension == "ttf" ||
                    extension == "otf" {
                    dir_list.push(file_path.to_string());
                }
            }
        }
    }
    return dir_list;
}

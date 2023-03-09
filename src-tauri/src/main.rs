#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
/*#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
*/
use std::{env, path};
use std::borrow::Cow;
use tauri::{Manager};
use window_shadows::set_shadow;
use std::fs;
use std::fs::Metadata;
use std::io::BufRead;
use rusttype::{Font};
use tauri::api::dir::is_dir;

use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
struct FileData {
    args: Vec<String>,
    dir_files: Vec<String>,
}


#[tauri::command]
fn get_args() -> String {
    /*    let path: String = env::args().nth(0).unwrap();
    */
    let mut paths: Vec<String> = env::args().collect();
    println!("{}", paths.join(","));
    let meta = &fs::metadata(r"C:\Users\ym174\OneDrive\デスクトップ\TBGoStdR-C6.otf");

    return paths.join(",");
}


#[tauri::command]
fn get_data() -> FileData {
    /*    let path: String = env::args().nth(0).unwrap();
    */
    let mut args: Vec<String> = env::args().collect();
    let mut dir_list: Vec<String> = vec![];
    let mut path = "";
    if args.len() == 1 {
        path = r"C:\Users\ym174\Desktop\TBGoStdR-C6.otf";
        println!("ok");
    } else {
        path = &*args[1];
    }
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
            */            println!("{:?}", file_path);
            let splited_name = file_path.split(r".").collect::<Vec<&str>>();
            if splited_name.len() > 1 {
                println!("{}", splited_name.len());
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
    println!("{:?}", dir_list);
    return FileData {
        args,
        dir_files: dir_list,
    };
}


/*
fn get_metadata() -> FileMetaData{

}

*/
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            set_shadow(&window, true).expect("Unsupported platform!");
            Ok(())
        })
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);
            app.emit_all("instance_detection", argv.join(",")).unwrap();
        }))
        .invoke_handler(tauri::generate_handler![get_args,get_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


/*fn get_metadata ->
*/
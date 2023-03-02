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
use std::env;
use tauri::{Manager};
use window_shadows::set_shadow;
use std::fs;
use std::fs::Metadata;
use rusttype::{ Font};


struct FileData {
    path: String,
    name: String,

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
fn get_data() -> () {
    /*    let path: String = env::args().nth(0).unwrap();
    */
    let mut args: Vec<String> = env::args().collect();
    let path = r"C:\Users\ym174\OneDrive\デスクトップ\TBGoStdR-C6.otf";

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
        .invoke_handler(tauri::generate_handler![get_args])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/*fn get_metadata ->
*/
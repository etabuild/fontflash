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

#[tauri::command]
fn get_args() -> String {
/*    let path: String = env::args().nth(0).unwrap();
*/
    let paths: Vec<String> = env::args().collect();
    println!("{}", paths.join(","));

    return paths.join(",")
}


fn main() {

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_args])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

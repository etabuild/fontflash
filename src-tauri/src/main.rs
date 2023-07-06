#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

mod font_name;

extern crate core;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
/*#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
*/
use font_name::request_name_data;
use window_vibrancy::{apply_blur, apply_mica, apply_vibrancy, NSVisualEffectMaterial};
use std::{env, path, thread};
use tauri::{AppHandle, Manager, Wry};
use window_shadows::set_shadow;
use std::fs;
use allsorts::binary::read::ReadScope;
use allsorts::error::ParseError;
use allsorts::font_data::FontData;
use allsorts::tables::{FontTableProvider, NameTable, OffsetTable, OpenTypeData};
use allsorts::tag;
use allsorts::woff2::Woff2Font;
use allsorts::woff::WoffFont;
use encoding_rs::{Encoding, MACINTOSH, UTF_16BE};
use serde::{Serialize, Deserialize};
use ttf_parser::Weight;


#[derive(Debug, Serialize, Deserialize, Clone)]
struct FileDataContainer {
    err: String,
    names: Vec<Vec<String>>,
    dir_files: Vec<String>,
}

fn get_path() -> String {
    let args: Vec<String> = env::args().collect();
    return args[1].clone();
}

fn get_dir(path: &String) -> Vec<String> {
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

#[tauri::command]
fn get_font_data_from_args(app: AppHandle<Wry>) -> FileDataContainer{
    let mut args: Vec<String> = env::args().collect();
    return if args.len() < 2 {
        FileDataContainer {
            err: "Couldn't find arg".to_string(),
            names: vec![],
            dir_files: vec![],
        }
    } else {
        app.emit_all("request_detected", args[1].clone()).expect("Couldn't send filepath");

        thread::spawn(move || {
            
        });

        request_name_data(args[1].clone())
    }

}

fn main() {
    /*    woff2();
    */    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();

/*            #[cfg(target_os = "macos")]
            apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
              .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");*/
/*
            #[cfg(target_os = "windows")]
            apply_mica(&window)
              .expect("Unsupported platform! 'apply_blur' is only supported on Windows");*/
            match fs::create_dir("data"){
                Ok(_) => println!("succeeded"),
                Err(_) => println!("failed")
            }
            Ok(())
        })
        /*.on_page_load(|app, window|{
/*            let window = app.get_window("main").unwrap();
*/            set_shadow(&window, true).expect("Unsupported platform!");
            let args: Vec<String> = env::args().collect();
            app.emit_all("init", args).unwrap();

        })*/
        .on_page_load(|window, payload| {
            println!("on page_load {:?}", payload);
            let args: Vec<String> = env::args().collect();
            set_shadow(&window, true).expect("Unsupported platform!");
            window.emit_all("init", args).unwrap();

        })
        .plugin(tauri_plugin_fs_extra::init())
        .plugin(tauri_plugin_single_instance::init(|app, arg, cwd| {
/*            println!("{}, {argv:?}, {cwd}", app.package_info().name);
*//*            let f = get_data(argv[1].clone());
*/
            app.emit_all("file_request", &arg[1]).unwrap();
            let window = app.get_window("main").unwrap();
            /*            send_fontdata("aa".to_string());
            */
            // 最小化されてる場合は解除。
            window.unminimize().expect("Failed to un-minimize!");
            // フォーカスを有効にする。
            window.set_focus().expect("Failed to set-on-top!");
        }))
        .invoke_handler(tauri::generate_handler![get_font_data_from_args])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


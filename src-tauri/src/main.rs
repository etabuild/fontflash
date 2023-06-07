#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

extern crate core;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
/*#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
*/
use std::{env, path, thread, time};
use std::borrow::Cow;
use tauri::{AppHandle, Manager, Wry};
use window_shadows::set_shadow;
use std::fs;
/*use std::fs::File;
*/use std::io::BufRead;
use allsorts::binary::read::ReadScope;
use allsorts::error::ParseError;
use allsorts::font_data::FontData;
use allsorts::tables::{NameTable, OpenTypeData};
use allsorts::tag;
use allsorts::woff2::Woff2Font;
use encoding_rs::{Encoding, MACINTOSH, UTF_16BE};
use tauri::api::dir::is_dir;
use fonttools::font::{self, Font, Table};
use fonttools::name::{name, NameRecord, NameRecordID};
use serde::{Serialize, Deserialize};
use tauri::api::http::FormPart::File;
use ttf_parser::Weight;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct FileData {
    has_patharg: bool,
    err: String,
    filepath: String,
    dir: String,
    dir_files: Vec<String>,
    font_name: Vec<String>,
    font_weight: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct FileDataContainer {
    err: String,
    names: Vec<Vec<String>>,
    dir_files: Vec<String>,
}

#[tauri::command]
fn get_args() -> FileData {
    let mut args: Vec<String> = env::args().collect();
    let _args = args;
    if _args.len() < 2 {
        return FileData {
            has_patharg: false,
            err: "".to_string(),
            filepath: "undefined".to_string(),
            dir: "undefined".to_string(),
            dir_files: vec![],
            font_name: vec![],
            font_weight: 0,
        };
    }

    return get_data(_args[1].clone());
}

fn get_path() -> String {
    let args: Vec<String> = env::args().collect();
    return args[1].clone();
}

/*#[tauri::command]
fn get_data_from_path() -> FileData {

}*/
#[tauri::command]
fn get_data(path: String) -> FileData {
    /*    let _path = path;

    */  println!("gettingdata");
    println!("{}", path.to_string());

    if check_extension(path.clone()) {
        let data = std::fs::read(path.clone()).unwrap();
        println!("aaaaa");
        let face = match ttf_parser::Face::parse(&data, 0) {
            Ok(f) => f,
            Err(e) => {
                eprint!("Error: {}.", e);
                std::process::exit(1);
            }
        };
        let mut family_names = Vec::new();
        for name in face.names() {
            if name.name_id == ttf_parser::name_id::FULL_NAME && name.is_unicode() {
                if let Some(family_name) = name.to_string() {
                    let language = name.language();
                    family_names.push(format!(
                        "{}",
                        family_name,
                        /* language.primary_language(),
                         language.region()*/
                    ));
                }
            }
        }
        let post_script_name = face
            .names()
            .into_iter()
            .find(|name| name.name_id == ttf_parser::name_id::POST_SCRIPT_NAME && name.is_unicode())
            .and_then(|name| name.to_string());

        println!("Family names: {:?}", family_names);
        println!("Weight: {:?}", face.weight());
        println!("PostScript name: {:?}", post_script_name);

        let dir_list = get_dir(path.clone());
        return FileData {
            has_patharg: true,
            err: "".to_string(),
            filepath: path.clone(),
            dir: "undefined".to_string(),

            dir_files: dir_list,
            font_name: family_names,
            font_weight: convert_weight(face.weight()),
        };
    }

    return FileData {
        has_patharg: false,
        err: "".to_string(),
        filepath: "undefined".to_string(),
        dir: "undefined".to_string(),
        dir_files: vec![],
        font_name: vec![],
        font_weight: 0,
    };

    /*    let data = std::fs::read(r"C:\Users\ym174\Desktop\A-OTF-AntiqueStd-AN3.otf").unwrap();
    */


    /*    println!("The font name is {}", font_name);
    */
}

/*fn read_woff2(){
    let font_data = std::fs::read(r"C:\Users\ym174\WebstormProjects\fontflash\src\assets\fonts\NotoSansCJKjp-Black.woff2").unwrap();
    let woff = Woff::parse(&font_data[..]).unwrap();
    println!("Metadata: {:?}", woff.metadata());
}
*/
#[tauri::command]
fn get_filelist(dirpath: String) -> Vec<String> {
    let mut dir_list: Vec<String> = vec![];
    println!("{}", dirpath.to_string());
    let target = path::PathBuf::from(dirpath);
    for dir_entry in fs::read_dir(target).unwrap() {
        // dir_entry(Result<DirEntry, Error>型)をfile_path(PathBuf型)に変換する
        let entry = dir_entry.unwrap();
        if entry.file_type().unwrap().is_file() {
            let file_path: String = entry.file_name().into_string().unwrap();
            /*            let file_path : &str = file_path.unwrap();
            */
            println!("{}", file_path.to_string());
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

fn get_dir(path: String) -> Vec<String> {
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

fn check_extension(path: String) -> bool {
    let filename = path.split(r"\").collect::<Vec<&str>>()[path.split(r"\").collect::<Vec<&str>>().len() - 1];
    let splited_name = filename.split(r".").collect::<Vec<&str>>();
    if splited_name.len() > 1 {
        let extension = splited_name[splited_name.len() - 1];
        if extension == "woff2" ||
            extension == "woff" ||
            extension == "ttf" ||
            extension == "otf" {
            return true;
        }
    }
    return false;
}

/*#[tauri::command]
fn send_fontpath(app_handle: AppHandle){

    let handle = thread::spawn(move || {
        let ten_millis = time::Duration::from_millis(10000);

        thread::sleep(ten_millis);
        println!("Here's a vector: {:?}",path);
    });
}
*/
fn convert_weight(weight: Weight) -> u16 {
    return match weight {
        Weight::Thin => 100,
        Weight::ExtraLight => 200,
        Weight::Light => 300,
        Weight::Normal => 400,
        Weight::Medium => 500,
        Weight::SemiBold => 600,
        Weight::Bold => 700,
        Weight::ExtraBold => 800,
        Weight::Black => 900,
        Weight::Other(value) => value,
        _ => {
            0
        }
    };
}

/*
fn get_metadata() -> FileMetaData{

}

*/
#[tauri::command]
fn get_font_data_from_args(app: AppHandle<Wry>) -> FileDataContainer{
    let mut args: Vec<String> = env::args().collect();
    return if args.len() >= 2 {
        thread::spawn(move || {
            app.emit_all("request_detected", &args[1]).expect("Couldn't send filepath");
        });

        request_name_data(args[1].clone())
    } else {
        FileDataContainer {
            err: "Couldn't find arg".to_string(),
            names: vec![],
            dir_files: vec![],
        }
    }

}

#[tauri::command]
fn request_name_data(path: String) -> FileDataContainer {
    let err_data = FileDataContainer {
        err: "noooooo".to_string(),
        names: vec![],
        dir_files: vec![],
    };
    let mut buffer = match std::fs::read(path) {
        Err(err) => return err_data,
        Ok(ok) => ok
    };
    /*  let buffer;
      match buffer_result {
          Err(err) => Err(err),
          Ok(ok) => Ok(buffer = ok)
      }.expect("aaaamoumuriiiii");*/

    let scope = ReadScope::new(&buffer);

    let mut font_file = match scope.read::<FontData>() {
        Err(err) => return err_data,
        Ok(ok) => ok
    };

    let result_name = match &font_file {
        /*FontData::OpenType(font_file) => match &font_file.data {
            OpenTypeData::Single(ttf) => {
                dump_ttf(&table_provider, &font_file.scope, ttf, table, flags)?
            }
            OpenTypeData::Collection(ttc) => {
                dump_ttc(&table_provider, &font_file.scope, ttc, table, flags)?
            }
        },
        FontData::Woff(woff_file) => dump_woff(woff_file, table, flags)?,*/
        FontData::Woff2(woff_file) => match get_woff2_name(woff_file) {
            Ok(ok) => ok,
            Err(_) => return err_data
        },
        _ => vec![],
    };
    /*    match result_name {
            Err(err)=> { data.err = err.to_string() }
            Ok(ok) => {data.names = ok}
        }*/
    let mut data = FileDataContainer {
        err: "undefined".to_string(),
        names: result_name,
        dir_files: vec![],
    };
    return data;
}

fn get_woff2_name(woff: &Woff2Font) -> Result<Vec<Vec<String>>, ParseError> {
    if let Ok(Some(table)) = woff.read_table(tag::NAME, 0) {
        println!();
        let name_table = table.scope().read::<NameTable>()?;
        Ok::<Result<Vec<Vec<String>>, ParseError>, ParseError>(dump_name_table(&name_table))?
    } else {
        return Ok(vec![]);
    }
}

fn dump_name_table(name_table: &NameTable) -> Result<Vec<Vec<String>>, ParseError> {
    let mut names: Vec<Vec<String>> = vec![];

    for name_record in &name_table.name_records {
        let mut pair: Vec<String> = vec!["".to_string(),"".to_string()];
        let platform = name_record.platform_id;
        let encoding = name_record.encoding_id;
        let language = name_record.language_id;
        let offset = usize::from(name_record.offset);
        let length = usize::from(name_record.length);
        let name_data = name_table
            .string_storage
            .offset_length(offset, length)?
            .data();
        let name = match (platform, encoding, language) {
            (0, _, _) => decode(UTF_16BE, name_data),
            (1, 0, _) => decode(MACINTOSH, name_data),
            (3, 0, _) => decode(UTF_16BE, name_data),
            (3, 1, _) => decode(UTF_16BE, name_data),
            (3, 10, _) => decode(UTF_16BE, name_data),
            _ => format!(
                "(unknown platform={} encoding={} language={})",
                platform, encoding, language
            ),
        };
        match get_name_meaning(name_record.name_id) {
            Some(meaning) => pair[0] = meaning.to_string(),
            None => pair[0] = "".to_string(),
        }
        println!("{:?}", name);
        pair[1] = name;
        println!();
        names.push(pair);
    }

    Ok(names)
}

fn decode(encoding: &'static Encoding, data: &[u8]) -> String {
    let mut decoder = encoding.new_decoder();
    if let Some(size) = decoder.max_utf8_buffer_length(data.len()) {
        let mut s = String::with_capacity(size);
        let (_res, _read, _repl) = decoder.decode_to_string(data, &mut s, true);
        s
    } else {
        String::new() // can only happen if buffer is enormous
    }
}

fn get_name_meaning(name_id: u16) -> Option<&'static str> {
    match name_id {
        0 => Some("Copyright"),
        1 => Some("Font Family"),
        2 => Some("Font Subfamily"),
        3 => Some("Unique Identifier"),
        4 => Some("Full Font Name"),
        5 => Some("Version"),
        6 => Some("PostScript Name"),
        7 => Some("Trademark"),
        8 => Some("Manufacturer"),
        9 => Some("Designer"),
        10 => Some("Description"),
        11 => Some("URL Vendor"),
        12 => Some("URL Designer"),
        13 => Some("License Description"),
        14 => Some("License Info URL"),
        15 => None, // Reserved
        16 => Some("Typographic Family"),
        17 => Some("Typographic Subfamily"),
        18 => Some("Compatible Full"),
        19 => Some("Sample Text"),
        20 => Some("PostScript CID findfont"),
        21 => Some("WWS Family Name"),
        22 => Some("WWS Subfamily Name"),
        23 => Some("Light Background Palette"),
        24 => Some("Dark Background Palette"),
        25 => Some("Variations PostScript Name Prefix"),
        _ => None,
    }
}

fn main() {
    /*    woff2();
    */    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            set_shadow(&window, true).expect("Unsupported platform!");
            let mut args: Vec<String> = env::args().collect();
            println!("{:?}", args);
            app.emit_all("init", args).unwrap();
            Ok(())
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
        .invoke_handler(tauri::generate_handler![get_args,get_data,get_filelist,request_name_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


use std::{env, fs};
use allsorts::binary::read::ReadScope;
use allsorts::error::ParseError;
use allsorts::font_data::FontData;
use allsorts::tables::{NameTable, OffsetTable, OpenTypeData, TTCHeader};
use allsorts::tag;
use allsorts::woff2::Woff2Font;
use allsorts::woff::WoffFont;
use encoding_rs::{Encoding, MACINTOSH, UTF_16BE};
use tauri::{AppHandle, Manager, Wry};
use crate::{FileDataContainer, get_dir};
pub(crate) fn request_name_data(path: String) -> FileDataContainer {
    let err_data = FileDataContainer {
        err: "noooooo".to_string(),
        names: vec![],
        dir_files: vec![],
    };
    let mut buffer = match fs::read(path.clone()) {
        Err(err) => return err_data,
        Ok(ok) => ok
    };
    /*  let buffer;
      match buffer_result {
          Err(err) => Err(err),
          Ok(ok) => Ok(buffer = ok)
      }.expect("aaaamoumuriiiii");*/

    let scope = ReadScope::new(&buffer);

    let font_file = match scope.read::<FontData>() {
        Err(err) => return err_data,
        Ok(ok) => ok
    };

    let result_name = match &font_file {
        /*  FontData::OpenType(font_file) => match &font_file.data {
              OpenTypeData::Single(ttf) => {
                  dump_ttf(&table_provider, &font_file.scope, ttf, table, flags)?
              }
              OpenTypeData::Collection(ttc) => {
                  dump_ttc(&table_provider, &font_file.scope, ttc, table, flags)?
              }
          },*/
        FontData::OpenType(font_file) => match &font_file.data{
            OpenTypeData::Single(ttf) => match get_ttf_name(&font_file.scope, ttf){
                Ok(ok) => ok,
                Err(_) => return err_data
            },
            OpenTypeData::Collection(ttc) => match get_ttc_name(&font_file.scope, ttc){
                Ok(ok) => ok,
                Err(_) => return err_data
            }

        },
        FontData::Woff(woff_file) => match get_woff_name(woff_file){
            Ok(ok) => ok,
            Err(_) => return err_data
        },
        FontData::Woff2(woff_file) => match get_woff2_name(woff_file) {
            Ok(ok) => ok,
            Err(_) => return err_data
        },
        _ => vec![],
    };
    let dir_files = get_dir(&path);
    /*    match result_name {
            Err(err)=> { data.err = err.to_string() }
            Ok(ok) => {data.names = ok}
        }*/
    let mut data = FileDataContainer {
        err: "undefined".to_string(),
        names: result_name,
        dir_files
    };
    return data;
}


fn get_ttc_name<'a>(scope:&ReadScope<'a>,ttc:&TTCHeader<'a>) -> Result<Vec<Vec<String>>,ParseError>{
    for offset_table_offset in &ttc.offset_tables {
        let offset_table_offset = usize::try_from(offset_table_offset).map_err(ParseError::from)?;
        let offset_table = scope.offset(offset_table_offset).read::<OffsetTable>()?;
        get_ttf_name(scope, &offset_table)?;
    }
    return Ok(vec![])
}

fn get_ttf_name<'a>(scope: &ReadScope<'a>,ttf:&OffsetTable<'a>) -> Result<Vec<Vec<String>>, ParseError>{
    return Ok(vec![])

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

fn get_woff_name(woff: &WoffFont) -> Result<Vec<Vec<String>>, ParseError> {
    if let Some(entry) =woff
        .table_directory
        .iter()
        .find(|entry| entry.tag == tag::NAME)
    {
        let table = entry.read_table(&woff.scope)?;
        let name_table = table.scope().read::<NameTable>()?;
        dump_name_table(&name_table)?;
        Ok::<Result<Vec<Vec<String>>, ParseError>, ParseError>(dump_name_table(&name_table))?
    }else{
        return Ok(vec![])
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

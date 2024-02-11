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
use crate::{FileDataContainer, NamesData};
use crate::dirs::get_file_from_current_dir;


pub(crate) fn get_name_records(path: String) -> Result<Option<NamesData>, String> {

    let mut buffer = match fs::read(path.clone()) {
        Err(err) => return Err(err.to_string()),
        Ok(ok) => ok
    };

    let scope = ReadScope::new(&buffer);

    let font_file = match scope.read::<FontData>() {
        Err(err) => return Err(err.to_string()),
        Ok(ok) => ok
    };

    let result_name = match &font_file {
        FontData::OpenType(font_file) => match &font_file.data {
            OpenTypeData::Single(ttf) => match get_ttf_name(&font_file.scope, ttf) {
                Ok(Some(data)) => Ok(Some(data)),
                Ok(None) => Ok(None),
                Err(err) =>Err(err.to_string())
            },
            _ => return Ok(None)
        },
        FontData::Woff(woff_file) => match get_woff_name(woff_file) {
            Ok(Some(data)) => Ok(Some(data)),
                Ok(None) => Ok(None),
                Err(err) =>Err(err.to_string())
        },
        FontData::Woff2(woff_file) => match get_woff2_name(woff_file) {
            Ok(Some(data)) => Ok(Some(data)),
                Ok(None) => Ok(None),
                Err(err) =>Err(err.to_string()),
        _ => return Ok(None),
        }
    };
    result_name
}
/* 
fn get_ttc_name<'a>(scope: &ReadScope<'a>, ttc: &TTCHeader<'a>) -> Result<Option<NamesData>, ParseError> {
    for offset_table_offset in &ttc.offset_tables {
        let offset_table_offset = usize::try_from(offset_table_offset).map_err(ParseError::from)?;
        let offset_table = scope.offset(offset_table_offset).read::<OffsetTable>()?;
        get_ttf_name(scope, &offset_table)?;
    }
    return Ok(vec![]);
} */

fn get_ttf_name<'a>(scope: &ReadScope<'a>, ttf: &OffsetTable<'a>) -> Result<Option<NamesData>, ParseError> {
    match ttf.read_table(scope, tag::NAME) {
        Ok(Some(name_table_data))=> {
            let name_table = name_table_data.read::<NameTable>()?;
            match dump_name_table(&name_table) {
                Ok(data) => Ok(Some(data)),
                Err(err) => Err(err)
            }
        },
        Ok(None) => Ok(None),
        Err(err) => Err(err)

    }

}

fn get_woff2_name(woff: &Woff2Font) -> Result<Option<NamesData>, ParseError>  {
    match woff.read_table(tag::NAME, 0) {
        Ok(Some(table)) =>  {
              let name_table = table.scope().read::<NameTable>()?;
              match dump_name_table(&name_table) {
                Ok(data) => Ok(Some(data)),
                Err(err) => Err(err)
            }
   }
        Ok(None) => Ok(None),
        Err(err) => Ok(None)
    }
   
}

fn get_woff_name(woff: &WoffFont) -> Result<Option<NamesData>,ParseError> {
    match woff
    .table_directory
    .iter()
    .find(|entry| entry.tag == tag::NAME){
        Some(entry) => {
            let table = entry.read_table(&woff.scope)?;
            let name_table = table.scope().read::<NameTable>()?;
            match dump_name_table(&name_table) {
                Ok(data) => Ok(Some(data)),
                Err(err) => Err(err)
            }         },
        None => Ok(None)
    }
 
}

fn dump_name_table(name_table: &NameTable) -> Result<NamesData, ParseError> {
    let mut names: Vec<Vec<String>> = vec![];
    let mut names_data = NamesData{
        error: false,
        copyright: vec![],
        font_family: vec![],
        font_subfamily: vec![],
        unique_identifier: vec![],
        full_font_name: vec![],
        version: vec![],
        postscript_name: vec![],
        trademark: vec![],
        manufacturer: vec![],
        designer: vec![],
        description: vec![],
        url_vendor: vec![],
        url_designer: vec![],
        license_description: vec![],
        license_info_url: vec![],
        typographic_family:vec![],
        typographic_subfamily: vec![],
        compatible_full: vec![],
        sample_text: vec![],
        postscript_cid_findfont: vec![],
        wws_family_name: vec![],
        wws_subfamily_name: vec![],
        light_background_palette: vec![],
        dark_background_palette: vec![],
        variations_postscript_name_prefix: vec![],
        others: vec![]
        
    };
    for name_record in &name_table.name_records {
        let mut pair: Vec<String> = vec!["".to_string(), "".to_string()];
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
            (1, 1, 11) => decode(MACINTOSH, name_data),
            _ => format!(
                "(unknown platform={} encoding={} language={})",
                platform, encoding, language
            ),
        };
/*         match get_name_mmeaning(namesData ,name_record.name_id, name) {
            Some(meaning) => {
                pair[0] = meaning.to_string();

            }

            None => pair[0] = "".to_string(),
        }
        
 */
            names_data = get_name_mmeaning(names_data ,name_record.name_id, &name);
        /*        println!("{:?}", name);
        */        pair[1] = name;
        /*        println!();

        */        names.push(pair);
    }


    Ok(names_data)
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
        0 => Some("CopyRight"),
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

fn get_name_mmeaning(mut record_data: NamesData, name_id: u16, namea: &String) ->NamesData {
    let name = namea.to_string();
    match name_id {
        0 => record_data.copyright.push(name),
        1 => record_data.font_family.push(name),
        2 => record_data.font_subfamily.push(name),
        3 => record_data.unique_identifier.push(name),
        4 => record_data.full_font_name.push(name),
        5 => record_data.version.push(name),
        6 => record_data.postscript_name.push(name),
        7 => record_data.trademark.push(name),
        8 => record_data.manufacturer.push(name),
        9 => record_data.designer.push(name),
        10 => record_data.description.push(name),
        11 => record_data.url_vendor.push(name),
        12 => record_data.url_designer.push(name),
        13 => record_data.license_description.push(name),
        14 => record_data.license_info_url.push(name),
        15 => record_data.others.push(name),
        16 => record_data.typographic_family.push(name),
        17 => record_data.typographic_subfamily.push(name),
        18 => record_data.compatible_full.push(name),
        19 => record_data.sample_text.push(name),
        20 => record_data.postscript_cid_findfont.push(name),
        21 => record_data.wws_family_name.push(name),
        22 => record_data.wws_subfamily_name.push(name),
        23 => record_data.light_background_palette.push(name),
        24 => record_data.dark_background_palette.push(name),
        25 => record_data.variations_postscript_name_prefix.push(name),
        _ => record_data.others.push(name),
    }
    return record_data
}



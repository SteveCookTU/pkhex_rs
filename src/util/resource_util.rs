use crate::RESOURCES;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type StringListCache = Arc<Mutex<HashMap<String, Arc<Vec<String>>>>>;
lazy_static! {
    static ref STRING_LIST_CACHE: StringListCache = Arc::new(Mutex::new(HashMap::new()));
}

pub fn get_species_list(language: impl AsRef<str>) -> Arc<Vec<String>> {
    get_string_list_from_param("species", language, None)
}

pub fn get_moves_list(language: impl AsRef<str>) -> Arc<Vec<String>> {
    get_string_list_from_param("moves", language, None)
}

pub fn get_abilities_list(language: impl AsRef<str>) -> Arc<Vec<String>> {
    get_string_list_from_param("abilities", language, None)
}

pub fn get_natures_list(language: impl AsRef<str>) -> Arc<Vec<String>> {
    get_string_list_from_param("natures", language, None)
}

pub fn get_forms_list(language: impl AsRef<str>) -> Arc<Vec<String>> {
    get_string_list_from_param("forms", language, None)
}

pub fn get_types_list(language: impl AsRef<str>) -> Arc<Vec<String>> {
    get_string_list_from_param("types", language, None)
}

pub fn get_characteristics_list(language: impl AsRef<str>) -> Arc<Vec<String>> {
    get_string_list_from_param("character", language, None)
}

pub fn get_items_list(language: impl AsRef<str>) -> Arc<Vec<String>> {
    get_string_list_from_param("items", language, None)
}

pub fn get_language_strings_7(file_name: impl AsRef<str>) -> Vec<Arc<Vec<String>>> {
    let blank = Arc::new(vec![]);
    vec![
        blank.clone(),
        get_string_list_from_param(file_name.as_ref(), "ja", None),
        get_string_list_from_param(file_name.as_ref(), "en", None),
        get_string_list_from_param(file_name.as_ref(), "fr", None),
        get_string_list_from_param(file_name.as_ref(), "it", None),
        get_string_list_from_param(file_name.as_ref(), "de", None),
        blank,
        get_string_list_from_param(file_name.as_ref(), "es", None),
    ]
}

pub fn get_language_strings_8(file_name: impl AsRef<str>) -> Vec<Arc<Vec<String>>> {
    let blank = Arc::new(vec![]);
    vec![
        blank.clone(),
        get_string_list_from_param(file_name.as_ref(), "ja", None),
        get_string_list_from_param(file_name.as_ref(), "en", None),
        get_string_list_from_param(file_name.as_ref(), "fr", None),
        get_string_list_from_param(file_name.as_ref(), "it", None),
        get_string_list_from_param(file_name.as_ref(), "de", None),
        blank,
        get_string_list_from_param(file_name.as_ref(), "es", None),
        get_string_list_from_param(file_name.as_ref(), "ko", None),
    ]
}

pub fn get_language_strings_10(
    file_name: impl AsRef<str>,
    zh2: Option<&str>,
) -> Vec<Arc<Vec<String>>> {
    let blank = Arc::new(vec![]);
    vec![
        blank.clone(),
        get_string_list_from_param(file_name.as_ref(), "ja", None),
        get_string_list_from_param(file_name.as_ref(), "en", None),
        get_string_list_from_param(file_name.as_ref(), "fr", None),
        get_string_list_from_param(file_name.as_ref(), "it", None),
        get_string_list_from_param(file_name.as_ref(), "de", None),
        blank,
        get_string_list_from_param(file_name.as_ref(), "es", None),
        get_string_list_from_param(file_name.as_ref(), "ko", None),
        get_string_list_from_param(file_name.as_ref(), "zh", None),
        get_string_list_from_param(file_name.as_ref(), zh2.unwrap_or("zh"), None),
    ]
}

pub fn get_string_list(file_name: impl AsRef<str>) -> Arc<Vec<String>> {
    let mut out = Arc::new(Vec::new());
    if is_string_list_cached(file_name.as_ref(), &mut out) {
        return out;
    }
    let txt = get_string_resource(file_name.as_ref());
    load_string_list(file_name, txt)
}

pub fn is_string_list_cached(file_name: impl AsRef<str>, out: &mut Arc<Vec<String>>) -> bool {
    if let Some(list) = STRING_LIST_CACHE.lock().unwrap().get(file_name.as_ref()) {
        *out = list.clone();
        true
    } else {
        false
    }
}

pub fn load_string_list(file_name: impl AsRef<str>, txt: String) -> Arc<Vec<String>> {
    if txt.is_empty() {
        Arc::new(vec![])
    } else {
        let raw = Arc::new(fast_split(&txt.chars().collect::<Vec<_>>()));
        let mut lock = STRING_LIST_CACHE.lock().unwrap();
        if !lock.contains_key(file_name.as_ref()) {
            lock.insert(file_name.as_ref().to_string(), raw.clone());
        }
        raw
    }
}

pub fn get_string_list_from_param(
    file_name: impl AsRef<str>,
    lang_2_char: impl AsRef<str>,
    data_type: Option<&str>,
) -> Arc<Vec<String>> {
    get_string_list(get_full_resource_name(
        file_name,
        lang_2_char,
        data_type.unwrap_or("text"),
    ))
}

fn get_full_resource_name(
    file_name: impl AsRef<str>,
    lang_2_char: impl AsRef<str>,
    data_type: impl AsRef<str>,
) -> String {
    String::new() + data_type.as_ref() + "_" + file_name.as_ref() + "_" + lang_2_char.as_ref()
}

pub fn get_bin_resource(name: impl AsRef<str>) -> Option<&'static [u8]> {
    let glob = "**/*";
    if let Ok(files) = RESOURCES.find(glob) {
        for file in files {
            if let Some(file) = file.as_file() {
                if file
                    .path()
                    .file_name()
                    .map(|s| s.to_str())
                    .unwrap()
                    .unwrap()
                    .to_lowercase()
                    .contains(&name.as_ref().to_lowercase())
                {
                    return Some(file.contents());
                }
            }
        }
    }
    None
}

pub fn get_string_resource(name: impl AsRef<str>) -> String {
    let glob = "**/*.txt";
    if let Ok(files) = RESOURCES.find(glob) {
        for file in files {
            if let Some(file) = file.as_file() {
                if file
                    .path()
                    .file_name()
                    .map(|s| s.to_str())
                    .unwrap()
                    .unwrap()
                    .to_lowercase()
                    .contains(&name.as_ref().to_lowercase())
                {
                    return if let Some(str) = file.contents_utf8() {
                        str.to_string()
                    } else {
                        let utf_16_packets = file
                            .contents()
                            .chunks(2)
                            .map(|e| u16::from_le_bytes(e.try_into().unwrap()))
                            .collect::<Vec<_>>();
                        String::from_utf16_lossy(&utf_16_packets)
                    };
                }
            }
        }
    }
    String::new()
}

fn fast_split(mut s: &[char]) -> Vec<String> {
    if s.is_empty() {
        return vec![];
    }

    let count = get_count(s);

    let mut result = Vec::with_capacity(count);
    let mut i = 0;
    loop {
        let index = s.iter().position(|&c| c == '\n').unwrap_or(s.len());
        let mut slice = &s[..index];
        if !slice.is_empty() && slice.last().unwrap() == &'\r' {
            slice = &slice[..slice.len() - 1]
        }

        result.push(slice.iter().copied().collect::<String>());
        i += 1;
        if i == count {
            return result;
        }
        s = &s[(index + 1)..];
    }
}

fn get_count(mut s: &[char]) -> usize {
    let mut count = 1;
    loop {
        let index = s.iter().position(|&c| c == '\n');
        if let Some(index) = index {
            count += 1;
            s = &s[(index + 1)..];
        } else {
            return count;
        }
    }
}

use crate::game::enums::LanguageID;
use crate::resource_util;
use lazy_static::lazy_static;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

type RegionList = Arc<Mutex<Vec<Arc<Vec<Vec<&'static str>>>>>>;

lazy_static! {
    static ref COUNTRY_LIST: Vec<Vec<&'static str>> = _get_country_list();
    static ref REGION_LIST: RegionList = Arc::new(Mutex::new({
        let mut v = Vec::with_capacity(COUNTRY_LIST.len());
        (0..COUNTRY_LIST.len()).for_each(|_| v.push(Arc::new(vec![])));
        v
    }));
}

pub(crate) const LANG_GEO: [&str; 9] = ["ja", "en", "fr", "de", "it", "es", "zh", "ko", "zh2"];

pub fn get_language_index(language: impl AsRef<str>) -> Option<usize> {
    LANG_GEO.iter().position(|&s| s == language.as_ref())
}

fn _get_language_index_from_id(language: LanguageID) -> Option<usize> {
    get_language_index(language.get_language_2_char_name())
}

const INVALID: &str = "INVALID";

fn _get_country_list() -> Vec<Vec<&'static str>> {
    let input = resource_util::get_string_list("countries");
    unpack_list(&input)
}

fn _get_region_list(country: usize) -> Vec<Vec<&'static str>> {
    let input = resource_util::get_string_list(format!("sr_{country:0>3}"));
    unpack_list(&input)
}

fn unpack_list(input: &[&'static str]) -> Vec<Vec<&'static str>> {
    let mut last_index = 0;
    let last = get_entry(input.last().unwrap(), &mut last_index);
    let mut list = vec![vec![]; last_index + 1];
    list[last_index] = last;
    for line in input {
        let mut index = 0;
        let entry = get_entry(line, &mut index);
        list[index] = entry;
    }
    list
}

fn get_entry(line: &'static str, index: &mut usize) -> Vec<&'static str> {
    let entries = line.split(',').collect::<Vec<_>>();
    *index = usize::from_str(entries[0]).unwrap_or_default();
    entries
}

fn _get_country_name(country: usize, l: Option<usize>) -> &'static str {
    if l.is_none() {
        return INVALID;
    }
    let l = l.unwrap();
    if country >= COUNTRY_LIST.len() {
        return INVALID;
    }
    let country_names = &COUNTRY_LIST[country];
    if !country_names.is_empty() && l < country_names.len() - 1 {
        return country_names[l + 1];
    }
    INVALID
}

fn _get_region_name(country: usize, region: usize, l: Option<usize>) -> &'static str {
    let mut lock = REGION_LIST.lock().unwrap();

    if l.is_none() {
        return INVALID;
    }
    let l = l.unwrap();
    if country >= lock.len() {
        return INVALID;
    }
    if lock[country].is_empty() {
        lock[country] = Arc::new(_get_region_list(country));
    }
    let region_names = &lock[country];
    if region >= region_names.len() {
        return INVALID;
    }
    let localized = &region_names[region];
    if !localized.is_empty() && l < localized.len() - 1 {
        return localized[l + 1];
    }
    INVALID
}

pub fn get_country_list(language: impl AsRef<str>) -> &'static [&'static str] {
    let index = get_language_index(language).unwrap_or_default();
    &COUNTRY_LIST[index]
}

pub fn get_country_name(language: impl AsRef<str>, country: usize) -> &'static str {
    _get_country_name(country, get_language_index(language))
}

pub fn get_region_name(language: impl AsRef<str>, country: usize, region: usize) -> &'static str {
    _get_region_name(country, region, get_language_index(language))
}

pub fn get_country_name_from_id(language: LanguageID, country: usize) -> &'static str {
    _get_country_name(country, _get_language_index_from_id(language))
}

pub fn get_region_name_from_id(
    language: LanguageID,
    country: usize,
    region: usize,
) -> &'static str {
    _get_region_name(country, region, _get_language_index_from_id(language))
}

pub fn get_country_region_text(
    country: usize,
    region: usize,
    language: impl AsRef<str>,
) -> (&'static str, &'static str) {
    let lang = LANG_GEO.iter().position(|&s| s == language.as_ref());
    let country_name = _get_country_name(country, lang);
    let region_name = _get_region_name(country, region, lang);
    (country_name, region_name)
}

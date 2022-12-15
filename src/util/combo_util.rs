use crate::game::enums::Ball;
use crate::game::game_strings::geo_location;
use crate::game::ComboItem;
use crate::resource_util;
use std::cmp::Ordering;
use std::str::FromStr;

fn comparer(a: &ComboItem<impl AsRef<str>>, b: &ComboItem<impl AsRef<str>>) -> Ordering {
    a.text.as_ref().cmp(b.text.as_ref())
}

pub fn get_country_region_list(text_file: &str, lang: impl AsRef<str>) -> Vec<ComboItem<String>> {
    let input_csv = resource_util::get_string_list(text_file);
    let index = geo_location::get_language_index(lang.as_ref()).unwrap_or_default();
    let mut list = get_cb_list_from_csv(&input_csv, index);
    list.sort_by(comparer);
    list
}

fn get_cb_list_from_csv(input_csv: &[&'static str], index: usize) -> Vec<ComboItem<String>> {
    let mut arr = Vec::with_capacity(input_csv.len());
    for &line in input_csv {
        let text = line.split(',').skip(4).nth(index).unwrap_or(&line[5..]);
        let value = &line[..3];
        let item = ComboItem::new(text.to_string(), i32::from_str(value).unwrap_or_default());
        arr.push(item);
    }
    arr
}

pub fn get_cb_list(strings: &[&str]) -> Vec<ComboItem<String>> {
    let mut list = Vec::with_capacity(strings.len());
    for (i, &string) in strings.iter().enumerate() {
        list.push(ComboItem::new(string.to_string(), i as i32));
    }
    list.sort_by(comparer);
    list
}

pub fn get_cb_list_allowed<'a>(strings: &[&'a str], allowed: &[u16]) -> Vec<ComboItem<String>> {
    let mut list = Vec::with_capacity(allowed.len() + 1);
    list.push(ComboItem::new(strings[0].to_string(), 0));
    for &i in allowed {
        list.push(ComboItem::new(strings[i as usize].to_string(), i as i32));
    }
    list.sort_by(comparer);
    list
}

pub fn get_cb_list_from_offset(
    strings: &[&str],
    index: usize,
    offset: Option<usize>,
) -> Vec<ComboItem<String>> {
    let offset = offset.unwrap_or_default();
    let mut list = Vec::new();
    add_cb_with_offset_index(&mut list, strings, offset, index);
    list
}

pub fn get_unsorted_cb_list<'a>(strings: &[&'a str], allowed: &[u8]) -> Vec<ComboItem<String>> {
    let mut list = Vec::with_capacity(allowed.len() + 1);
    list.push(ComboItem::new(strings[0].to_string(), 0));
    for &i in allowed {
        list.push(ComboItem::new(strings[i as usize].to_string(), i as i32));
    }

    list
}

pub fn add_cb_with_offset_index<'a>(
    list: &mut Vec<ComboItem<String>>,
    strings: &[&'a str],
    offset: usize,
    index: usize,
) {
    let item = ComboItem::new(strings[index - offset].to_string(), index as i32);
    list.push(item);
}

pub fn add_cb_with_offset_allowed_small<'a>(
    list: &mut Vec<ComboItem<String>>,
    strings: &[&'a str],
    offset: usize,
    allowed: &[u8],
) {
    let begin_count = list.len();
    for &index in allowed {
        let item = ComboItem::new(strings[index as usize - offset].to_string(), index as i32);
        list.push(item);
    }
    list[begin_count..(begin_count + allowed.len())].sort_by(comparer);
}

pub fn add_cb_with_offset_allowed<'a>(
    list: &mut Vec<ComboItem<String>>,
    strings: &[&'a str],
    offset: usize,
    allowed: &[u16],
) {
    let begin_count = list.len();
    for &index in allowed {
        let item = ComboItem::new(strings[index as usize - offset].to_string(), index as i32);
        list.push(item);
    }
    list[begin_count..(begin_count + allowed.len())].sort_by(comparer);
}

pub fn add_cb_with_offset<'a>(
    list: &mut Vec<ComboItem<String>>,
    strings: &[&'a str],
    offset: usize,
) {
    let begin_count = list.len();
    for (i, &index) in strings.iter().enumerate() {
        let item = ComboItem::new(index.to_string(), (i + offset) as i32);
        list.push(item);
    }
    list[begin_count..(begin_count + strings.len())].sort_by(comparer);
}

pub fn get_varied_cb_list_ball(
    strings: &[&str],
    string_num: &[u16],
    string_val: &[u8],
) -> Vec<ComboItem<String>> {
    const FORCED_TOP: usize = 3;
    let mut list = Vec::with_capacity(FORCED_TOP + string_num.len());
    list.push(ComboItem::new(strings[4].to_string(), Ball::Poke as i32));
    list.push(ComboItem::new(strings[3].to_string(), Ball::Great as i32));
    list.push(ComboItem::new(strings[2].to_string(), Ball::Ultra as i32));

    for (i, &num) in string_num.iter().enumerate() {
        let val = string_val[i];
        let text = strings[num as usize];
        list.push(ComboItem::new(text.to_string(), val as i32));
    }

    list[3..].sort_by(comparer);

    list
}

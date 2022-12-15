use crate::game::enums::GameVersion;
use crate::game::game_strings::{game_language, GameDataSource, GameStrings};
use crate::game::ComboItem;
use crate::pkm::shared::EntityContext;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref LANGUAGES: Arc<Mutex<Vec<Option<Arc<GameStrings>>>>> =
        Arc::new(Mutex::new(vec![None; game_language::LANGUAGE_COUNT]));
    pub static ref CURRENT_LANGUAGE: Arc<Mutex<String>> =
        Arc::new(Mutex::new(game_language::DEFAULT_LANGUAGE.to_string()));
    static ref _STRINGS: Arc<Mutex<Arc<GameStrings>>> = Arc::new(Mutex::new(
        get_strings_from_lang(CURRENT_LANGUAGE.lock().unwrap().as_str())
    ));
}

lazy_static! {
    static ref SOURCES: Arc<Mutex<GameDataSource>> = Arc::new(Mutex::new(GameDataSource::new(
        _STRINGS.lock().unwrap().clone()
    )));
}

pub const GENDER_SYMBOL_UNICODE: [&str; 3] = ["♂", "♀", "-"];
pub const GENDER_SYMBOL_ASCII: [&str; 3] = ["M", "F", "-"];

pub fn get_strings_from_lang(lang: impl AsRef<str>) -> Arc<GameStrings> {
    let index = game_language::get_language_index(lang.as_ref());
    get_strings(index)
}

pub fn get_strings(index: usize) -> Arc<GameStrings> {
    let mut lock = LANGUAGES.lock().unwrap();
    if let Some(strings) = &lock[index] {
        strings.clone()
    } else {
        let strings = GameStrings::new(game_language::language_2_char(index));
        lock[index] = Some(strings.clone());
        strings
    }
}

pub fn strings() -> Arc<GameStrings> {
    _STRINGS.lock().unwrap().clone()
}

pub fn set_strings(strings: Arc<GameStrings>) {
    *_STRINGS.lock().unwrap() = strings.clone();
    set_sources(GameDataSource::new(strings))
}

fn set_sources(sources: GameDataSource) {
    *SOURCES.lock().unwrap() = sources;
}

pub fn get_version_name(version: GameVersion) -> String {
    let list = version_data_source();
    let first = list.into_iter().find(|v| v.value == version as i32);
    if let Some(first) = first {
        first.text
    } else {
        format!("{:?}", version)
    }
}

pub fn species_data_source() -> Vec<ComboItem<String>> {
    SOURCES.lock().unwrap().species_data_source.clone()
}

pub fn ball_data_source() -> Vec<ComboItem<String>> {
    SOURCES.lock().unwrap().ball_data_source.clone()
}

pub fn nature_data_source() -> Vec<ComboItem<String>> {
    SOURCES.lock().unwrap().nature_data_source.clone()
}

pub fn ability_data_source() -> Vec<ComboItem<String>> {
    SOURCES.lock().unwrap().ability_data_source.clone()
}

pub fn version_data_source() -> Vec<ComboItem<String>> {
    SOURCES.lock().unwrap().version_data_source.clone()
}

pub fn move_data_source() -> Vec<ComboItem<String>> {
    SOURCES.lock().unwrap().hax_move_data_source.clone()
}

pub fn ground_tile_data_source() -> Vec<ComboItem<String>> {
    SOURCES.lock().unwrap().ground_tile_data_source.clone()
}

pub fn regions() -> Vec<ComboItem<&'static str>> {
    GameDataSource::REGIONS.to_vec()
}

pub fn language_data_source(gen: u8) -> Vec<ComboItem<&'static str>> {
    GameDataSource::language_data_source(gen)
}

pub fn get_location_name(
    is_egg_location: bool,
    location: u16,
    format: u8,
    generation: u8,
    version: GameVersion,
) -> String {
    strings().get_location_name(is_egg_location, location, format, generation, version)
}

pub fn get_location_list(
    version: GameVersion,
    context: EntityContext,
    egg: Option<bool>,
) -> Vec<ComboItem<String>> {
    SOURCES
        .lock()
        .unwrap()
        .met_data_source
        .get_location_list(version, context, egg)
}

use crate::{language, LanguageID};

pub fn get_species_name_generation(species: usize, language: usize, generation: usize) -> String {
    match generation {
        _ if generation <= 4 => get_species_name_1234(species, language, generation),
        7 if language == LanguageID::ChineseS as usize => "".to_string(),
        _ => "".to_string(),
    };
    todo!()
}

fn get_species_name_1234(species: usize, language: usize, generation: usize) -> String {
    if species == 0 {
        return get_egg_name_1234(species, language, generation);
    }
    todo!()
}

fn get_egg_name_1234(species: usize, language: usize, generation: usize) -> String {
    todo!()
}

pub fn get_species_name_language(
    species: usize,
    nickname: String,
    generation: usize,
) -> Option<usize> {
    let langs = language::get_available_game_languages(generation);
    get_species_name_language_from_langs(species, nickname, generation, langs)
}

fn get_species_name_language_from_langs(
    species: usize,
    nickname: String,
    generation: usize,
    langs: &[usize],
) -> Option<usize> {
    for lang in langs.iter() {
        if get_species_name_generation(species, *lang, generation) == nickname {
            return Some(*lang);
        }
    }
    None
}

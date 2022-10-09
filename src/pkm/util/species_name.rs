use crate::{game_strings, string_converter_2_kor, string_converter_4_util, Species};
use crate::{pkx, LanguageID};
use lazy_static::lazy_static;

const LATEST_GENERATION: u8 = pkx::GENERATION;
const EGG_NAMES: [&str; 11] = [
    "タマゴ",
    "タマゴ",
    "Egg",
    "Œuf",
    "Uovo",
    "Ei",
    "Huevo",
    "Huevo",
    "알",
    "蛋",
    "蛋",
];

lazy_static! {
    pub static ref SPECIES_LANG: Vec<&'static Vec<&'static str>> = vec![
        &game_strings::SPECIES_JA,
        &game_strings::SPECIES_JA,
        &game_strings::SPECIES_EN,
        &game_strings::SPECIES_FR,
        &game_strings::SPECIES_IT,
        &game_strings::SPECIES_DE,
        &game_strings::SPECIES_ES,
        &game_strings::SPECIES_ES,
        &game_strings::SPECIES_KO,
        &game_strings::SPECIES_ZH,
        &game_strings::SPECIES_ZH2
    ];
}

pub fn get_species_name(species: u16, language: u8) -> String {
    if language as usize >= SPECIES_LANG.len() {
        String::new()
    } else if species == 0 {
        EGG_NAMES[language as usize].to_string()
    } else {
        let arr = &SPECIES_LANG[language as usize];
        if species as usize >= arr.len() {
            String::new()
        } else {
            arr[species as usize].to_string()
        }
    }
}

pub fn get_species_name_generation(species: u16, language: u8, generation: u8) -> String {
    match generation {
        i if i <= 4 => get_species_name_1234(species, language, generation),
        7 if language == LanguageID::ChineseS as u8 => get_species_name_7zh(species, language),
        _ => get_species_name(species, language),
    }
}

pub fn get_egg_name(language: u8, generation: u8) -> String {
    match generation {
        i if i <= 4 => get_egg_name_1234(0, language, generation),
        _ => {
            if language as usize >= EGG_NAMES.len() {
                String::new()
            } else {
                EGG_NAMES[language as usize].to_string()
            }
        }
    }
}

fn get_species_name_1234(species: u16, language: u8, generation: u8) -> String {
    if species == 0 {
        return get_egg_name_1234(species, language, generation);
    }

    let nick = get_species_name(species, language);
    match LanguageID::from(language) {
        LanguageID::Korean if generation == 2 => {
            return string_converter_2_kor::localize_kor_2(nick);
        }
        LanguageID::Korean | LanguageID::Japanese => {
            return nick;
        }
        _ => {}
    }

    let mut result = nick.chars().collect::<Vec<char>>();

    result
        .iter_mut()
        .for_each(|c| *c = c.to_uppercase().to_string().chars().next().unwrap());
    if language == LanguageID::French as u8 {
        string_converter_4_util::strip_diacritics_fr4(&mut result);
    }

    if generation >= 3 {
        return result.iter().collect::<String>();
    }

    if let Some((index_space, _)) = result.iter().enumerate().find(|(_, &c)| c == ' ') {
        let len = result.len();
        result = [&result[..index_space], &result[(index_space + 1)..]].concat();
        result = result.into_iter().take(len - 1).collect();
    }

    result.iter().collect::<String>()
}

fn get_egg_name_1234(species: u16, language: u8, generation: u8) -> String {
    if generation == 3 {
        return "タマゴ".to_string();
    }

    if language == LanguageID::French as u8 {
        return if generation == 2 {
            "OEUF".to_string()
        } else {
            "Oeuf".to_string()
        };
    }
    let nick = get_species_name(species, language);

    if generation == 4 {
        return nick;
    }

    nick.to_uppercase()
}

fn get_species_name_7zh(species: u16, language: u8) -> String {
    match species {
        i if i == Species::Porygon2 as u16 => "多边兽Ⅱ".to_string(),
        i if i == Species::PorygonZ as u16 => "多边兽Ｚ".to_string(),
        i if i == Species::Mimikyu as u16 => "谜拟Ｑ".to_string(),
        i if i == Species::Cofagrigus as u16 => "死神棺".to_string(),
        i if i == Species::Pangoro as u16 => "流氓熊猫".to_string(),
        _ => get_species_name(species, language),
    }
}

pub fn is_nicknamed(species: u16, nickname: &str, language: u8, generation: u8) -> bool {
    get_species_name_generation(species, language, generation).as_str() != nickname
}

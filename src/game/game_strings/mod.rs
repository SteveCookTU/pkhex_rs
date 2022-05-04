use lazy_static::lazy_static;
use crate::resource_util::load_string_list;
pub mod game_language;

const ABILITIES_EN_RAW: &str = include_str!("../../resources/text/other/en/abilities_en.txt");
const NATURES_EN_RAW: &str = include_str!("../../resources/text/other/en/natures_en.txt");
const SPECIES_EN_RAW: &str = include_str!("../../resources/text/other/en/species_en.txt");
const MOVES_EN_RAW: &str = include_str!("../../resources/text/other/en/moves_en.txt");
const ITEMS_EN_RAW: &str = include_str!("../../resources/text/items/items_en.txt");
const TYPES_EN_RAW: &str = include_str!("../../resources/text/other/en/types_en.txt");
const FORMS_EN_RAW: &str = include_str!("../../resources/text/other/en/forms_en.txt");

lazy_static! {
    pub static ref ABILITIES_EN: Vec<&'static str> = load_string_list(ABILITIES_EN_RAW);
    pub static ref NATURES_EN: Vec<&'static str> = load_string_list(NATURES_EN_RAW);
    pub static ref SPECIES_EN: Vec<&'static str> = load_string_list(SPECIES_EN_RAW);
    pub static ref MOVES_EN: Vec<&'static str> = load_string_list(MOVES_EN_RAW);
    pub static ref ITEMS_EN: Vec<&'static str> = load_string_list(ITEMS_EN_RAW);
    pub static ref FORMS_EN: Vec<&'static str> = load_string_list(FORMS_EN_RAW);
    pub static ref TYPES_EN: Vec<&'static str> = load_string_list(TYPES_EN_RAW);
}
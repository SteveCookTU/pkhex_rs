use crate::resource_util::load_string_list;
use lazy_static::lazy_static;
pub mod game_language;

const ABILITIES_EN_RAW: &str = include_str!("../../resources/text/other/en/text_Abilities_en.txt");
const NATURES_EN_RAW: &str = include_str!("../../resources/text/other/en/text_Natures_en.txt");
const SPECIES_EN_RAW: &str = include_str!("../../resources/text/other/en/text_Species_en.txt");
const MOVES_EN_RAW: &str = include_str!("../../resources/text/other/en/text_Moves_en.txt");
const ITEMS_EN_RAW: &str = include_str!("../../resources/text/items/items_en.txt");
const TYPES_EN_RAW: &str = include_str!("../../resources/text/other/en/text_Types_en.txt");
const FORMS_EN_RAW: &str = include_str!("../../resources/text/other/en/text_Forms_en.txt");

const SPECIES_JA_RAW: &str = include_str!("../../resources/text/other/ja/text_Species_ja.txt");
const SPECIES_FR_RAW: &str = include_str!("../../resources/text/other/fr/text_Species_fr.txt");
const SPECIES_IT_RAW: &str = include_str!("../../resources/text/other/it/text_Species_it.txt");
const SPECIES_DE_RAW: &str = include_str!("../../resources/text/other/de/text_Species_de.txt");
const SPECIES_ES_RAW: &str = include_str!("../../resources/text/other/es/text_Species_es.txt");
const SPECIES_KO_RAW: &str = include_str!("../../resources/text/other/ko/text_Species_ko.txt");
const SPECIES_ZH_RAW: &str = include_str!("../../resources/text/other/zh/text_Species_zh.txt");
const SPECIES_ZH2_RAW: &str = include_str!("../../resources/text/other/zh/text_Species_zh2.txt");

lazy_static! {
    pub static ref ABILITIES_EN: Vec<&'static str> = load_string_list(ABILITIES_EN_RAW);
    pub static ref NATURES_EN: Vec<&'static str> = load_string_list(NATURES_EN_RAW);
    pub static ref SPECIES_EN: Vec<&'static str> = load_string_list(SPECIES_EN_RAW);
    pub static ref MOVES_EN: Vec<&'static str> = load_string_list(MOVES_EN_RAW);
    pub static ref ITEMS_EN: Vec<&'static str> = load_string_list(ITEMS_EN_RAW);
    pub static ref FORMS_EN: Vec<&'static str> = load_string_list(FORMS_EN_RAW);
    pub static ref TYPES_EN: Vec<&'static str> = load_string_list(TYPES_EN_RAW);
    pub static ref SPECIES_JA: Vec<&'static str> = load_string_list(SPECIES_JA_RAW);
    pub static ref SPECIES_FR: Vec<&'static str> = load_string_list(SPECIES_FR_RAW);
    pub static ref SPECIES_IT: Vec<&'static str> = load_string_list(SPECIES_IT_RAW);
    pub static ref SPECIES_DE: Vec<&'static str> = load_string_list(SPECIES_DE_RAW);
    pub static ref SPECIES_ES: Vec<&'static str> = load_string_list(SPECIES_ES_RAW);
    pub static ref SPECIES_KO: Vec<&'static str> = load_string_list(SPECIES_KO_RAW);
    pub static ref SPECIES_ZH: Vec<&'static str> = load_string_list(SPECIES_ZH_RAW);
    pub static ref SPECIES_ZH2: Vec<&'static str> = load_string_list(SPECIES_ZH2_RAW);
}

use crate::combo_util;
use crate::game::enums::{GameVersion, LanguageID};
use crate::game::game_strings::{GameStrings, MetDataSource};
use crate::game::ComboItem;
use crate::moves::move_info;
use crate::pkm::enums::GroundTileType;
use crate::pkm::shared::EntityContext;
use std::sync::Arc;

pub struct GameDataSource {
    pub strings: Arc<GameStrings>,
    pub met_data_source: MetDataSource,
    pub empty: ComboItem<String>,
    pub species_data_source: Vec<ComboItem<String>>,
    pub ball_data_source: Vec<ComboItem<String>>,
    pub nature_data_source: Vec<ComboItem<String>>,
    pub ability_data_source: Vec<ComboItem<String>>,
    pub version_data_source: Vec<ComboItem<String>>,
    pub legal_move_data_source: Vec<ComboItem<String>>,
    pub hax_move_data_source: Vec<ComboItem<String>>,
    pub ground_tile_data_source: Vec<ComboItem<String>>,
}

impl GameDataSource {
    pub const REGIONS: [ComboItem<&'static str>; 6] = [
        ComboItem::new("Japan (日本)", 0),
        ComboItem::new("Americas (NA/SA)", 1),
        ComboItem::new("Europe (EU/AU)", 2),
        ComboItem::new("China (中国大陆)", 4),
        ComboItem::new("Korea (한국)", 5),
        ComboItem::new("Taiwan (香港/台灣)", 6),
    ];

    const LANGUAGE_LIST: [ComboItem<&'static str>; 9] = [
        ComboItem::new("JPN (日本語)", LanguageID::Japanese as i32),
        ComboItem::new("ENG (English)", LanguageID::English as i32),
        ComboItem::new("FRE (Français)", LanguageID::French as i32),
        ComboItem::new("ITA (Italiano)", LanguageID::Italian as i32),
        ComboItem::new("GER (Deutsch)", LanguageID::German as i32),
        ComboItem::new("ESP (Español)", LanguageID::Spanish as i32),
        ComboItem::new("KOR (한국어)", LanguageID::Korean as i32),
        ComboItem::new("CHS (简体中文)", LanguageID::ChineseS as i32),
        ComboItem::new("CHT (繁體中文)", LanguageID::ChineseT as i32),
    ];

    pub fn new(s: Arc<GameStrings>) -> GameDataSource {
        let moves =
            combo_util::get_cb_list(&s.move_list.iter().map(|s| s.as_ref()).collect::<Vec<_>>());

        let legal = moves
            .iter()
            .cloned()
            .filter(|cb| move_info::is_move_knowable(cb.value as u16))
            .collect::<Vec<_>>();

        let empty = ComboItem::new(s.item_list[0].to_string(), 0);
        let mut games = GameDataSource::get_version_list(s.clone());
        if let Some(last) = games.last_mut() {
            *last = empty.clone();
        }

        Self {
            met_data_source: MetDataSource::new(s.clone()),
            empty,
            species_data_source: combo_util::get_cb_list(
                &s.species_list
                    .iter()
                    .map(|s| s.as_ref())
                    .collect::<Vec<_>>(),
            ),
            ball_data_source: GameDataSource::get_balls(
                &s.item_list.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            ),
            nature_data_source: combo_util::get_cb_list(
                &s.natures.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            ),
            ability_data_source: combo_util::get_cb_list(
                &s.ability_list
                    .iter()
                    .map(|s| s.as_ref())
                    .collect::<Vec<_>>(),
            ),
            version_data_source: games,
            legal_move_data_source: legal,
            hax_move_data_source: moves,
            ground_tile_data_source: combo_util::get_unsorted_cb_list(
                &s.species_list
                    .iter()
                    .map(|s| s.as_ref())
                    .collect::<Vec<_>>(),
                &GroundTileType::VALID_TILE_TYPES,
            ),
            strings: s.clone(),
        }
    }

    fn get_balls(item_list: &[&str]) -> Vec<ComboItem<String>> {
        let ball_nums = [
            7, 576, 13, 492, 497, 14, 495, 493, 496, 494, 11, 498, 8, 6, 12, 15, 9, 5, 499, 10, 1,
            16, 851, 1785, 1710, 1711, 1712, 1713, 1746, 1747, 1748, 1749, 1750, 1771,
        ];
        let ball_vals = [
            7, 25, 13, 17, 22, 14, 20, 18, 21, 19, 11, 23, 8, 6, 12, 15, 9, 5, 24, 10, 1, 16, 26,
            27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37,
        ];
        combo_util::get_varied_cb_list_ball(item_list, &ball_nums, &ball_vals)
    }

    fn get_version_list(s: Arc<GameStrings>) -> Vec<ComboItem<String>> {
        let games = [
            50, 51, // 9 sv
            47, // 8 legends arceus
            48, 49, // 8 bdsp
            44, 45, // 8 swsh
            42, 43, // 7 gg
            30, 31, // 7 sm
            32, 33, // 7 usum
            24, 25, // 6 xy
            27, 26, // 6 oras
            21, 20, // 5 bw
            23, 22, // 5 b2w2
            10, 11, 12, // 4 dppt
            7, 8, // 4 hgss
            2, 1, 3, // 3 rse
            4, 5,  // 3 frlg
            15, // 3 cxd
            39, 40, 41, // 7vc2
            35, 36, 37, 38, // 7vc1
            34, // 7go
            0,
        ];

        combo_util::get_unsorted_cb_list(
            &s.game_list.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            &games,
        )
    }

    pub fn get_item_data_source(
        &self,
        game: GameVersion,
        context: EntityContext,
        allowed: &[u16],
        hax: Option<bool>,
    ) -> Vec<ComboItem<String>> {
        let items = self.strings.get_item_strings(context, Some(game));
        if hax.unwrap_or_default() {
            combo_util::get_cb_list(&items.iter().map(|s| s.as_ref()).collect::<Vec<_>>())
        } else {
            combo_util::get_cb_list_allowed(
                &items.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
                allowed,
            )
        }
    }

    pub fn language_data_source(gen: u8) -> Vec<ComboItem<&'static str>> {
        let mut languages = GameDataSource::LANGUAGE_LIST.to_vec();
        if gen == 3 {
            languages.retain(|c| c.value < LanguageID::Korean as i32);
        } else if gen < 7 {
            languages.retain(|c| c.value <= LanguageID::Korean as i32);
        }
        languages
    }
}

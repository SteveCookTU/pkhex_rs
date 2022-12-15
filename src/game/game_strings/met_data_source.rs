use crate::combo_util;
use crate::game::enums::GameVersion;
use crate::game::game_strings::GameStrings;
use crate::game::{locations, ComboItem};
use crate::pkm::shared::EntityContext;
use std::sync::Arc;

pub struct MetDataSource {
    met_gen_2: Vec<ComboItem<String>>,
    met_gen_3: Vec<ComboItem<String>>,
    met_gen_3_cxd: Vec<ComboItem<String>>,
    met_gen_4: Vec<ComboItem<String>>,
    met_gen_5: Vec<ComboItem<String>>,
    met_gen_6: Vec<ComboItem<String>>,
    met_gen_7: Vec<ComboItem<String>>,
    met_gen_7_gg: Vec<ComboItem<String>>,
    met_gen_8: Vec<ComboItem<String>>,
    met_gen_8a: Vec<ComboItem<String>>,
    met_gen_8b: Vec<ComboItem<String>>,
    met_gen_9: Vec<ComboItem<String>>,
    met_gen_4_transfer: Option<Vec<ComboItem<String>>>,
    met_gen_5_transfer: Option<Vec<ComboItem<String>>>,
}

impl MetDataSource {
    pub fn new(s: Arc<GameStrings>) -> Self {
        Self {
            met_gen_2: MetDataSource::create_gen_2(s.clone()),
            met_gen_3: MetDataSource::create_gen_3(s.clone()),
            met_gen_3_cxd: MetDataSource::create_gen_3_cxd(s.clone()),
            met_gen_4: MetDataSource::create_gen_4(s.clone()),
            met_gen_5: MetDataSource::create_gen_5(s.clone()),
            met_gen_6: MetDataSource::create_gen_6(s.clone()),
            met_gen_7: MetDataSource::create_gen_7(s.clone()),
            met_gen_7_gg: MetDataSource::create_gen_7_gg(s.clone()),
            met_gen_8: MetDataSource::create_gen_8(s.clone()),
            met_gen_8a: MetDataSource::create_gen_8a(s.clone()),
            met_gen_8b: MetDataSource::create_gen_8b(s.clone()),
            met_gen_9: MetDataSource::create_gen_9(s),
            met_gen_4_transfer: None,
            met_gen_5_transfer: None,
        }
    }

    fn create_gen_2(s: Arc<GameStrings>) -> Vec<ComboItem<String>> {
        let mut locations = combo_util::get_cb_list(
            &s.gen_2.met_0[..0x5F]
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<_>>(),
        );
        combo_util::add_cb_with_offset(
            &mut locations,
            &s.gen_2.met_0[0x7E..0x80]
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<_>>(),
            0x7E,
        );
        locations
    }

    fn create_gen_3(s: Arc<GameStrings>) -> Vec<ComboItem<String>> {
        let mut locations = combo_util::get_cb_list(
            &s.gen_3.met_0[..213]
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<_>>(),
        );
        combo_util::add_cb_with_offset(
            &mut locations,
            &s.gen_3.met_0[253..256]
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<_>>(),
            253,
        );
        locations
    }

    fn create_gen_3_cxd(s: Arc<GameStrings>) -> Vec<ComboItem<String>> {
        let mut locations =
            combo_util::get_cb_list(&s.cxd.met_0.iter().map(|s| s.as_ref()).collect::<Vec<_>>());
        locations.retain(|s| !s.text.is_empty());
        locations
    }

    fn create_gen_4(s: Arc<GameStrings>) -> Vec<ComboItem<String>> {
        let mut locations = combo_util::get_cb_list_from_offset(
            &s.gen_4.met_0.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            0,
            None,
        );
        combo_util::add_cb_with_offset_index(
            &mut locations,
            &s.gen_4.met_2.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            2000,
            locations::DAYCARE_4 as usize,
        );
        combo_util::add_cb_with_offset_index(
            &mut locations,
            &s.gen_4.met_2.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            2000,
            locations::LINK_TRADE_4 as usize,
        );
        combo_util::add_cb_with_offset_index(
            &mut locations,
            &s.gen_4.met_3.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            3000,
            locations::RANGER_4 as usize,
        );
        combo_util::add_cb_with_offset_allowed_small(
            &mut locations,
            &s.gen_4.met_0.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            0,
            &locations::locations_4::MET_0,
        );
        combo_util::add_cb_with_offset_allowed(
            &mut locations,
            &s.gen_4.met_2.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            2000,
            &locations::locations_4::MET_2,
        );
        combo_util::add_cb_with_offset_allowed(
            &mut locations,
            &s.gen_4.met_3.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            3000,
            &locations::locations_4::MET_3,
        );
        locations
    }

    fn create_gen_4_transfer(&self) -> Vec<ComboItem<String>> {
        let mut met = self.met_gen_4.clone();
        let index = met
            .iter()
            .position(|s| s.value == locations::TRANSFER_3 as i32)
            .unwrap_or_default();
        let pal = met.remove(index);
        met.insert(0, pal);
        met
    }

    fn create_gen_5(s: Arc<GameStrings>) -> Vec<ComboItem<String>> {
        let mut locations = combo_util::get_cb_list_from_offset(
            &s.gen_5.met_0.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            0,
            None,
        );
        combo_util::add_cb_with_offset_index(
            &mut locations,
            &s.gen_5.met_6.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            60000,
            locations::DAYCARE_5 as usize,
        );
        combo_util::add_cb_with_offset_index(
            &mut locations,
            &s.gen_5.met_3.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            30000,
            locations::LINK_TRADE_5 as usize,
        );
        combo_util::add_cb_with_offset_allowed_small(
            &mut locations,
            &s.gen_5.met_0.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            0,
            &locations::locations_5::MET_0,
        );
        combo_util::add_cb_with_offset_allowed(
            &mut locations,
            &s.gen_5.met_3.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            30000,
            &locations::locations_5::MET_3,
        );
        combo_util::add_cb_with_offset_allowed(
            &mut locations,
            &s.gen_5.met_4.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            40000,
            &locations::locations_5::MET_4,
        );
        combo_util::add_cb_with_offset_allowed(
            &mut locations,
            &s.gen_5.met_6.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            60000,
            &locations::locations_5::MET_6,
        );
        locations
    }

    fn create_gen_5_transfer(&self) -> Vec<ComboItem<String>> {
        let mut met = self.met_gen_5.clone();
        let index = met
            .iter()
            .position(|s| s.value == locations::TRANSFER_4 as i32)
            .unwrap_or_default();
        let xfr = met.remove(index);
        met.insert(0, xfr);
        met
    }

    fn create_gen_6(s: Arc<GameStrings>) -> Vec<ComboItem<String>> {
        let mut locations = combo_util::get_cb_list_from_offset(
            &s.gen_6.met_0.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            0,
            None,
        );
        combo_util::add_cb_with_offset_index(
            &mut locations,
            &s.gen_6.met_6.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            60000,
            locations::DAYCARE_5 as usize,
        );
        combo_util::add_cb_with_offset_index(
            &mut locations,
            &s.gen_6.met_3.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            30000,
            locations::LINK_TRADE_6 as usize,
        );
        combo_util::add_cb_with_offset_allowed(
            &mut locations,
            &s.gen_6.met_0.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            0,
            &locations::locations_6::MET_0,
        );
        combo_util::add_cb_with_offset_allowed(
            &mut locations,
            &s.gen_6.met_3.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            30000,
            &locations::locations_6::MET_3,
        );
        combo_util::add_cb_with_offset_allowed(
            &mut locations,
            &s.gen_6.met_4.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            40000,
            &locations::locations_6::MET_4,
        );
        combo_util::add_cb_with_offset_allowed(
            &mut locations,
            &s.gen_6.met_6.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            60000,
            &locations::locations_6::MET_6,
        );
        locations
    }

    fn create_gen_7(s: Arc<GameStrings>) -> Vec<ComboItem<String>> {
        let mut locations = combo_util::get_cb_list_from_offset(
            &s.gen_7.met_0.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            0,
            None,
        );
        combo_util::add_cb_with_offset_index(
            &mut locations,
            &s.gen_7.met_6.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            60000,
            locations::DAYCARE_5 as usize,
        );
        combo_util::add_cb_with_offset_index(
            &mut locations,
            &s.gen_7.met_3.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            30000,
            locations::LINK_TRADE_6 as usize,
        );
        combo_util::add_cb_with_offset_allowed_small(
            &mut locations,
            &s.gen_7.met_0.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            0,
            &locations::locations_7::MET_0,
        );
        combo_util::add_cb_with_offset_allowed(
            &mut locations,
            &s.gen_7.met_3.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            30000,
            &locations::locations_7::MET_3,
        );
        combo_util::add_cb_with_offset_allowed(
            &mut locations,
            &s.gen_7.met_4.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            40000,
            &locations::locations_7::MET_4,
        );
        combo_util::add_cb_with_offset_allowed(
            &mut locations,
            &s.gen_7.met_6.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            60000,
            &locations::locations_7::MET_6,
        );
        locations
    }

    fn create_gen_7_gg(s: Arc<GameStrings>) -> Vec<ComboItem<String>> {
        let mut locations = combo_util::get_cb_list_from_offset(
            &s.gen_7b
                .met_0
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<_>>(),
            0,
            None,
        );
        combo_util::add_cb_with_offset_index(
            &mut locations,
            &s.gen_7b
                .met_6
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<_>>(),
            60000,
            locations::DAYCARE_5 as usize,
        );
        combo_util::add_cb_with_offset_index(
            &mut locations,
            &s.gen_7b
                .met_3
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<_>>(),
            30000,
            locations::LINK_TRADE_6 as usize,
        );
        combo_util::add_cb_with_offset_allowed_small(
            &mut locations,
            &s.gen_7b
                .met_0
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<_>>(),
            0,
            &locations::locations_7b::MET_0,
        );
        combo_util::add_cb_with_offset_allowed(
            &mut locations,
            &s.gen_7b
                .met_3
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<_>>(),
            30000,
            &locations::locations_7b::MET_3,
        );
        combo_util::add_cb_with_offset_allowed(
            &mut locations,
            &s.gen_7b
                .met_4
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<_>>(),
            40000,
            &locations::locations_7b::MET_4,
        );
        combo_util::add_cb_with_offset_allowed(
            &mut locations,
            &s.gen_7b
                .met_6
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<_>>(),
            60000,
            &locations::locations_7b::MET_6,
        );
        locations
    }

    fn create_gen_8(s: Arc<GameStrings>) -> Vec<ComboItem<String>> {
        let mut locations = combo_util::get_cb_list_from_offset(
            &s.gen_8.met_0.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            0,
            None,
        );
        combo_util::add_cb_with_offset_index(
            &mut locations,
            &s.gen_8.met_6.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            60000,
            locations::DAYCARE_5 as usize,
        );
        combo_util::add_cb_with_offset_index(
            &mut locations,
            &s.gen_8.met_3.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            30000,
            locations::LINK_TRADE_6 as usize,
        );
        combo_util::add_cb_with_offset_allowed_small(
            &mut locations,
            &s.gen_8.met_0.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            0,
            &locations::locations_8::MET_0,
        );
        combo_util::add_cb_with_offset_allowed(
            &mut locations,
            &s.gen_8.met_3.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            30000,
            &locations::locations_8::MET_3,
        );
        combo_util::add_cb_with_offset_allowed(
            &mut locations,
            &s.gen_8.met_4.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            40000,
            &locations::locations_8::MET_4,
        );
        combo_util::add_cb_with_offset_allowed(
            &mut locations,
            &s.gen_8.met_6.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            60000,
            &locations::locations_8::MET_6,
        );

        locations.push(ComboItem::new(
            s.egg_name.clone(),
            locations::HOME_SWSH_BDSP_EGG as i32,
        ));
        locations.push(ComboItem::new(
            s.game_list[GameVersion::BD as usize].clone(),
            locations::HOME_SWBD as i32,
        ));
        locations.push(ComboItem::new(
            s.game_list[GameVersion::SP as usize].clone(),
            locations::HOME_SHSP as i32,
        ));
        locations.push(ComboItem::new(
            s.game_list[GameVersion::PLA as usize].clone(),
            locations::HOME_SWLA as i32,
        ));
        locations
    }

    fn create_gen_8a(s: Arc<GameStrings>) -> Vec<ComboItem<String>> {
        let mut locations = combo_util::get_cb_list_from_offset(
            &s.gen_8a
                .met_0
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<_>>(),
            0,
            None,
        );
        combo_util::add_cb_with_offset_index(
            &mut locations,
            &s.gen_8a
                .met_3
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<_>>(),
            30000,
            locations::LINK_TRADE_6 as usize,
        );
        combo_util::add_cb_with_offset_allowed_small(
            &mut locations,
            &s.gen_8a
                .met_0
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<_>>(),
            0,
            &locations::locations_8a::MET_0,
        );
        combo_util::add_cb_with_offset_allowed(
            &mut locations,
            &s.gen_8a
                .met_3
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<_>>(),
            30000,
            &locations::locations_8a::MET_3,
        );
        combo_util::add_cb_with_offset_allowed(
            &mut locations,
            &s.gen_8a
                .met_4
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<_>>(),
            40000,
            &locations::locations_8a::MET_4,
        );
        combo_util::add_cb_with_offset_allowed(
            &mut locations,
            &s.gen_8a
                .met_6
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<_>>(),
            60000,
            &locations::locations_8a::MET_6,
        );

        locations.push(ComboItem::new(
            s.egg_name.clone(),
            locations::HOME_SWSH_BDSP_EGG as i32,
        ));
        locations.push(ComboItem::new(
            s.game_list[GameVersion::BD as usize].clone(),
            locations::HOME_SWBD as i32,
        ));
        locations.push(ComboItem::new(
            s.game_list[GameVersion::SP as usize].clone(),
            locations::HOME_SHSP as i32,
        ));
        locations.push(ComboItem::new(
            s.game_list[GameVersion::PLA as usize].clone(),
            locations::HOME_SWLA as i32,
        ));
        locations
    }

    fn create_gen_8b(s: Arc<GameStrings>) -> Vec<ComboItem<String>> {
        let mut locations = vec![ComboItem::new(
            s.gen_8.met_0[0].clone(),
            locations::DEFAULT_8B_NONE as i32,
        )];
        combo_util::add_cb_with_offset_index(
            &mut locations,
            &s.gen_8b
                .met_6
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<_>>(),
            60000,
            locations::DAYCARE_5 as usize,
        );
        combo_util::add_cb_with_offset_index(
            &mut locations,
            &s.gen_8b
                .met_3
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<_>>(),
            30000,
            locations::LINK_TRADE_6 as usize,
        );
        combo_util::add_cb_with_offset_allowed(
            &mut locations,
            &s.gen_8b
                .met_0
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<_>>(),
            0,
            &locations::locations_8b::MET_0,
        );
        combo_util::add_cb_with_offset_allowed(
            &mut locations,
            &s.gen_8b
                .met_3
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<_>>(),
            30000,
            &locations::locations_8b::MET_3,
        );
        combo_util::add_cb_with_offset_allowed(
            &mut locations,
            &s.gen_8b
                .met_4
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<_>>(),
            40000,
            &locations::locations_8b::MET_4,
        );
        combo_util::add_cb_with_offset_allowed(
            &mut locations,
            &s.gen_8b
                .met_6
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<_>>(),
            60000,
            &locations::locations_8b::MET_6,
        );

        locations
    }

    fn create_gen_9(s: Arc<GameStrings>) -> Vec<ComboItem<String>> {
        let mut locations = combo_util::get_cb_list_from_offset(
            &s.gen_9.met_0.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            0,
            None,
        );
        combo_util::add_cb_with_offset_index(
            &mut locations,
            &s.gen_9.met_6.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            60000,
            locations::DAYCARE_5 as usize,
        );
        combo_util::add_cb_with_offset_index(
            &mut locations,
            &s.gen_9.met_3.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            30000,
            locations::LINK_TRADE_6 as usize,
        );
        combo_util::add_cb_with_offset_allowed_small(
            &mut locations,
            &s.gen_9.met_0.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            0,
            &locations::locations_9::MET_0,
        );
        combo_util::add_cb_with_offset_allowed(
            &mut locations,
            &s.gen_9.met_3.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            30000,
            &locations::locations_7b::MET_3,
        );
        combo_util::add_cb_with_offset_allowed(
            &mut locations,
            &s.gen_9.met_4.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            40000,
            &locations::locations_9::MET_4,
        );
        combo_util::add_cb_with_offset_allowed(
            &mut locations,
            &s.gen_9.met_6.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            60000,
            &locations::locations_9::MET_6,
        );
        locations
    }

    pub fn get_location_list(
        &mut self,
        version: GameVersion,
        context: EntityContext,
        egg: Option<bool>,
    ) -> Vec<ComboItem<String>> {
        if context == EntityContext::Gen2 {
            return self.met_gen_2.clone();
        }

        let egg = egg.unwrap_or_default();
        let mut result =
            if egg && version < GameVersion::W && context.generation().unwrap_or_default() >= 5 {
                self.met_gen_4.clone()
            } else {
                self.get_location_list_internal(version, context)
            };

        if context == EntityContext::Gen8b && !GameVersion::BDSP.contains(&version) {
            result.insert(
                1,
                ComboItem::new(result[0].text.clone(), locations::DEFAULT_8B_NONE as i32),
            );
        }

        result
    }

    fn get_location_list_internal(
        &mut self,
        version: GameVersion,
        context: EntityContext,
    ) -> Vec<ComboItem<String>> {
        match version {
            GameVersion::CXD if context == EntityContext::Gen3 => self.met_gen_3_cxd.clone(),
            GameVersion::R | GameVersion::S if context == EntityContext::Gen3 => {
                partition_1(&self.met_gen_3, locations::is_met_location_3_rs)
            }
            GameVersion::E if context == EntityContext::Gen3 => {
                partition_1(&self.met_gen_3, locations::is_met_location_3_e)
            }
            GameVersion::FR | GameVersion::LG if context == EntityContext::Gen3 => {
                partition_1(&self.met_gen_3, locations::is_met_location_3_frlg)
            }
            GameVersion::D | GameVersion::P if context == EntityContext::Gen4 => {
                partition_2(&self.met_gen_4, locations::is_met_location_4_dp, Some(4))
            }
            GameVersion::Pt if context == EntityContext::Gen4 => {
                partition_2(&self.met_gen_4, locations::is_met_location_4_pt, Some(4))
            }
            GameVersion::HG | GameVersion::SS if context == EntityContext::Gen4 => {
                partition_2(&self.met_gen_4, locations::is_met_location_4_hgss, Some(4))
            }

            GameVersion::B | GameVersion::W => {
                partition_2(&self.met_gen_5, locations::is_met_location_5_bw, None)
            }
            GameVersion::B2 | GameVersion::W2 => self.met_gen_5.clone(),
            GameVersion::X | GameVersion::Y => {
                partition_2(&self.met_gen_6, locations::is_met_location_6_xy, None)
            }
            GameVersion::AS | GameVersion::OR => {
                partition_2(&self.met_gen_6, locations::is_met_location_6_ao, None)
            }
            GameVersion::SN | GameVersion::MN => {
                partition_2(&self.met_gen_7, locations::is_met_location_7_sm, None)
            }
            GameVersion::US
            | GameVersion::UM
            | GameVersion::RD
            | GameVersion::BU
            | GameVersion::GN
            | GameVersion::YW
            | GameVersion::GD
            | GameVersion::SI
            | GameVersion::C => {
                partition_2(&self.met_gen_7, locations::is_met_location_7_usum, None)
            }
            GameVersion::GP | GameVersion::GE | GameVersion::GO => {
                partition_2(&self.met_gen_7_gg, locations::is_met_location_7_gg, None)
            }
            GameVersion::SW | GameVersion::SH => {
                partition_2(&self.met_gen_8, locations::is_met_location_8_swsh, None)
            }
            GameVersion::BD | GameVersion::SP => {
                partition_2(&self.met_gen_8b, locations::is_met_location_8_bdsp, None)
            }
            GameVersion::PLA => {
                partition_2(&self.met_gen_8a, locations::is_met_location_8_la, None)
            }
            GameVersion::SL | GameVersion::VL => {
                partition_2(&self.met_gen_9, locations::is_met_location_9_sv, None)
            }
            _ => self.get_location_list_modified(version, context),
        }
    }

    fn get_location_list_modified(
        &mut self,
        version: GameVersion,
        context: EntityContext,
    ) -> Vec<ComboItem<String>> {
        match version {
            i if i <= GameVersion::CXD && context == EntityContext::Gen4 => {
                if let Some(transfer_4) = self.met_gen_4_transfer.as_ref() {
                    transfer_4.clone()
                } else {
                    let transfer_4 = self.create_gen_4_transfer();
                    self.met_gen_4_transfer = Some(transfer_4.clone());
                    transfer_4
                }
            }
            i if i < GameVersion::X && context.generation().unwrap_or_default() >= 5 => {
                if let Some(transfer_5) = self.met_gen_5_transfer.as_ref() {
                    transfer_5.clone()
                } else {
                    let transfer_5 = self.create_gen_5_transfer();
                    self.met_gen_5_transfer = Some(transfer_5.clone());
                    transfer_5
                }
            }
            _ => vec![],
        }
    }
}

fn partition_1(
    list: &[ComboItem<String>],
    criteria: impl Fn(u16) -> bool,
) -> Vec<ComboItem<String>> {
    let mut result = vec![ComboItem::new("".to_string(), 0); list.len()];
    get_ordered_list(list, &mut result, criteria, None);
    result
}

fn get_ordered_list(
    list: &[ComboItem<String>],
    result: &mut Vec<ComboItem<String>>,
    criteria: impl Fn(u16) -> bool,
    start: Option<usize>,
) {
    let mut start = start.unwrap_or_default();
    let mut end = list.len() - 1;
    for item in list.iter().skip(start) {
        if criteria(item.value as u16) {
            result[start] = item.clone();
            start += 1;
        } else {
            result[end] = item.clone();
            end -= 1;
        }
    }
    let len = result.len();
    result[start..(len - (list.len() - start))].reverse();
}

fn partition_2(
    list: &[ComboItem<String>],
    criteria: impl Fn(u16) -> bool,
    keep_first: Option<usize>,
) -> Vec<ComboItem<String>> {
    let keep_first = keep_first.unwrap_or(3);
    let mut result = vec![ComboItem::new("".to_string(), 0); list.len()];
    for (i, cb) in list.iter().take(keep_first).enumerate() {
        result[i] = cb.clone();
    }
    get_ordered_list(list, &mut result, criteria, Some(keep_first));
    result
}

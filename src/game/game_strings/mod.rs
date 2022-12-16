use crate::game::enums::{Ability, GameVersion, Species};
use crate::game::game_strings::location_set::{
    LocationSet, LocationSet0, LocationSet4, LocationSet6,
};
use crate::game::BasicStrings;
use crate::pkm::shared::EntityContext;
use crate::{legality, resource_util};
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::Arc;

mod game_data_source;
pub mod game_info;
pub mod game_language;
pub mod geo_location;
pub mod location_set;
mod met_data_source;

pub use game_data_source::*;
pub use met_data_source::*;

#[derive(Clone)]
pub struct GameStrings {
    pub species_list: Vec<String>,
    pub move_list: Vec<String>,
    pub item_list: Vec<String>,
    pub ability_list: Vec<String>,
    pub types: Vec<String>,
    pub natures: Vec<String>,
    pub forms: Vec<String>,
    pub memories: Vec<String>,
    pub gen_loc: Vec<String>,
    pub feeling_6: Vec<String>,
    pub feeling_8: Vec<String>,
    pub intensity: Vec<String>,
    pub training_bags: Vec<String>,
    pub training_stage: Vec<String>,
    pub characteristics: Vec<String>,
    pub ground_tile_types: Vec<String>,
    pub ball_list: Vec<String>,
    pub game_list: Vec<String>,
    pub poke_blocks: Vec<String>,
    pub ribbons: Vec<String>,
    g4_items: Vec<String>,
    g3_colo_items: Vec<String>,
    g3_xd_items: Vec<String>,
    g3_items: Vec<String>,
    g2_items: Vec<String>,
    g1_items: Vec<String>,
    gen_2: LocationSet0,
    gen_3: LocationSet0,
    cxd: LocationSet0,
    gen_4: LocationSet4,
    gen_5: LocationSet6,
    gen_6: LocationSet6,
    gen_7: LocationSet6,
    gen_7b: LocationSet6,
    gen_8: LocationSet6,
    gen_8a: LocationSet6,
    gen_8b: LocationSet6,
    gen_9: LocationSet6,
    wallpaper_names: Vec<String>,
    puffs: Vec<String>,
    walker_courses: Vec<String>,
    ug_goods: Vec<String>,
    ug_spheres: Vec<String>,
    ug_traps: Vec<String>,
    ug_treasures: Vec<String>,
    lang: String,
    language_index: usize,
    pub egg_name: String,
}

impl GameStrings {
    const NPC: &'static str = "NPC";
    const EMPTY_INDEX: &'static str = "---";

    #[allow(clippy::zero_prefixed_literal)]
    const ITEMS_BALL: [u16; 38] = [
        0000, 0001, 0002, 0003, 0004, 0005, 0006, 0007, 0008, 0009, 0010, 0011, 0012, 0013, 0014,
        0015, 0016, 0492, 0493, 0494, 0495, 0496, 0497, 0498, 0499, 0576, 0851, 1785, 1710, 1711,
        1712, 1713, 1746, 1747, 1748, 1749, 1750, 1771,
    ];

    fn get(ident: impl AsRef<str>, lang: impl AsRef<str>) -> Vec<String> {
        game_language::get_strings(ident, lang, None)
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    fn get_4(ident: impl AsRef<str> + Display, lang: impl AsRef<str>) -> LocationSet4 {
        let met_0 = GameStrings::get(format!("{ident}_00000"), lang.as_ref())
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let met_2 = GameStrings::get(format!("{ident}_02000"), lang.as_ref())
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let met_3 = GameStrings::get(format!("{ident}_03000"), lang)
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        LocationSet4::new(met_0, met_2, met_3)
    }

    fn get_6(ident: impl AsRef<str> + Display, lang: impl AsRef<str>) -> LocationSet6 {
        let met_0 = GameStrings::get(format!("{ident}_00000"), lang.as_ref())
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let met_3 = GameStrings::get(format!("{ident}_30000"), lang.as_ref())
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let met_4 = GameStrings::get(format!("{ident}_40000"), lang.as_ref())
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let met_6 = GameStrings::get(format!("{ident}_60000"), lang)
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        LocationSet6::new(met_0, met_3, met_4, met_6)
    }

    fn get_g3_cxd(
        arr: &[String],
        file_name: impl AsRef<str>,
        lang: impl AsRef<str>,
    ) -> Vec<String> {
        let item_500 = GameStrings::get(file_name.as_ref(), lang);
        let mut result = vec![String::new(); 500 + item_500.len()];
        for (i, r) in result.iter_mut().enumerate().skip(arr.len()) {
            *r = format!("UNUSED {i}");
        }
        for (i, s) in arr.iter().enumerate() {
            result[i] = s.clone();
        }
        for (i, s) in item_500.into_iter().enumerate() {
            result[i + 500] = s;
        }

        result
    }

    fn get_6_from_other(
        ident: impl AsRef<str> + Display,
        lang: impl AsRef<str>,
        met_3: &[String],
        met_6: &[String],
    ) -> LocationSet6 {
        let met_0 = GameStrings::get(format!("{ident}_00000"), lang.as_ref())
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let met_4 = GameStrings::get(format!("{ident}_40000"), lang.as_ref())
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        LocationSet6::new(met_0, met_3.to_vec(), met_4, met_6.to_vec())
    }

    fn sanitize_met_strings_cxd(cxd: &mut [String]) {
        for (i, cxd) in cxd.iter_mut().enumerate().take(227) {
            if !cxd.is_empty() {
                *cxd = format!("{cxd} [{i:0>3}]");
            }
        }
    }

    fn sanitize(&mut self) {
        self.sanitize_item_names();
        self.sanitize_met_locations();

        self.ability_list[Ability::AsOneI as usize] +=
            format!(" ({})", self.species_list[Species::Glastrier as usize]).as_str();
        self.ability_list[Ability::AsOneG as usize] +=
            format!(" ({})", self.species_list[Species::Spectrier as usize]).as_str();

        self.species_list[0] = GameStrings::EMPTY_INDEX.to_string();
        let none = format!("({})", self.item_list[0]);
        self.ability_list[0] = none.clone();
        self.item_list[0] = none.clone();
        self.move_list[0] = none.clone();
        self.gen_6.met_0[0] = none.clone();
        self.gen_5.met_0[0] = none.clone();
        self.gen_4.met_0[0] = none.clone();
        self.cxd.met_0[0] = none.clone();
        self.puffs[0] = none;
    }

    fn sanitize_item_names(&mut self) {
        let hm_06 = self.item_list[425].clone();
        let hm_0 = hm_06.as_str()[..(hm_06.len() - 1)].to_string();
        self.item_list[426] = format!("{hm_0}7 (G4)");
        self.item_list[427] = format!("{hm_0}8 (G4)");
        self.item_list[456] += " (HG/SS)"; // S.S. Ticket
        self.item_list[736] += " (OR/AS)"; // S.S. Ticket
        self.item_list[463] += " (DPPt)"; // Storage Key
        self.item_list[734] += " (OR/AS)"; // Storage Key
        self.item_list[476] += " (HG/SS)"; // Basement Key
        self.item_list[723] += " (OR/AS)"; // Basement Key
        self.item_list[621] += " (M)"; // Xtransceiver
        self.item_list[626] += " (F)"; // Xtransceiver
        self.item_list[629] += " (2)"; // DNA Splicers
        self.item_list[637] += " (2)"; // Dropped Item
        self.item_list[707] += " (2)"; // Travel Trunk
        self.item_list[713] += " (2)"; // Alt Bike
        self.item_list[714] += " (2)"; // Holo Caster
        self.item_list[729] += " (1)"; // Meteorite
        self.item_list[740] += " (2)"; // Contest Costume
        self.item_list[751] += " (2)"; // Meteorite
        self.item_list[771] += " (3)"; // Meteorite
        self.item_list[772] += " (4)"; // Meteorite
        self.item_list[842] += " (SM)"; // Fishing Rod
        self.item_list[945] += " (2)"; // Used Solarizer
        self.item_list[946] += " (2)"; // Used Lunarizer

        self.item_list[873] += " (GP/GE)"; // S.S. Ticket
        self.item_list[459] += " (HG/SS)"; // Parcel
        self.item_list[467] += " (Pt)"; // Secret Key
        self.item_list[475] += " (HG/SS)"; // Card Key
        self.item_list[894] += " (GP)"; // Leaf Letter
        self.item_list[895] += " (GE)"; // Leaf Letter

        // some languages have same names for other items!
        self.item_list[878] += " (GP/GE)"; // Lift Key (Elevator Key=700)
        self.item_list[479] += " (HG/SS)"; // Lost Item (Dropped Item=636)

        for &i in &legality::tables7::POUCH_Z_CRYSTAL_USUM {
            self.item_list[i as usize] += " [Z]";
        }

        self.item_list[121] += " (1)"; // Pokémon Box Link
        self.item_list[1075] += " (2)"; // Pokémon Box Link

        self.item_list[1080] += " (SW/SH)"; // Fishing Rod

        self.item_list[1081] += " (1)"; // Rotom Bike
        self.item_list[1266] += " (2)"; // Rotom Bike
        self.item_list[1585] += " (3)"; // Rotom Bike
        self.item_list[1586] += " (4)"; // Rotom Bike

        self.item_list[1590] += " (1)"; // Reins of Unity
        self.item_list[1591] += " (2)"; // Reins of Unity
        self.item_list[1607] += " (3)"; // Reins of Unity

        for i in 12..=29 {
            self.g3_colo_items[500 + i] += format!(" ({:0>2})", i - 11).as_str();
        }

        self.g3_colo_items[500 + 10] += " (COLO)";

        GameStrings::sanitize_items_la(&mut self.item_list);
        GameStrings::sanitize_items_sv(&mut self.item_list);

        if self.lang.as_str() == "fr" {
            self.item_list[1681] += " (LA)"; // Galet Noir       dup with 617 (Dark Stone | Black Tumblestone)
            self.item_list[1262] += " (G8)"; // Nouilles         dup with 1934 (Instant Noodles | Rice)
            self.item_list[1263] += " (G8)"; // Steak Haché      dup with 1925 (Precooked Burger | Herbed Sausage)
        } else if self.lang.as_str() == "ja" {
            self.item_list[1693] += " (LA)"; // むしよけスプレー   dup with 79 (Repel)
            self.item_list[1716] += " (LA)"; // ビビリだま        dup with 847 (Adrenaline Orb | Scatter Bang)
            self.item_list[1717] += " (LA)"; // けむりだま        dup with 228 (Smoke Ball | Smoke Bomb)
        }

        self.item_list[464] += " (G4)"; // Secret Medicine
        self.item_list[1763] += " (LA)"; // Secret Medicine
    }

    fn sanitize_met_locations(&mut self) {
        GameStrings::sanitize_met_g4(&mut self.gen_4, &self.egg_name);
        GameStrings::sanitize_met_g5(
            &mut self.gen_5,
            self.language_index,
            &self.egg_name,
            &self.species_list,
        );
        GameStrings::sanitize_met_g6(&mut self.gen_6, &self.egg_name);
        GameStrings::sanitize_met_g7(&mut self.gen_7, &self.egg_name);
        GameStrings::sanitize_met_gen_7b(&mut self.gen_7b);
        GameStrings::sanitize_met_gen_8(&mut self.gen_8, &self.egg_name);
        GameStrings::sanitize_met_gen_8b(&mut self.gen_8b, &self.egg_name);
        GameStrings::sanitize_met_gen_8a(&mut self.gen_8a, &self.egg_name, &self.lang);
        GameStrings::sanitize_met_gen_9(&mut self.gen_9, &self.egg_name);

        if ["es", "it"].contains(&self.lang.as_str()) {
            // Campeonato Mundial duplicates
            for i in 28..35 {
                self.gen_6.met_4[i] += " (-)";
            }

            // Evento de Videojuegos -- first as duplicate
            self.gen_6.met_4[35] += " (-)";
            self.gen_7.met_4[38] += " (-)";
            self.gen_7b.met_4[27] += " (-)";
        }

        if self.lang.as_str() == "ko" {
            // Pokémon Ranger duplicate (should be Ranger Union)
            self.gen_5.met_4[71] += " (-)";
        }
    }

    fn sanitize_met_g4(set: &mut LocationSet4, egg_name: impl AsRef<str>) {
        set.met_0[54] += " (DP/Pt)"; // Victory Road
        set.met_0[221] += " (HG/SS)"; // Victory Road

        // German language duplicate; handle for all since it can be confused.
        set.met_0[104] += " (DP/Pt)"; // Vista Lighthouse
        set.met_0[212] += " (HG/SS)"; // Lighthouse

        set.met_0[1] += format!(" ({})", GameStrings::NPC).as_str(); // Anything from an NPC
        set.met_0[2] += format!(" ({})", egg_name.as_ref()).as_str(); // Egg From Link Trade
    }

    fn sanitize_met_g5(
        set: &mut LocationSet6,
        language_index: usize,
        egg_name: impl AsRef<str>,
        species_list: &[String],
    ) {
        set.met_0[36] = format!("{}/{}", set.met_0[84], set.met_0[36]); // Cold Storage in BW = PWT in BW2
        set.met_0[40] += " (B/W)"; // Victory Road in BW
        set.met_0[134] += " (B2/W2)"; // Victory Road in B2W2

        for i in 76..106 {
            set.met_0[i] += "●"; // BW2 Entries from 76 to 105 are for Entralink in BW
        }

        // Collision between 40002 (legal) and 00002 (illegal) "Faraway place"
        if set.met_0[2] == set.met_4[2] {
            set.met_0[2] += " (2)";
        }

        for i in 97..109 {
            set.met_4[i] += format!(" ({})", i - 97).as_str();
        }

        // Localize the Poketransfer to the language (30001)
        set.met_3[1] = game_language::get_transporter_name(language_index).to_string();
        set.met_3[2] += format!(" ({})", GameStrings::NPC).as_str(); // Anything from an NPC
        set.met_3[3] += format!(" ({})", egg_name.as_ref()).as_str(); // Link Trade (Egg)

        // Zorua/Zoroark events
        set.met_3[10] = format!("{} ({} 1)", species_list[251], species_list[570]); // Celebi's Zorua Event
        set.met_3[11] = format!("{} ({} 2)", species_list[251], species_list[570]); // Celebi's Zorua Event
        set.met_3[12] = format!("{} (1)", species_list[571]); // Zoroark
        set.met_3[13] = format!("{} (2)", species_list[571]); // Zoroark

        set.met_6[3] += format!(" ({})", egg_name.as_ref()).as_str(); // Egg Treasure Hunter/Breeder, whatever...
    }

    fn sanitize_met_g6(set: &mut LocationSet6, egg_name: impl AsRef<str>) {
        for i in (8..=136).step_by(2) {
            let next_loc = set.met_0[i + 1].clone();
            if next_loc.is_empty() {
                continue;
            }
            set.met_0[i] += format!(" ({next_loc})").as_str();
            set.met_0[i + 1] = String::new();
        }

        set.met_0[104] += " (X/Y)"; // Victory Road
        set.met_0[106] += " (X/Y)"; // Pokémon League
        set.met_0[202] += " (OR/AS)"; // Pokémon League
        set.met_0[298] += " (OR/AS)"; // Victory Road
        set.met_3[1] += format!(" ({})", GameStrings::NPC).as_str(); // Anything from an NPC
        set.met_3[2] += format!(" ({})", egg_name.as_ref()).as_str(); // Egg From Link Trade

        for i in 63..=69 {
            set.met_4[i] += format!(" ({})", i - 62).as_str()
        }
    }

    fn sanitize_met_g7(set: &mut LocationSet6, egg_name: impl AsRef<str>) {
        for i in (6..set.met_0.len()).step_by(2) {
            if (194..198).contains(&i) {
                continue;
            }
            let next_loc = set.met_0[i + 1].clone();
            if next_loc.is_empty() {
                continue;
            }
            set.met_0[i] += format!(" ({next_loc})").as_str();
            set.met_0[i + 1] = String::new();
        }

        set.met_0[32] += " (2)";
        set.met_0[102] += " (2)";
        set.met_3[1] += format!(" ({})", GameStrings::NPC).as_str(); // Anything from an NPC
        set.met_3[2] += format!(" ({})", egg_name.as_ref()).as_str(); // Egg From Link Trade

        for i in 3..=6 {
            set.met_3[i] += " (-)";
        }

        for i in 59..66 {
            set.met_4[i] += " (-)";
        }
    }

    fn sanitize_met_gen_7b(set: &mut LocationSet6) {
        for i in 48..55 {
            set.met_4[i] += " (-)";
        }
    }

    fn sanitize_met_gen_8(set: &mut LocationSet6, egg_name: impl AsRef<str>) {
        for i in (88..set.met_0.len()).step_by(2) {
            let next_loc = set.met_0[i + 1].clone();
            if next_loc.is_empty() {
                continue;
            }
            set.met_0[i] += format!(" ({next_loc})").as_str();
            set.met_0[i + 1] = String::new();
        }

        set.met_3[1] += format!(" ({})", GameStrings::NPC).as_str(); // Anything from an NPC
        set.met_3[2] += format!(" ({})", egg_name.as_ref()).as_str(); // Egg From Link Trade
        for i in 3..=6 {
            set.met_3[i] += " (-)"; // distinguish first set of regions (unused) from second (used)
        }
        set.met_3[19] += " (?)"; // Kanto for the third time

        for i in 55..61 {
            set.met_4[i] += " (-)"; // distinguish Event year duplicates
        }

        set.met_4[30] += " (-)"; // a Video game Event (in spanish etc) -- duplicate with line 39
        set.met_4[53] += " (-)"; // a Pokémon event -- duplicate with line 37

        set.met_4[81] += " (-)"; // Pokémon GO -- duplicate with 30000's entry
        set.met_4[86] += " (-)"; // Pokémon HOME -- duplicate with 30000's entry
    }

    fn sanitize_met_gen_8b(set: &mut LocationSet6, egg_name: impl AsRef<str>) {
        set.met_3[1] += format!(" ({})", GameStrings::NPC).as_str(); // Anything from an NPC
        set.met_3[2] += format!(" ({})", egg_name.as_ref()).as_str(); // Egg From Link Trade

        GameStrings::deduplicate(&mut set.met_0);
        GameStrings::deduplicate(&mut set.met_3);
        GameStrings::deduplicate(&mut set.met_4);
        GameStrings::deduplicate(&mut set.met_6);
    }

    fn sanitize_met_gen_8a(
        set: &mut LocationSet6,
        egg_name: impl AsRef<str>,
        lang: impl AsRef<str>,
    ) {
        set.met_0[31] += " (2)"; // in Floaro Gardens
        set.met_3[1] += format!(" ({})", GameStrings::NPC).as_str(); // Anything from an NPC
        set.met_3[2] += format!(" ({})", egg_name.as_ref()).as_str(); // Egg From Link Trade
        for i in 3..=6 {
            set.met_3[i] += " (-)"; // distinguish first set of regions (unused) from second (used)
        }
        set.met_3[19] += " (?)"; // Kanto for the third time

        set.met_4[30] += " (-)"; // a Video game Event (in spanish etc) -- duplicate with line 39
        set.met_4[53] += " (-)"; // a Pokémon event -- duplicate with line 37

        set.met_4[81] += " (-)"; // Pokémon GO -- duplicate with 30000's entry
        set.met_4[86] += " (-)"; // Pokémon HOME -- duplicate with 30000's entry

        for i in 55..61 {
            set.met_4[i] += " (-)"; // distinguish second set of YYYY Event from the first
        }

        if ["en", "es", "de", "it", "fr"].contains(&lang.as_ref()) {
            // Final four locations are not nouns, rather the same location reference (at the...) as prior entries.
            set.met_0[152] += " (152)"; // Galaxy Hall
            set.met_0[153] += " (153)"; // Front Gate
            set.met_0[154] += " (154)"; // Farm
            set.met_0[155] += " (155)"; // Training Grounds
        }
    }

    fn sanitize_met_gen_9(set: &mut LocationSet6, egg_name: impl AsRef<str>) {
        {
            let m = &mut set.met_0;
            m[110] += " (1)"; // Area Zero
            m[112] += " (2)"; // Area Zero
            m[114] += " (3)"; // Area Zero
            m[116] += " (4)"; // Area Zero
            m[124] += " (5)"; // Area Zero
            m[126] += " (6)"; // Area Zero
            m[128] += " (7)"; // Area Zero

            m[40] += " (1)"; // Casseroya Lake
            m[108] += " (2)"; // Casseroya Lake
            m[34] += " (1)"; // East Province (Area One)
            m[104] += " (2)"; // East Province (Area One)
            m[38] += " (1)"; // Glaseado Mountain
            m[42] += " (2)"; // Glaseado Mountain
            m[68] += " (3)"; // Glaseado Mountain
            m[8] += " (1)"; // Mesagoza
            m[72] += " (2)"; // Mesagoza
            m[74] += " (3)"; // Mesagoza
            m[44] += " (1)"; // North Province (Area Three)
            m[102] += " (2)"; // North Province (Area Three)
            m[47] += " (1)"; // North Province (Area Two)
            m[98] += " (2)"; // North Province (Area Two)
            m[16] += " (1)"; // South Province (Area Six)
            m[66] += " (2)"; // South Province (Area Six)
            m[30] += " (1)"; // Tagtree Thicket
            m[106] += " (2)"; // Tagtree Thicket
            m[22] += " (1)"; // West Province (Area One)
            m[100] += " (2)"; // West Province (Area One)
            m[52] += " (1)"; // Zero Gate
            m[54] += " (2)"; // Zero Gate
            m[118] += " (1)"; // Zero Lab
            m[120] += " (2)"; // Zero Lab
            m[122] += " (3)"; // Zero Lab
        }
        set.met_3[1] += format!(" ({})", GameStrings::NPC).as_str(); // Anything from an NPC
        set.met_3[2] += format!(" ({})", egg_name.as_ref()).as_str(); // Egg From Link Trade

        for i in 3..=6 {
            set.met_3[i] += " (-)"; // distinguish first set of regions (unused) from second (used)
        }
        set.met_3[19] += " (?)"; // Kanto for the third time

        for i in 49..=54 {
            set.met_4[i] += " (-)"; // distinguish second set of YYYY Event from the first
        }

        set.met_4[27] += " (-)"; // a Video game Event (in spanish etc) -- duplicate with line 36
        set.met_4[48] += " (-)"; // a Pokémon event -- duplicate with line 34

        set.met_4[73] += " (-)"; // Pokémon GO -- duplicate with 30000's entry
        set.met_4[78] += " (-)"; // Pokémon HOME -- duplicate with 30000's entry
    }

    fn sanitize_items_sv(items: &mut [String]) {
        items[2313] += " (1)"; // Academy Bottle
        items[2314] += " (2)"; // Academy Bottle
        items[2318] += " (1)"; // Academy Cup
        items[2319] += " (2)"; // Academy Cup
        items[2323] += " (1)"; // Academy Tablecloth
        items[2324] += " (2)"; // Academy Tablecloth
        items[2329] += " (1)"; // Academy Ball
        items[2330] += " (2)"; // Academy Ball
        items[694] += " (G6-8)"; // TM100, not held.
    }

    fn sanitize_items_la(items: &mut [String]) {
        // Recipes
        items[1784] += " (~)"; // Gigaton Ball
        items[1783] += " (~)"; // Leaden Ball
        items[1753] += " (~)"; // Heavy Ball
        items[1752] += " (~)"; // Jet Ball
        items[1751] += " (~)"; // Wing Ball
        items[1731] += " (~)"; // Twice-Spiced Radish
        items[1730] += " (~)"; // Choice Dumpling
        items[1729] += " (~)"; // Swap Snack
        items[1677] += " (~)"; // Aux Powerguard
        items[1676] += " (~)"; // Aux Evasion
        items[1675] += " (~)"; // Dire Hit
        items[1674] += " (~)"; // Aux Guard
        items[1673] += " (~)"; // Aux Power
        items[1671] += " (~)"; // Stealth Spray
        items[1670] += " (~)"; // Max Elixir
        items[1669] += " (~)"; // Max Ether
        items[1668] += " (~)"; // Max Revive
        items[1667] += " (~)"; // Revive
        items[1666] += " (~)"; // Full Heal
        items[1665] += " (~)"; // Jubilife Muffin
        items[1664] += " (~)"; // Old Gateau
        items[1663] += " (~)"; // Superb Remedy
        items[1662] += " (~)"; // Fine Remedy
        items[1661] += " (~)"; // Remedy
        items[1660] += " (~)"; // Full Restore
        items[1659] += " (~)"; // Max Potion
        items[1658] += " (~)"; // Hyper Potion
        items[1657] += " (~)"; // Super Potion
        items[1656] += " (~)"; // Potion
        items[1655] += " (~)"; // Salt Cake
        items[1654] += " (~)"; // Bean Cake
        items[1653] += " (~)"; // Grain Cake
        items[1652] += " (~)"; // Honey Cake
        items[1650] += " (~)"; // Mushroom Cake
        items[1649] += " (~)"; // Star Piece
        items[1648] += " (~)"; // Sticky Glob
        items[1647] += " (~)"; // Scatter Bang
        items[1646] += " (~)"; // Smoke Bomb
        items[1644] += " (~)"; // Pokéshi Doll
        items[1643] += " (~)"; // Feather Ball
        items[1642] += " (~)"; // Ultra Ball
        items[1641] += " (~)"; // Great Ball
        items[1640] += " (~)"; // Poké Ball

        // Items
        items[1616] += " (LA)"; // Dire Hit
        items[1689] += " (LA)"; // Snowball
        items[1710] += " (LA)"; // Poké Ball
        items[1711] += " (LA)"; // Great Ball
        items[1712] += " (LA)"; // Ultra Ball
        items[1748] += " (LA)"; // Heavy Ball

        // Key Items
        items[1622] += " (-)"; // Poké Ball
        items[1765] += " (1)"; // Lost Satchel
        items[1766] += " (2)"; // Lost Satchel
        items[1767] += " (3)"; // Lost Satchel
        items[1768] += " (4)"; // Lost Satchel
        items[1769] += " (5)"; // Lost Satchel
    }

    pub fn new(lang: impl AsRef<str>) -> Arc<Self> {
        let gen7 = GameStrings::get_6("sm", lang.as_ref());

        let mut result = Self {
            species_list: vec![],
            move_list: vec![],
            item_list: vec![],
            ability_list: vec![],
            types: vec![],
            natures: vec![],
            forms: vec![],
            memories: vec![],
            gen_loc: vec![],
            feeling_6: vec![],
            feeling_8: vec![],
            intensity: vec![],
            training_bags: vec![],
            training_stage: vec![],
            characteristics: vec![],
            ground_tile_types: vec![],
            ball_list: vec![],
            game_list: vec![],
            poke_blocks: vec![],
            ribbons: vec![],
            g4_items: vec![],
            g3_colo_items: vec![],
            g3_xd_items: vec![],
            g3_items: vec![],
            g2_items: vec![],
            g1_items: vec![],
            gen_2: LocationSet0::new(GameStrings::get("gsc_00000", lang.as_ref())),
            gen_3: LocationSet0::new(GameStrings::get("rsefrlg_00000", lang.as_ref())),
            cxd: LocationSet0::new(GameStrings::get("cxd_00000", lang.as_ref())),
            gen_4: GameStrings::get_4("hgss", lang.as_ref()),
            gen_5: GameStrings::get_6("bw2", lang.as_ref()),
            gen_6: GameStrings::get_6("xy", lang.as_ref()),
            gen_7b: GameStrings::get_6_from_other("gg", lang.as_ref(), &gen7.met_3, &gen7.met_6),
            gen_7: gen7,
            gen_8: GameStrings::get_6("swsh", lang.as_ref()),
            gen_8a: GameStrings::get_6("la", lang.as_ref()),
            gen_8b: GameStrings::get_6("bdsp", lang.as_ref()),
            gen_9: GameStrings::get_6("sv", lang.as_ref()),
            wallpaper_names: vec![],
            puffs: vec![],
            walker_courses: vec![],
            ug_goods: vec![],
            ug_spheres: vec![],
            ug_traps: vec![],
            ug_treasures: vec![],
            language_index: game_language::get_language_index(lang.as_ref()),
            lang: lang.as_ref().to_string(),
            egg_name: "".to_string(),
        };

        result.ribbons = GameStrings::get("ribbons", lang.as_ref());

        result.g3_items = GameStrings::get("ItemsG3", lang.as_ref());
        result.g3_colo_items =
            GameStrings::get_g3_cxd(&result.g3_items, "ItemsG3Colosseum", lang.as_ref());
        result.g3_xd_items = GameStrings::get_g3_cxd(&result.g3_items, "ItemsG3XD", lang.as_ref());

        result.g2_items = GameStrings::get("ItemsG2", lang.as_ref());
        result.g1_items = GameStrings::get("ItemsG1", lang.as_ref());

        GameStrings::sanitize_met_strings_cxd(&mut result.cxd.met_0);

        result.natures = resource_util::get_natures_list(lang.as_ref())
            .iter()
            .map(|s| s.to_string())
            .collect();
        result.types = GameStrings::get("types", lang.as_ref());
        result.ability_list = GameStrings::get("abilities", lang.as_ref());

        result.move_list = GameStrings::get("moves", lang.as_ref());

        for i in 622..658 {
            const P: &str = " (P)";
            const S: &str = " (P)";
            let is_physical_z_move = (i & 1) == 0;
            result.move_list[i] += if is_physical_z_move { P } else { S };
        }

        result.characteristics = GameStrings::get("character", lang.as_ref());
        result.species_list = GameStrings::get("species", lang.as_ref());
        result.wallpaper_names = GameStrings::get("wallpaper", lang.as_ref());
        result.ground_tile_types = GameStrings::get("groundtile", lang.as_ref());
        result.game_list = GameStrings::get("games", lang.as_ref());

        result.item_list = GameStrings::get("items", lang.as_ref());
        let mut ball_list = vec![String::new(); GameStrings::ITEMS_BALL.len()];
        for (i, ball) in ball_list.iter_mut().enumerate() {
            *ball = result.item_list[GameStrings::ITEMS_BALL[i] as usize].clone();
        }
        result.ball_list = ball_list;

        result.poke_blocks = GameStrings::get("pokeblock", lang.as_ref());
        result.forms = GameStrings::get("forms", lang.as_ref());
        result.memories = GameStrings::get("memories", lang.as_ref());
        result.feeling_6 = GameStrings::get("feeling6", lang.as_ref());
        result.feeling_8 = GameStrings::get("feeling", lang.as_ref());
        result.intensity = GameStrings::get("intensity", lang.as_ref());
        result.gen_loc = GameStrings::get("genloc", lang.as_ref());
        result.training_bags = GameStrings::get("TrainingBag", lang.as_ref());
        result.training_stage = GameStrings::get("supertraining", lang.as_ref());
        result.puffs = GameStrings::get("puff", lang.as_ref());

        result.walker_courses = GameStrings::get("hgss_walkercourses", lang.as_ref());

        result.ug_goods = GameStrings::get("dppt_uggoods", lang.as_ref());
        result.ug_spheres = GameStrings::get("dppt_ugspheres", lang.as_ref());
        result.ug_traps = GameStrings::get("dppt_ugtraps", lang.as_ref());
        result.ug_treasures = GameStrings::get("dppt_ugtreasures", lang.as_ref());

        result.egg_name = result.species_list[0].clone();

        result.sanitize();

        result.g4_items = result.item_list.clone();
        for (i, s) in GameStrings::get("mail4", lang.as_ref())
            .into_iter()
            .enumerate()
        {
            result.g4_items[i + 137] = s;
        }

        Arc::new(result)
    }

    fn deduplicate(arr: &mut [String]) {
        let mut counts = HashMap::new();

        for s in arr.iter() {
            *counts.entry(s.clone()).or_insert(0) += 1;
        }

        let mut max_counts = counts.clone();

        for (i, s) in arr.iter_mut().enumerate().rev() {
            let count = counts.entry(s.clone()).or_insert(0);
            *count -= 1;
            if *count == 1 {
                continue;
            }
            let suffix = match *max_counts.entry(s.clone()).or_insert(0) {
                i if i >= 100 => format!(" ({:0>3})", i),
                i if i >= 10 => format!(" ({:0>2})", i),
                _ => format!(" ({})", i),
            };

            *s += suffix.as_str();
        }
    }

    pub fn get_item_strings(
        &self,
        context: EntityContext,
        game: Option<GameVersion>,
    ) -> Vec<String> {
        let game = game.unwrap_or(GameVersion::Any);
        match context {
            EntityContext::Gen1 => self.g1_items.clone(),
            EntityContext::Gen2 => self.g2_items.clone(),
            EntityContext::Gen3 => self.get_item_strings_3(game),
            EntityContext::Gen4 => self.g4_items.clone(),
            EntityContext::Gen8b => self.get_item_strings_8b(),
            EntityContext::Gen9 => self.get_item_strings_9(),
            _ => self.item_list.clone(),
        }
    }

    fn get_item_strings_8b(&self) -> Vec<String> {
        let mut clone = self.item_list.clone();
        let tm = clone[419][..2].to_string();
        for (i, s) in clone.iter_mut().enumerate().take(427 + 1).skip(420) {
            *s = format!("{tm}{}", i - 420 + 93);
        }

        clone[618] += "(-)"; // TM93
        clone[619] += "(-)"; // TM94
        clone[620] += "(-)"; // TM95
        clone[690] += "(-)"; // TM96
        clone[691] += "(-)"; // TM97
        clone[692] += "(-)"; // TM98
        clone[693] += "(-)"; // TM99
        clone[694] += "(-)"; // TM100

        clone
    }

    fn get_item_strings_9(&self) -> Vec<String> {
        let mut clone = self.item_list.clone();
        let zero = if ["ja", "zh", "zh2"].contains(&self.lang.as_str()) {
            "０"
        } else {
            "0"
        };

        for s in clone.iter_mut().take(419 + 1).skip(328) {
            let len = s.len();
            s.insert_str(len - 2, zero);
        }

        for s in clone.iter_mut().take(620 + 1).skip(618) {
            let len = s.len();
            s.insert_str(len - 2, zero);
        }

        for s in clone.iter_mut().take(693 + 1).skip(690) {
            let len = s.len();
            s.insert_str(len - 2, zero);
        }

        clone
    }

    fn get_item_strings_3(&self, game: GameVersion) -> Vec<String> {
        match game {
            GameVersion::COLO => self.g3_colo_items.clone(),
            GameVersion::XD => self.g3_xd_items.clone(),
            _ => self.g3_items.clone(), // TODO: E-Reader Berry
        }
    }

    pub fn get_location_name(
        &self,
        is_egg_location: bool,
        location: u16,
        format: u8,
        generation: u8,
        version: GameVersion,
    ) -> String {
        if format == 1 {
            if location == 0 {
                return String::new();
            }
            return self.gen_3.get_location_name(location).to_string();
        }

        let generation = GameStrings::get_generation(generation, is_egg_location, format);
        let set = self.get_locations(generation.unwrap_or(u8::MAX), version);
        if let Some(set) = set {
            set.get_location_name(location).to_string()
        } else {
            String::new()
        }
    }

    fn get_generation(generation: u8, is_egg_location: bool, format: u8) -> Option<u8> {
        if format == 2 {
            Some(2)
        } else if format == 3 {
            Some(3)
        } else if generation == 4 && (is_egg_location || format == 4) {
            Some(4)
        } else if generation >= 5 {
            Some(generation)
        } else if format >= 5 {
            Some(format)
        } else {
            None
        }
    }

    fn get_locations(&self, gen: u8, version: GameVersion) -> Option<&dyn LocationSet> {
        match gen {
            2 => Some(&self.gen_2),
            3 => {
                if GameVersion::CXD.contains(&version) {
                    Some(&self.cxd)
                } else {
                    Some(&self.gen_3)
                }
            }
            4 => Some(&self.gen_4),
            5 => Some(&self.gen_5),
            6 => Some(&self.gen_6),
            7 => {
                if GameVersion::Gen7b.contains(&version) {
                    Some(&self.gen_7b)
                } else {
                    Some(&self.gen_7)
                }
            }
            8 if version == GameVersion::PLA => Some(&self.gen_8a),
            8 if GameVersion::BDSP.contains(&version) => Some(&self.gen_8b),
            8 => Some(&self.gen_8),
            9 => Some(&self.gen_9),
            _ => None,
        }
    }

    pub fn get_location_names(
        &self,
        gen: u8,
        version: GameVersion,
        bank_id: Option<u16>,
    ) -> &[String] {
        let bank_id = bank_id.unwrap_or_default();
        let set = self.get_locations(gen, version);
        if let Some(set) = set {
            set.get_location_names(bank_id)
        } else {
            &[]
        }
    }
}

impl BasicStrings for GameStrings {
    fn species(&self) -> &[String] {
        &self.species_list
    }

    fn items(&self) -> &[String] {
        &self.item_list
    }

    fn moves(&self) -> &[String] {
        &self.move_list
    }

    fn abilities(&self) -> &[String] {
        &self.ability_list
    }

    fn types(&self) -> &[String] {
        &self.types
    }

    fn natures(&self) -> &[String] {
        &self.natures
    }

    fn egg_name(&self) -> &str {
        &self.egg_name
    }
}

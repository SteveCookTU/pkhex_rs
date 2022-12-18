use crate::game::enums::Species;

pub mod tables1;
pub mod tables2;
pub mod tables3;
pub mod tables4;
pub mod tables5;
pub mod tables6;
pub mod tables7;
pub mod tables7b;
pub mod tables8;
pub mod tables8a;
pub mod tables8bs;
pub mod tables9;

const fn get_permit_list<const SIZE: usize>(allowed: &[u16]) -> [bool; SIZE] {
    let mut result = [false; SIZE];
    let mut index = 0;
    while index < allowed.len() {
        result[allowed[index] as usize] = true;
        index += 1;
    }
    result
}

const fn get_permit_list_disallowed<const SIZE: usize>(
    allowed: &[u16],
    disallow: &[u16],
) -> [bool; SIZE] {
    let mut result = get_permit_list(allowed);
    let mut index = 0;
    while index < disallow.len() {
        result[disallow[index] as usize] = false;
        index += 1;
    }
    result
}

pub const BATTLE_FRONTIER_BAN_LIST: [u16; 43] = [
    Species::Mewtwo as u16,
    Species::Mew as u16,
    Species::Lugia as u16,
    Species::HoOh as u16,
    Species::Celebi as u16,
    Species::Kyogre as u16,
    Species::Groudon as u16,
    Species::Rayquaza as u16,
    Species::Jirachi as u16,
    Species::Deoxys as u16,
    Species::Dialga as u16,
    Species::Palkia as u16,
    Species::Giratina as u16,
    Species::Phione as u16,
    Species::Manaphy as u16,
    Species::Darkrai as u16,
    Species::Shaymin as u16,
    Species::Arceus as u16,
    Species::Victini as u16,
    Species::Reshiram as u16,
    Species::Zekrom as u16,
    Species::Kyurem as u16,
    Species::Keldeo as u16,
    Species::Meloetta as u16,
    Species::Genesect as u16,
    Species::Xerneas as u16,
    Species::Yveltal as u16,
    Species::Zygarde as u16,
    Species::Diancie as u16,
    Species::Hoopa as u16,
    Species::Volcanion as u16,
    Species::Cosmog as u16,
    Species::Cosmoem as u16,
    Species::Solgaleo as u16,
    Species::Lunala as u16,
    Species::Necrozma as u16,
    Species::Magearna as u16,
    Species::Marshadow as u16,
    Species::Zeraora as u16,
    Species::Meltan as u16,
    Species::Melmetal as u16,
    Species::Koraidon as u16,
    Species::Miraidon as u16,
];

pub const MYTHICALS: [u16; 22] = [
    Species::Mew as u16,
    Species::Celebi as u16,
    Species::Jirachi as u16,
    Species::Deoxys as u16,
    Species::Phione as u16,
    Species::Manaphy as u16,
    Species::Darkrai as u16,
    Species::Shaymin as u16,
    Species::Arceus as u16,
    Species::Victini as u16,
    Species::Keldeo as u16,
    Species::Meloetta as u16,
    Species::Genesect as u16,
    Species::Diancie as u16,
    Species::Hoopa as u16,
    Species::Volcanion as u16,
    Species::Magearna as u16,
    Species::Marshadow as u16,
    Species::Zeraora as u16,
    Species::Meltan as u16,
    Species::Melmetal as u16,
    Species::Zarude as u16,
];

pub const LEGENDS: [u16; 48] = [
    Species::Mewtwo as u16,
    Species::Mew as u16,
    Species::Lugia as u16,
    Species::HoOh as u16,
    Species::Celebi as u16,
    Species::Kyogre as u16,
    Species::Groudon as u16,
    Species::Rayquaza as u16,
    Species::Jirachi as u16,
    Species::Deoxys as u16,
    Species::Dialga as u16,
    Species::Palkia as u16,
    Species::Giratina as u16,
    Species::Phione as u16,
    Species::Manaphy as u16,
    Species::Darkrai as u16,
    Species::Shaymin as u16,
    Species::Arceus as u16,
    Species::Victini as u16,
    Species::Reshiram as u16,
    Species::Zekrom as u16,
    Species::Kyurem as u16,
    Species::Keldeo as u16,
    Species::Meloetta as u16,
    Species::Genesect as u16,
    Species::Xerneas as u16,
    Species::Yveltal as u16,
    Species::Zygarde as u16,
    Species::Diancie as u16,
    Species::Hoopa as u16,
    Species::Volcanion as u16,
    Species::Cosmog as u16,
    Species::Cosmoem as u16,
    Species::Solgaleo as u16,
    Species::Lunala as u16,
    Species::Necrozma as u16,
    Species::Magearna as u16,
    Species::Marshadow as u16,
    Species::Zeraora as u16,
    Species::Meltan as u16,
    Species::Melmetal as u16,
    Species::Zacian as u16,
    Species::Zamazenta as u16,
    Species::Eternatus as u16,
    Species::Zarude as u16,
    Species::Calyrex as u16,
    Species::Koraidon as u16,
    Species::Miraidon as u16,
];

pub const SUB_LEGENDS: [u16; 51] = [
    Species::Articuno as u16,
    Species::Zapdos as u16,
    Species::Moltres as u16,
    Species::Raikou as u16,
    Species::Entei as u16,
    Species::Suicune as u16,
    Species::Regirock as u16,
    Species::Regice as u16,
    Species::Registeel as u16,
    Species::Latias as u16,
    Species::Latios as u16,
    Species::Uxie as u16,
    Species::Mesprit as u16,
    Species::Azelf as u16,
    Species::Heatran as u16,
    Species::Regigigas as u16,
    Species::Cresselia as u16,
    Species::Cobalion as u16,
    Species::Terrakion as u16,
    Species::Virizion as u16,
    Species::Tornadus as u16,
    Species::Thundurus as u16,
    Species::Landorus as u16,
    Species::TypeNull as u16,
    Species::Silvally as u16,
    Species::TapuKoko as u16,
    Species::TapuLele as u16,
    Species::TapuBulu as u16,
    Species::TapuFini as u16,
    Species::Nihilego as u16,
    Species::Buzzwole as u16,
    Species::Pheromosa as u16,
    Species::Xurkitree as u16,
    Species::Celesteela as u16,
    Species::Kartana as u16,
    Species::Guzzlord as u16,
    Species::Poipole as u16,
    Species::Naganadel as u16,
    Species::Stakataka as u16,
    Species::Blacephalon as u16,
    Species::Kubfu as u16,
    Species::Urshifu as u16,
    Species::Regieleki as u16,
    Species::Regidrago as u16,
    Species::Glastrier as u16,
    Species::Spectrier as u16,
    Species::Enamorus as u16,
    Species::TingLu as u16,
    Species::ChienPao as u16,
    Species::WoChien as u16,
    Species::ChiYu as u16,
];

pub const FIXED_GENDER_FROM_BI_GENDER: [u16; 13] = [
    Species::Nincada as u16,
    Species::Shedinja as u16, // (G)
    Species::Burmy as u16,
    Species::Wormadam as u16, //(F)
    Species::Mothim as u16,   // (M)
    Species::Ralts as u16,
    Species::Gallade as u16, // (M)
    Species::Snorunt as u16,
    Species::Froslass as u16, // (F)
    Species::Espurr as u16,
    Species::Meowstic as u16, // (M/F) form specific
    Species::Lechonk as u16,
    Species::Oinkologne as u16, // (M/F) form specific
];

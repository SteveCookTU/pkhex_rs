use crate::Species;

pub const FUTURE_EVOLUTIONS_GEN1: [usize; 21] = [
    Species::Crobat as usize,
    Species::Bellossom as usize,
    Species::Politoed as usize,
    Species::Espeon as usize,
    Species::Umbreon as usize,
    Species::Slowking as usize,
    Species::Steelix as usize,
    Species::Scizor as usize,
    Species::Kingdra as usize,
    Species::Porygon2 as usize,
    Species::Blissey as usize,
    Species::Magnezone as usize,
    Species::Lickilicky as usize,
    Species::Rhyperior as usize,
    Species::Tangrowth as usize,
    Species::Electivire as usize,
    Species::Magmortar as usize,
    Species::Leafeon as usize,
    Species::Glaceon as usize,
    Species::PorygonZ as usize,
    Species::Sylveon as usize,
];

pub const FUTURE_EVOLUTIONS_GEN2: [usize; 18] = [
    Species::Ambipom as usize,
    Species::Mismagius as usize,
    Species::Honchkrow as usize,
    Species::Weavile as usize,
    Species::Magnezone as usize,
    Species::Lickilicky as usize,
    Species::Rhyperior as usize,
    Species::Tangrowth as usize,
    Species::Electivire as usize,
    Species::Magmortar as usize,
    Species::Togekiss as usize,
    Species::Yanmega as usize,
    Species::Leafeon as usize,
    Species::Glaceon as usize,
    Species::Gliscor as usize,
    Species::Mamoswine as usize,
    Species::PorygonZ as usize,
    Species::Sylveon as usize,
];

pub const FUTURE_EVOLUTIONS_GEN3: [usize; 23] = [
    Species::Roserade as usize,
    Species::Ambipom as usize,
    Species::Mismagius as usize,
    Species::Honchkrow as usize,
    Species::Weavile as usize,
    Species::Magnezone as usize,
    Species::Lickilicky as usize,
    Species::Rhyperior as usize,
    Species::Tangrowth as usize,
    Species::Electivire as usize,
    Species::Magmortar as usize,
    Species::Togekiss as usize,
    Species::Yanmega as usize,
    Species::Leafeon as usize,
    Species::Glaceon as usize,
    Species::Gliscor as usize,
    Species::Mamoswine as usize,
    Species::PorygonZ as usize,
    Species::Gallade as usize,
    Species::Probopass as usize,
    Species::Dusknoir as usize,
    Species::Froslass as usize,
    Species::Sylveon as usize,
];

pub const FUTURE_EVOLUTIONS_GEN4: [usize; 1] = [Species::Sylveon as usize];

pub const FUTURE_EVOLUTIONS_GEN5: [usize; 1] = FUTURE_EVOLUTIONS_GEN4;

pub fn get_future_gen_evolutions(generation: usize) -> &'static [usize] {
    match generation {
        1 => &FUTURE_EVOLUTIONS_GEN1,
        2 => &FUTURE_EVOLUTIONS_GEN2,
        3 => &FUTURE_EVOLUTIONS_GEN3,
        4 => &FUTURE_EVOLUTIONS_GEN4,
        5 => &FUTURE_EVOLUTIONS_GEN5,
        _ => &[],
    }
}

use crate::game::enums::{GameVersion, Species};
use crate::pkm::pkm_trait::Pkm;
use crate::pkm::traits::form_argument::FormArgument;
use crate::pkm::traits::metadata::SpeciesForm;
use crate::pkm::PKM;

pub trait ObedienceLevelReadOnly {
    fn obedience_level(&self) -> u8;
}

pub trait ObedienceLevel: ObedienceLevelReadOnly {
    fn set_obedience_level(&mut self, level: u8);

    fn get_suggested_obedience_level(&self, entity: &PKM, original_met: u8) -> u8 {
        let pkm = entity.as_pkm();
        if [Species::Koraidon as u16, Species::Miraidon as u16].contains(&pkm.species()) {
            if let Some(pk9) = entity.as_pk9() {
                if pk9.form_argument() != 0 {
                    return 0;
                }
            }
        }
        if ![GameVersion::SL as u8, GameVersion::VL as u8].contains(&pkm.version()) {
            return pkm.current_level();
        }
        original_met
    }
}

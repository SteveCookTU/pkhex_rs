pub mod enums;
mod pk8;
mod pk9;
mod pkm_trait;
pub mod shared;
pub mod strings;
pub mod traits;
pub mod util;

use crate::legality::evolutions::EvolutionHistory;
use crate::pkm::traits::form_argument::{
    get_form_argument_max, get_form_argument_min_evolution, is_form_argument_type_date_pair,
    FormArgument,
};
use crate::pkm::traits::metadata::SpeciesForm;
use crate::pkm::traits::templates::ContestStatsReadOnly;
use crate::pkm::traits::HyperTrain;
pub use pk8::*;
pub use pk9::*;
pub use pkm_trait::*;

use crate::pkm::util::entity_file_extension;

pub fn extensions() -> Vec<String> {
    entity_file_extension::get_extensions(None)
}

pub enum PKM {
    PK9(PK9),
}

impl PKM {
    pub fn as_pkm(&self) -> &(impl Pkm + Sized) {
        match self {
            PKM::PK9(pk9) => pk9,
        }
    }

    pub fn as_pk9(&self) -> Option<&PK9> {
        match self {
            PKM::PK9(pk9) => Some(pk9),
        }
    }

    pub fn as_contest_stats_read_only(&self) -> Option<&(impl ContestStatsReadOnly + Pkm + Sized)> {
        match self {
            PKM::PK9(pk9) => Some(pk9),
        }
    }

    pub fn as_form_argument_mut(&mut self) -> Option<&mut (impl FormArgument + Pkm + Sized)> {
        match self {
            PKM::PK9(pk9) => Some(pk9),
        }
    }

    pub fn as_hyper_train_mut(&mut self) -> Option<&mut (impl HyperTrain + Pkm + Sized)> {
        match self {
            PKM::PK9(pk9) => Some(pk9),
        }
    }

    pub fn set_suggested_form_argument(&mut self, original_species: Option<u16>) {
        let original_species = original_species.unwrap_or_default();
        if let Some(pk) = self.as_form_argument_mut() {
            let value = if is_form_argument_type_date_pair(pk.species(), pk.form()) {
                get_form_argument_max(pk.species(), pk.form(), pk.format().unwrap_or_default())
            } else {
                get_form_argument_min_evolution(pk.species(), original_species)
            };
            pk.change_form_argument(
                pk.species(),
                pk.form(),
                pk.format().unwrap_or_default(),
                value,
            );
        }
    }

    pub fn change_form_argument(&mut self, value: u32) {
        if let Some(pk) = self.as_form_argument_mut() {
            pk.change_form_argument(
                pk.species(),
                pk.form(),
                pk.format().unwrap_or_default(),
                value,
            );
        }
    }

    pub fn is_hyper_training_available(&self, _h: &EvolutionHistory) -> bool {
        //TODO: PA8
        true
    }

    pub fn set_suggested_hyper_training_data(&mut self, h: &EvolutionHistory, ivs: &[u8]) {
        if self.as_hyper_train_mut().is_none() {
            return;
        }
        if self.is_hyper_training_available(h) {
            self.as_hyper_train_mut().unwrap().set_hyper_train_flags(0);
            return;
        }
        {
            let pk = self.as_hyper_train_mut().unwrap();
            pk.set_ht_hp(ivs[0] != 31);
            pk.set_ht_atk(ivs[1] != 31 && ivs[1] > 2);
            pk.set_ht_def(ivs[2] != 31);
            pk.set_ht_spa(ivs[4] != 31);
            pk.set_ht_spd(ivs[5] != 31);

            pk.set_ht_spe(
                ivs[3] != 31
                    && ivs[3] > 2
                    && (ivs[3] > 17
                        || pk.ht_hp()
                        || pk.ht_atk()
                        || pk.ht_def()
                        || pk.ht_spa()
                        || pk.ht_spd()),
            );
        }

        //TODO: CombatPower
    }
}

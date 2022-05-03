use crate::form_argument_util::{get_form_argument_max, is_form_argument_type_date_pair};
use crate::Species;

pub trait FormArgument {
    fn get_form_argument(&self) -> usize;
    fn set_form_argument(&mut self, arg: usize);
    fn get_form_argument_remain(&self) -> u8;
    fn set_form_argument_remain(&mut self, remain: u8);
    fn get_form_argument_elapsed(&self) -> u8;
    fn set_form_argument_elapsed(&mut self, elapsed: u8);
    fn get_form_argument_maximum(&self) -> u8;
    fn set_form_argument_maximum(&mut self, maximum: u8);

    fn change_form_argument(
        &mut self,
        species: usize,
        form: usize,
        generation: usize,
        value: usize,
    ) {
        if !is_form_argument_type_date_pair(species, form) {
            self.set_form_argument(value);
            return;
        }

        let max = get_form_argument_max(species, form, generation);
        self.set_form_argument_remain(value as u8);
        if value == max {
            self.set_form_argument_elapsed(0);
            self.set_form_argument_maximum(0);
            return;
        }

        let elapsed = if max < value {
            0u8
        } else {
            (max - value) as u8
        };
        self.set_form_argument_elapsed(elapsed);
        if species == Species::Furfrou as usize {
            self.set_form_argument_maximum(self.get_form_argument_maximum().max(elapsed));
        }
    }
}

pub mod form_argument_util {
    use super::FormArgument;
    use crate::{AlcremieDecoration, PersonalInfo, Pkm, Species};

    pub fn set_suggested_form_argument<I: PersonalInfo, T: FormArgument + Pkm<I>>(
        pkm: &mut T,
        original_species: usize,
    ) {
        if !is_form_argument_type_date_pair(pkm.get_species(), pkm.get_form()) {
            let suggest = match original_species {
                562 if pkm.get_species() == Species::Runerigus as usize => 49,
                211 if pkm.get_species() == Species::Overqwil as usize => 20,
                234 if pkm.get_species() == Species::Wyrdeer as usize => 20,
                550 if pkm.get_species() == Species::Basculegion as usize => 294,
                _ => 0,
            };
            pkm.change_form_argument(pkm.get_species(), pkm.get_form(), pkm.format(), suggest);
            return;
        }
        let max = get_form_argument_max(pkm.get_species(), pkm.get_form(), pkm.format());
        pkm.change_form_argument(pkm.get_species(), pkm.get_form(), pkm.format(), max);
    }

    pub fn get_form_argument_max(species: usize, form: usize, generation: usize) -> usize {
        if generation <= 5 {
            return 0;
        }

        match species {
            676 if form != 0 => 5,
            720 if form == 1 => 3,
            562 if form == 1 => 9999,
            867 if form == 0 => 9999,
            869 => AlcremieDecoration::Ribbon as usize,
            211 | 904 if generation == 8 => 9999,
            234 | 899 if generation == 8 => 9999,
            550 if form == 2 => 9999,
            902 => 9999,
            _ => 0,
        }
    }

    pub fn is_form_argument_type_date_pair(species: usize, form: usize) -> bool {
        match species {
            676 if form != 0 => true, // Furfrou
            720 if form == 1 => true, // Hoopa
            _ => false,
        }
    }
}

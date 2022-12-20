use crate::game::enums::Species;
use crate::pkm::enums::AlchremieDecoration;

pub trait FormArgument {
    fn form_argument(&self) -> u32;
    fn set_form_argument(&mut self, arg: u32);
    fn form_argument_remain(&self) -> u8;
    fn set_form_argument_remain(&mut self, remain: u8);
    fn form_argument_elapsed(&self) -> u8;
    fn set_form_argument_elapsed(&mut self, elapsed: u8);
    fn form_argument_maximum(&self) -> u8;
    fn set_form_argument_maximum(&mut self, maximum: u8);

    fn change_form_argument(&mut self, species: u16, form: u8, generation: u8, value: u32) {
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

        let elapsed = if max < value { 0 } else { (max - value) as u8 };
        self.set_form_argument_elapsed(elapsed);
        if species == Species::Furfrou as u16 {
            self.set_form_argument_maximum(self.form_argument_maximum().max(elapsed));
        }
    }
}

pub fn get_form_argument_max(species: u16, form: u8, generation: u8) -> u32 {
    if generation <= 5 {
        return 0;
    }

    match species {
        i if i == Species::Furfrou as u16 && form != 0 => 5,
        i if i == Species::Hoopa as u16 && form == 1 => 3,
        i if i == Species::Yamask as u16 && form == 1 => 9999,
        i if i == Species::Runerigus as u16 && form == 0 => 9999,
        i if i == Species::Alcremie as u16 => AlchremieDecoration::Ribbon as u32,
        i if i == Species::Qwilfish as u16 && form == 1 && generation >= 8 => 9999,
        i if i == Species::Overqwil as u16 => 9999,
        i if [Species::Stantler as u16, Species::Wyrdeer as u16].contains(&i)
            && generation >= 8 =>
        {
            9999
        }
        i if i == Species::Basculin as u16 && form == 2 => 9999,
        i if i == Species::Basculegion as u16 => 9999,
        i if [Species::Primeape as u16, Species::Annihilape as u16].contains(&i)
            && generation >= 9 =>
        {
            9999
        }
        i if [Species::Bisharp as u16, Species::Kingambit as u16].contains(&i)
            && generation >= 9 =>
        {
            9999
        }
        i if i == Species::Gholdengo as u16 => 999,
        i if [Species::Koraidon as u16, Species::Miraidon as u16].contains(&i) => 1,
        _ => 0,
    }
}

pub fn get_form_argument_min_evolution(current_species: u16, original_species: u16) -> u32 {
    match original_species {
        i if i == Species::Yamask as u16 && current_species == Species::Runerigus as u16 => 49,
        i if i == Species::Qwilfish as u16 && current_species == Species::Overqwil as u16 => 20,
        i if i == Species::Stantler as u16 && current_species == Species::Wyrdeer as u16 => 20,
        i if i == Species::Basculin as u16 && current_species == Species::Basculegion as u16 => 294,
        i if [Species::Mankey as u16, Species::Primeape as u16].contains(&i)
            && current_species == Species::Annihilape as u16 =>
        {
            20
        }
        i if [Species::Pawniard as u16, Species::Bisharp as u16].contains(&i)
            && current_species == Species::Kingambit as u16 =>
        {
            3
        }
        i if i == Species::Gimmighoul as u16 && current_species == Species::Gholdengo as u16 => 999,
        _ => 0,
    }
}

pub fn is_form_argument_type_date_pair(species: u16, form: u8) -> bool {
    match species {
        i if i == Species::Furfrou as u16 && form != 0 => true,
        i if i == Species::Hoopa as u16 && form == 1 => true,
        _ => false,
    }
}

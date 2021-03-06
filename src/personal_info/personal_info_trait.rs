pub trait PersonalInfo: Sized {
    fn new(data: Vec<u8>) -> Self;

    fn get_data(&self) -> &Vec<u8>;

    fn write(&mut self) -> Vec<u8>;

    fn get_hp(&self) -> usize;
    fn set_hp(&mut self, hp: usize);
    fn get_atk(&self) -> usize;
    fn set_atk(&mut self, atk: usize);
    fn get_def(&self) -> usize;
    fn set_def(&mut self, def: usize);
    fn get_spe(&self) -> usize;
    fn set_spe(&mut self, spe: usize);
    fn get_spa(&self) -> usize;
    fn set_spa(&mut self, spa: usize);
    fn get_spd(&self) -> usize;
    fn set_spd(&mut self, spd: usize);

    fn stats(&self) -> [usize; 6] {
        [
            self.get_hp(),
            self.get_atk(),
            self.get_def(),
            self.get_spe(),
            self.get_spa(),
            self.get_spd(),
        ]
    }

    fn get_ev_hp(&self) -> usize;
    fn set_ev_hp(&mut self, hp: usize);
    fn get_ev_atk(&self) -> usize;
    fn set_ev_atk(&mut self, atk: usize);
    fn get_ev_def(&self) -> usize;
    fn set_ev_def(&mut self, def: usize);
    fn get_ev_spe(&self) -> usize;
    fn set_ev_spe(&mut self, spe: usize);
    fn get_ev_spa(&self) -> usize;
    fn set_ev_spa(&mut self, spa: usize);
    fn get_ev_spd(&self) -> usize;
    fn set_ev_spd(&mut self, spd: usize);

    fn get_type_1(&self) -> usize;
    fn set_type_1(&mut self, type_1: usize);
    fn get_type_2(&self) -> usize;
    fn set_type_2(&mut self, type_2: usize);

    fn get_egg_group_1(&self) -> usize;
    fn set_egg_group_1(&mut self, egg_group_1: usize);
    fn get_egg_group_2(&self) -> usize;
    fn set_egg_group_2(&mut self, egg_group_2: usize);

    fn get_catch_rate(&self) -> usize;
    fn set_catch_rate(&mut self, catch_rate: usize);

    fn get_evo_stage(&self) -> usize {
        0
    }
    fn set_evo_stage(&mut self, _evo_stage: usize) {}

    fn get_items(&self) -> Vec<usize>;
    fn set_items(&mut self, items: Vec<usize>);

    fn get_gender(&self) -> usize;
    fn set_gender(&mut self, gender: usize);

    fn get_hatch_cycles(&self) -> usize;
    fn set_hatch_cycles(&mut self, hatch_cycles: usize);

    fn get_base_friendship(&self) -> usize;
    fn set_base_friendship(&mut self, base_friendship: usize);

    fn get_exp_growth(&self) -> usize;
    fn set_exp_growth(&mut self, exp_growth: usize);

    fn get_abilities(&self) -> Vec<usize>;
    fn set_abilities(&mut self, abilities: Vec<usize>);

    fn get_ability_index(&self, ability_id: usize) -> Option<usize>;

    fn get_escape_rate(&self) -> usize;
    fn set_escape_rate(&mut self, escape_rate: usize);

    fn get_form_count(&self) -> usize {
        1
    }
    fn set_form_count(&mut self, _form_count: usize) {}

    fn get_form_stats_index(&self) -> usize {
        0
    }
    fn set_form_stats_index(&mut self, _form_stats_index: usize) {}

    fn get_form_sprite(&self) -> usize {
        0
    }
    fn set_form_sprite(&mut self, _form_sprite: usize) {}

    fn get_base_exp(&self) -> usize;
    fn set_base_exp(&mut self, base_exp: usize);

    fn get_color(&self) -> usize;
    fn set_color(&mut self, color: usize);

    fn get_height(&self) -> usize {
        0
    }
    fn set_height(&mut self, _height: usize) {}

    fn get_weight(&self) -> usize {
        0
    }
    fn set_weight(&mut self, _weight: usize) {}

    fn get_tmhm(&self) -> Vec<bool> {
        vec![]
    }
    fn set_tmhm(&mut self, tmhm: Vec<bool>);

    fn get_type_tutors(&self) -> Vec<bool> {
        vec![]
    }
    fn set_type_tutors(&mut self, type_tutors: Vec<bool>);

    fn get_special_tutors(&self) -> Vec<Vec<bool>> {
        vec![]
    }
    fn set_special_tutors(&mut self, special_tutors: Vec<Vec<bool>>);

    fn form_index(&self, species: usize, form: usize) -> usize {
        if !self.has_form(form) {
            species
        } else {
            self.get_form_stats_index() + form - 1
        }
    }

    fn has_form(&self, form: usize) -> bool {
        !(form == 0 || self.get_form_stats_index() == 0 || form >= self.get_form_count())
    }

    fn is_dual_gender(&self) -> bool {
        self.get_gender() - 1 < 253
    }

    fn genderless(&self) -> bool {
        self.get_gender() == RATIO_MAGIC_GENDERLESS
    }

    fn only_female(&self) -> bool {
        self.get_gender() == RATIO_MAGIC_FEMALE
    }

    fn only_male(&self) -> bool {
        self.get_gender() == RATIO_MAGIC_MALE
    }

    fn has_forms(&self) -> bool {
        self.get_form_count() > 1
    }

    fn bst(&self) -> usize {
        self.get_hp()
            + self.get_atk()
            + self.get_def()
            + self.get_spe()
            + self.get_spa()
            + self.get_spd()
    }

    fn is_form_within_range(&self, form: usize) -> bool {
        if form == 0 {
            true
        } else {
            form < self.get_form_count()
        }
    }

    fn is_valid_type_combination(&self, type_1: usize, type_2: usize) -> bool {
        self.get_type_1() == type_1 && self.get_type_2() == type_2
    }

    fn is_type_single(&self, type_1: usize) -> bool {
        self.get_type_1() == type_1 || self.get_type_2() == type_1
    }

    fn is_type_full(&self, type_1: usize, type_2: usize) -> bool {
        (self.get_type_1() == type_1 && self.get_type_2() == type_2)
            || (self.get_type_1() == type_2 || self.get_type_2() == type_1)
    }

    fn is_egg_group(&self, group: usize) -> bool {
        self.get_egg_group_1() == group || self.get_egg_group_2() == group
    }
}

pub const RATIO_MAGIC_GENDERLESS: usize = 255;
pub const RATIO_MAGIC_FEMALE: usize = 254;
pub const RATIO_MAGIC_MALE: usize = 0;

pub fn get_bits(data: &[u8]) -> Vec<bool> {
    let len = data.len() << 3;
    let mut result = vec![false; len];
    for i in (0..len).rev() {
        result[i] = (data[i >> 3] >> (i & 7) & 0x1) == 1;
    }
    result
}

pub fn set_bits(data: &mut [u8], bits: &[bool]) {
    for bit in (0..bits.len()).rev() {
        data[bit >> 3] |= if bits[bit] { 1 << (bit & 0x7) } else { 0 }
    }
}

pub fn is_single_gender(gt: usize) -> bool {
    gt > 253
}

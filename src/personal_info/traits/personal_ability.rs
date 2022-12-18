pub trait PersonalAbility {
    fn get_index_of_ability(&self, ability_id: u16) -> Option<u8>;
    fn get_ability_at_index(&self, ability_index: usize) -> Option<u16>;
    fn get_ability_count(&self) -> u8;

    fn get_abilities(&self, _result: &mut [u16]) {}
}

pub trait PersonalAbility12: PersonalAbility {
    fn ability_1(&self) -> u16;
    fn ability_2(&self) -> u16;

    fn get_is_ability_12_same(&self) -> bool {
        self.ability_1() == self.ability_2()
    }

    fn get_abilities(&self, result: &mut [u16]) {
        result[0] = self.ability_1();
        result[1] = self.ability_2();
    }
}

pub trait PersonalAbility12H: PersonalAbility12 {
    fn ability_h(&self) -> u16;

    fn get_is_ability_hidden_unique(&self) -> bool {
        self.ability_1() != self.ability_h()
    }

    fn get_is_ability_patch_possible(&self) -> bool {
        self.ability_1() != self.ability_h() || self.ability_2() != self.ability_h()
    }

    fn get_abilities(&self, result: &mut [u16]) {
        <Self as PersonalAbility12>::get_abilities(self, result);
        result[2] = self.ability_h();
    }
}

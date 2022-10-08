use std::ops::Index;

#[derive(Copy, Clone)]
pub struct IndividualValueSet {
    pub hp: Option<u8>,
    pub atk: Option<u8>,
    pub def: Option<u8>,
    pub spe: Option<u8>,
    pub spa: Option<u8>,
    pub spd: Option<u8>,
    pub set_type: u8,
}

impl IndividualValueSet {
    pub fn is_specified(&self) -> bool {
        self.set_type != 0
    }

    pub fn copy_to_speed_last(&self, slice: &mut [Option<u8>; 6]) {
        slice[5] = self.spe;
        slice[4] = self.spd;
        slice[3] = self.spa;
        slice[2] = self.def;
        slice[1] = self.atk;
        slice[0] = self.hp;
    }
}

impl Index<usize> for IndividualValueSet {
    type Output = Option<u8>;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.hp,
            1 => &self.atk,
            2 => &self.def,
            3 => &self.spe,
            4 => &self.spa,
            5 => &self.spd,
            _ => panic!("Index out of range for Individual Value Set"),
        }
    }
}

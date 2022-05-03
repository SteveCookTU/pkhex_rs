use rand::Rng;

#[derive(PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum PokeSize {
    XS,
    S,
    AV,
    L,
    XL,
}

impl PokeSize {
    pub fn get_size_rating(scalar: u8) -> PokeSize {
        match scalar {
            _ if scalar < 0x10 => PokeSize::XS,
            _ if scalar < 0x30 => PokeSize::S,
            _ if scalar < 0xD0 => PokeSize::AV,
            _ if scalar < 0xF0 => PokeSize::L,
            _ => PokeSize::XL,
        }
    }

    pub fn get_random_scalar_from_self(&self) -> u8 {
        let mut rand = rand::thread_rng();
        match self {
            PokeSize::XS => rand.gen_range(0..0x10),
            PokeSize::S => rand.gen_range(0..0x20) + 0x10,
            PokeSize::AV => rand.gen_range(0..0xA0) + 0x30,
            PokeSize::L => rand.gen_range(0..0x20) + 0xD0,
            PokeSize::XL => rand.gen_range(0..0x10) + 0xF0,
        }
    }

    pub fn get_random_scalar() -> u8 {
        let mut rand = rand::thread_rng();
        rand.gen_range(0..0x81) + rand.gen_range(0..0x80)
    }
}

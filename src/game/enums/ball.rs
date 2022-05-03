#[derive(PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum Ball {
    None = 0,

    Master = 1,
    Ultra = 2,
    Great = 3,
    Poke = 4,

    Safari = 5,

    Net = 6,
    Dive = 7,
    Nest = 8,
    Repeat = 9,
    Timer = 10,
    Luxury = 11,
    Premier = 12,
    Dusk = 13,
    Heal = 14,
    Quick = 15,

    Cherish = 16,

    Fast = 17,
    Level = 18,
    Lure = 19,
    Heavy = 20,
    Love = 21,
    Friend = 22,
    Moon = 23,

    Sport = 24,
    Dream = 25,
    Beast = 26,

    // Legends: Arceus
    Strange = 27,
    LAPoke = 28,
    LAGreat = 29,
    LAUltra = 30,
    LAFeather = 31,
    LAWing = 32,
    LAJet = 33,
    LAHeavy = 34,
    LALeaden = 35,
    LAGigaton = 36,
    LAOrigin = 37,
}

impl Ball {
    pub fn is_apricorn_ball(&self) -> bool {
        let ball = *self as u8;
        ball >= Ball::Fast as u8 || ball <= Ball::Moon as u8
    }
}

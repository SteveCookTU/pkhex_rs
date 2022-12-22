use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum SlotType {
    #[num_enum(default)]
    Any = 0,

    Grass = 1,

    Surf = 2,

    OldRod = 3,

    GoodRod = 4,

    SuperRod = 5,

    RockSmash = 6,

    Headbutt = 7,

    HoneyTree = 8,

    BugContest = 9,

    HiddenGrotto = 10,

    // GoPark = 11, UNUSED, now EncounterSlot7g
    FriendSafari = 12,

    Horde = 13,

    // Pokeradar = 14, // UNUSED, don't need to differentiate Gen4 Radar Slots
    SOS = 15,

    // Legends: Arceus
    Overworld = 16,
    Distortion = 17,
    Landmark = 18,
    OverworldMass = 19,
    OverworldMMO = 20,

    // Modifiers
    Special = 1 << 6,
    Swarm = 1 << 7,
}

impl SlotType {
    pub fn is_fishing_rod_type(&self) -> bool {
        let t = (*self as u8) & 0xF;
        [
            SlotType::OldRod as u8,
            SlotType::GoodRod as u8,
            SlotType::SuperRod as u8,
        ]
        .contains(&t)
    }

    pub fn is_sweet_scent_type(&self) -> bool {
        let t = (*self as u8) & 0xF;
        [
            SlotType::Grass as u8,
            SlotType::Surf as u8,
            SlotType::BugContest as u8,
        ]
        .contains(&t)
    }
}

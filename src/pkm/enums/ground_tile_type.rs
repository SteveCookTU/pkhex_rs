use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum GroundTileType {
    #[num_enum(default)]
    None = 0, // No animation for the tile
    Sand = 1, // Obtainable only via HG/SS
    Grass = 2,
    Puddle = 3, // No encounters from this tile type
    Rock = 4,
    Cave = 5,
    Snow = 6, // No encounters from this tile type
    Water = 7,
    Ice = 8, // No encounters from this tile type
    Building = 9,
    Marsh = 10,
    Bridge = 11, // No encounters from this tile type
    MaxDP = 12,
    // added tile types in Pt
    // no encounters from these tile types
    Elite42 = 13,            // Elite Four Room #2
    Elite43 = 14,            // Elite Four Room #3
    Elite44 = 15,            // Elite Four Room #4
    Elite4M = 16,            // Elite Four Champion Room
    DistortionSideways = 17, // Distortion World (Not Giratina)
    BattleTower = 18,
    BattleFactory = 19,
    BattleArcade = 20,
    BattleCastle = 21,
    BattleHall = 22,

    Distortion = 23,
    MaxPt = 24, // Unspecific, catch-all for Pt undefined tiles.
}

impl GroundTileType {
    pub fn is_obtainable(&self) -> bool {
        let val = *self as usize;
        ((0b1_1000_0000_0001_0110_1011_0111 >> val) & 1) == 1
    }

    pub const VALID_TILE_TYPES: [u8; 11] = [
        GroundTileType::None as u8,
        GroundTileType::Sand as u8,
        GroundTileType::Grass as u8,
        GroundTileType::Rock as u8,
        GroundTileType::Cave as u8,
        GroundTileType::Water as u8,
        GroundTileType::Building as u8,
        GroundTileType::Marsh as u8,
        GroundTileType::MaxDP as u8,
        GroundTileType::Distortion as u8,
        GroundTileType::MaxPt as u8,
    ];
}

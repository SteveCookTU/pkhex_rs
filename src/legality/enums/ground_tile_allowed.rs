use crate::pkm::enums::GroundTileType;
use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum GroundTileAllowed {
    #[num_enum(default)]
    Undefined = 0,
    None = 1 << 00, // No animation for the tile
    Sand = 1 << 01, // Obtainable only via HG/SS
    Grass = 1 << 02,
    Puddle = 1 << 03, // No encounters from this tile type
    Rock = 1 << 04,
    Cave = 1 << 05,
    Snow = 1 << 06, // No encounters from this tile type
    Water = 1 << 07,
    Ice = 1 << 08, // No encounters from this tile type
    Building = 1 << 09,
    Marsh = 1 << 10,
    Bridge = 1 << 11, // No encounters from this tile type
    MaxDP = 1 << 12,  // Unspecific, catch-all for DP undefined tiles.

    Elite4_2 = 1 << 13,           // Elite Four Room #2
    Elite4_3 = 1 << 14,           // Elite Four Room #3
    Elite4_4 = 1 << 15,           // Elite Four Room #4
    Elite4M = 1 << 16,            // Elite Four Champion Room
    DistortionSideways = 1 << 17, // Distortion World (Not Giratina)
    BattleTower = 1 << 18,
    BattleFactory = 1 << 19,
    BattleArcade = 1 << 20,
    BattleCastle = 1 << 21,
    BattleHall = 1 << 22,

    Distortion = 1 << 23,
    MaxPt = 1 << 24, // Unspecific, catch-all for Pt undefined tiles.
}

impl GroundTileAllowed {
    pub fn contains(&self, g2: GroundTileType) -> bool {
        (*self as usize) & (1 << (g2 as usize)) != 0
    }

    pub fn get_index(&self) -> GroundTileType {
        let mut val = *self as usize;
        for i in 0..(8 * 25) {
            val >>= 1;
            if val == 0 {
                return GroundTileType::from(i);
            }
        }
        GroundTileType::from(0)
    }
}

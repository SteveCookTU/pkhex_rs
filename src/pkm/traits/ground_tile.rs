use crate::GroundTileType;

pub trait GroundTile {
    fn get_ground_tile(&self) -> GroundTileType;
    fn set_ground_tile(&mut self, tile: GroundTileType);
}

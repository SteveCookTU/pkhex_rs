use crate::ribbons::{RibbonSetCommon3, RibbonSetEvent3, RibbonSetOnly3, RibbonSetUnique3};
use crate::traits::{ContestStats, ContestStatsMutable};
use crate::{GCVersion, GameVersion, PersonalInfo, PKM};

pub trait G3PKM<Personal: PersonalInfo + 'static>:
    PKM<Personal>
    + RibbonSetEvent3
    + RibbonSetCommon3
    + RibbonSetUnique3
    + RibbonSetOnly3
    + ContestStats
    + ContestStatsMutable
{
    fn get_species_id_3(&self) -> u16;
    fn set_species_id_3(&mut self, species: u16);

    fn get_ability_bit(&self) -> bool;
    fn set_ability_bit(&mut self, value: bool);
}

pub(crate) fn swap_bits(value: u8, p1: u32, p2: u32) -> u8 {
    let bit1 = (value >> p1) & 1;
    let bit2 = (value >> p2) & 1;
    let mut x = bit1 ^ bit2;
    x = (x << p1) | (x << p2);
    value ^ x
}

pub(crate) fn get_gba_version_id(gc: u8) -> u8 {
    GameVersion::from(GCVersion::from(gc)) as u8
}

pub(crate) fn get_gc_version_id(gba: u8) -> u8 {
    GCVersion::from(GameVersion::from(gba as usize)) as u8
}

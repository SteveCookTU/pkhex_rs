use crate::poke_crypto::SIZE_8STORED;
use crate::{
    BattleVersion, ContestStats, ContestStatsMutable, DynamaxLevel, Favorite, FormArgument,
    Gigantamax, HandlerLanguage, HomeTrack, HyperTrain, PersonalInfo, Pkm, RibbonIndex,
    RibbonSetAffixed, RibbonSetCommon3, RibbonSetCommon4, RibbonSetCommon6, RibbonSetCommon7,
    RibbonSetCommon8, RibbonSetEvent3, RibbonSetEvent4, RibbonSetMark8, SanityChecksum, ScaledSize,
    Sociability, TechRecord8, TrainerMemories,
};

pub trait G8PKM<I: PersonalInfo>:
    Pkm<I>
    + SanityChecksum
    + RibbonSetEvent3
    + RibbonSetEvent4
    + RibbonSetCommon3
    + RibbonSetCommon4
    + RibbonSetCommon6
    + RibbonSetCommon8
    + RibbonSetCommon7
    + RibbonSetMark8
    + RibbonSetAffixed
    + TechRecord8
    + Sociability
    + ContestStats
    + ContestStatsMutable
    + HyperTrain
    + ScaledSize
    + Gigantamax
    + Favorite
    + DynamaxLevel
    + RibbonIndex
    + HandlerLanguage
    + FormArgument
    + HomeTrack
    + BattleVersion
    + TrainerMemories
{
    fn calculate_checksum(&self) -> u16 {
        let mut chk = 0;
        for i in (8..SIZE_8STORED).step_by(2) {
            chk += u16::from_le_bytes(self.get_data()[i..(i + 2)].try_into().unwrap());
        }
        chk
    }

    fn fix_relearn(&mut self) {
        loop {
            if self.get_relearn_move_4() != 0 && self.get_relearn_move_3() == 0 {
                self.set_relearn_move_3(self.get_relearn_move_4());
                self.set_relearn_move_4(0);
            }
            if self.get_relearn_move_3() != 0 && self.get_relearn_move_2() == 0 {
                self.set_relearn_move_2(self.get_relearn_move_3());
                self.set_relearn_move_3(0);
                continue;
            }
            if self.get_relearn_move_2() != 0 && self.get_relearn_move_1() == 0 {
                self.set_relearn_move_1(self.get_relearn_move_2());
                self.set_relearn_move_2(0);
                continue;
            }
            break;
        }
    }

    fn get_palma(&self) -> u32;

    fn set_palma(&mut self, palma: u32);

    fn get_unk_e3(&self) -> u8;

    fn set_unk_e3(&mut self, e3: u8);
}

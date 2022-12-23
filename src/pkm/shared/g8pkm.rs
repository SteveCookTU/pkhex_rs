use crate::flag_util;
use crate::pkm::traits::form_argument::FormArgument;
use crate::pkm::traits::{
    BattleVersion, ContestStats, DynamaxLevel, Favorite, Gigantamax, HandlerLanguage, HomeTrack,
    HyperTrain, MoveReset, SanityChecksum, ScaledSize, Sociability, TechRecord, TrainerMemories,
};
use crate::pkm::Pkm;
use crate::ribbons::{
    RibbonIndex, RibbonSetAffixed, RibbonSetCommon3, RibbonSetCommon4, RibbonSetCommon6,
    RibbonSetCommon7, RibbonSetCommon8, RibbonSetEvent3, RibbonSetEvent4, RibbonSetMark8,
    RibbonSetMemory6,
};
use no_std_io::{Reader, Writer};

pub trait G8Pkm:
    Pkm
    + SanityChecksum
    + MoveReset
    + RibbonSetEvent3
    + RibbonSetEvent4
    + RibbonSetCommon3
    + RibbonSetCommon4
    + RibbonSetCommon6
    + RibbonSetMemory6
    + RibbonSetCommon7
    + RibbonSetCommon8
    + RibbonSetMark8
    + RibbonSetAffixed
    + TechRecord
    + Sociability
    + ContestStats
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
    fn palma(&self) -> u32 {
        self.data().default_read_le(0x98)
    }
    fn set_palma(&mut self, palma: u32) {
        self.data_mut().checked_write_le(0x98, &palma);
    }

    fn get_poke_job_flag(&self, index: usize) -> bool {
        if index > 112 {
            false
        } else {
            let ofs = index >> 3;
            flag_util::get_flag(self.data(), 0xCE + ofs, index & 7)
        }
    }

    fn set_poke_job_flag(&mut self, index: usize, value: bool) {
        if index > 112 {
            return;
        }
        let ofs = index >> 3;
        flag_util::set_flag(self.data_mut(), 0xCE + ofs, index & 7, value);
    }

    fn get_poke_job_flag_any(&self) -> bool {
        self.data()[0xCE..0xDC].iter().any(|&z| z != 0)
    }

    fn clear_poke_job_flags(&mut self) {
        self.data_mut()[0xCE..0xDC].fill(0);
    }
}

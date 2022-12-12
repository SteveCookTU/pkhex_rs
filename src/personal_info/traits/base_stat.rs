use crate::{PKError, PKResult};

pub trait BaseStat {
    fn hp(&self) -> u8;
    fn atk(&self) -> u8;
    fn def(&self) -> u8;
    fn spa(&self) -> u8;
    fn spd(&self) -> u8;
    fn spe(&self) -> u8;

    fn get_base_state_total(&self) -> u16 {
        self.hp() as u16
            + self.atk() as u16
            + self.def() as u16
            + self.spa() as u16
            + self.spd() as u16
            + self.spe() as u16
    }

    fn get_base_state_value(&self, index: usize) -> PKResult<u8> {
        match index {
            0 => Ok(self.hp()),
            1 => Ok(self.atk()),
            2 => Ok(self.def()),
            3 => Ok(self.spe()),
            4 => Ok(self.spa()),
            5 => Ok(self.spd()),
            _ => Err(PKError::IndexOutOfRange { index }),
        }
    }

    fn get_sorted_stat_indices(&self, stats: &mut [(usize, u8)]) -> PKResult<()> {
        for (i, stat) in stats.iter_mut().enumerate() {
            *stat = (i, self.get_base_state_value(i)?);
        }

        stats.sort_by(|(_, stat), (_, stat2)| stat2.cmp(stat));

        Ok(())
    }
}

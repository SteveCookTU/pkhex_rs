use crate::{PKError, PKResult};

pub trait BaseState {
    fn hp(&self) -> u16;
    fn atk(&self) -> u16;
    fn def(&self) -> u16;
    fn spa(&self) -> u16;
    fn spd(&self) -> u16;
    fn spe(&self) -> u16;

    fn get_base_state_total(&self) -> u16 {
        self.hp() + self.atk() + self.def() + self.spa() + self.spd() + self.spe()
    }

    fn get_base_state_value(&self, index: usize) -> PKResult<u16> {
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

    fn get_sorted_stat_indices(&self, stats: &mut [(usize, u16)]) -> PKResult<()> {
        for (i, stat) in stats.iter_mut().enumerate() {
            *stat = (i, self.get_base_state_value(i)?);
        }

        stats.sort_by(|(_, stat), (_, stat2)| stat2.cmp(stat));

        Ok(())
    }
}

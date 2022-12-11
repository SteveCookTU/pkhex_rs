use crate::{PKError, PKResult};

pub trait BaseState {
    fn hp(&self) -> i32;
    fn atk(&self) -> i32;
    fn def(&self) -> i32;
    fn spa(&self) -> i32;
    fn spd(&self) -> i32;
    fn spe(&self) -> i32;

    fn get_base_state_total(&self) -> i32 {
        self.hp() + self.atk() + self.def() + self.spa() + self.spd() + self.spe()
    }

    fn get_base_state_value(&self, index: usize) -> PKResult<i32> {
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

    fn get_sorted_stat_indices(&self, stats: &mut [(usize, i32)]) -> PKResult<()> {
        for (i, stat) in stats.iter_mut().enumerate() {
            *stat = (i, self.get_base_state_value(i)?);
        }

        stats.sort_by(|(_, stat), (_, stat2)| stat2.cmp(stat));

        Ok(())
    }
}

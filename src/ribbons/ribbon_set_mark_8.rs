use crate::ribbons::RibbonHasMark;

pub trait RibbonSetMark8: RibbonHasMark {
    fn ribbon_mark_lunchtime(&self) -> bool;
    fn set_ribbon_mark_lunchtime(&mut self, val: bool);
    fn ribbon_mark_sleepy_time(&self) -> bool;
    fn set_ribbon_mark_sleepy_time(&mut self, val: bool);
    fn ribbon_mark_dusk(&self) -> bool;
    fn set_ribbon_mark_dusk(&mut self, val: bool);
    fn ribbon_mark_dawn(&self) -> bool;
    fn set_ribbon_mark_dawn(&mut self, val: bool);
    fn ribbon_mark_cloudy(&self) -> bool;
    fn set_ribbon_mark_cloudy(&mut self, val: bool);
    fn ribbon_mark_rainy(&self) -> bool;
    fn set_ribbon_mark_rainy(&mut self, val: bool);
    fn ribbon_mark_stormy(&self) -> bool;
    fn set_ribbon_mark_stormy(&mut self, val: bool);
    fn ribbon_mark_snowy(&self) -> bool;
    fn set_ribbon_mark_snowy(&mut self, val: bool);
    fn ribbon_mark_blizzard(&self) -> bool;
    fn set_ribbon_mark_blizzard(&mut self, val: bool);
    fn ribbon_mark_dry(&self) -> bool;
    fn set_ribbon_mark_dry(&mut self, val: bool);
    fn ribbon_mark_sandstorm(&self) -> bool;
    fn set_ribbon_mark_sandstorm(&mut self, val: bool);
    fn ribbon_mark_misty(&self) -> bool;
    fn set_ribbon_mark_misty(&mut self, val: bool);
    fn ribbon_mark_destiny(&self) -> bool;
    fn set_ribbon_mark_destiny(&mut self, val: bool);
    fn ribbon_mark_fishing(&self) -> bool;
    fn set_ribbon_mark_fishing(&mut self, val: bool);
    fn ribbon_mark_curry(&self) -> bool;
    fn set_ribbon_mark_curry(&mut self, val: bool);
    fn ribbon_mark_uncommon(&self) -> bool;
    fn set_ribbon_mark_uncommon(&mut self, val: bool);
    fn ribbon_mark_rare(&self) -> bool;
    fn set_ribbon_mark_rare(&mut self, val: bool);
    fn ribbon_mark_rowdy(&self) -> bool;
    fn set_ribbon_mark_rowdy(&mut self, val: bool);
    fn ribbon_mark_absent_minded(&self) -> bool;
    fn set_ribbon_mark_absent_minded(&mut self, val: bool);
    fn ribbon_mark_jittery(&self) -> bool;
    fn set_ribbon_mark_jittery(&mut self, val: bool);
    fn ribbon_mark_excited(&self) -> bool;
    fn set_ribbon_mark_excited(&mut self, val: bool);
    fn ribbon_mark_charismatic(&self) -> bool;
    fn set_ribbon_mark_charismatic(&mut self, val: bool);
    fn ribbon_mark_calmness(&self) -> bool;
    fn set_ribbon_mark_calmness(&mut self, val: bool);
    fn ribbon_mark_intense(&self) -> bool;
    fn set_ribbon_mark_intense(&mut self, val: bool);
    fn ribbon_mark_zoned_out(&self) -> bool;
    fn set_ribbon_mark_zoned_out(&mut self, val: bool);
    fn ribbon_mark_joyful(&self) -> bool;
    fn set_ribbon_mark_joyful(&mut self, val: bool);
    fn ribbon_mark_angry(&self) -> bool;
    fn set_ribbon_mark_angry(&mut self, val: bool);
    fn ribbon_mark_smiley(&self) -> bool;
    fn set_ribbon_mark_smiley(&mut self, val: bool);
    fn ribbon_mark_teary(&self) -> bool;
    fn set_ribbon_mark_teary(&mut self, val: bool);
    fn ribbon_mark_upbeat(&self) -> bool;
    fn set_ribbon_mark_upbeat(&mut self, val: bool);
    fn ribbon_mark_peeved(&self) -> bool;
    fn set_ribbon_mark_peeved(&mut self, val: bool);
    fn ribbon_mark_intellectual(&self) -> bool;
    fn set_ribbon_mark_intellectual(&mut self, val: bool);
    fn ribbon_mark_ferocious(&self) -> bool;
    fn set_ribbon_mark_ferocious(&mut self, val: bool);
    fn ribbon_mark_crafty(&self) -> bool;
    fn set_ribbon_mark_crafty(&mut self, val: bool);
    fn ribbon_mark_scowling(&self) -> bool;
    fn set_ribbon_mark_scowling(&mut self, val: bool);
    fn ribbon_mark_kindly(&self) -> bool;
    fn set_ribbon_mark_kindly(&mut self, val: bool);
    fn ribbon_mark_flustered(&self) -> bool;
    fn set_ribbon_mark_flustered(&mut self, val: bool);
    fn ribbon_mark_pumped_up(&self) -> bool;
    fn set_ribbon_mark_pumped_up(&mut self, val: bool);
    fn ribbon_mark_zero_energy(&self) -> bool;
    fn set_ribbon_mark_zero_energy(&mut self, val: bool);
    fn ribbon_mark_prideful(&self) -> bool;
    fn set_ribbon_mark_prideful(&mut self, val: bool);
    fn ribbon_mark_unsure(&self) -> bool;
    fn set_ribbon_mark_unsure(&mut self, val: bool);
    fn ribbon_mark_humble(&self) -> bool;
    fn set_ribbon_mark_humble(&mut self, val: bool);
    fn ribbon_mark_thorny(&self) -> bool;
    fn set_ribbon_mark_thorny(&mut self, val: bool);
    fn ribbon_mark_vigor(&self) -> bool;
    fn set_ribbon_mark_vigor(&mut self, val: bool);
    fn ribbon_mark_slump(&self) -> bool;
    fn set_ribbon_mark_slump(&mut self, val: bool);

    fn has_weather_mark(&self) -> bool {
        self.ribbon_mark_cloudy()
            || self.ribbon_mark_rainy()
            || self.ribbon_mark_stormy()
            || self.ribbon_mark_snowy()
            || self.ribbon_mark_blizzard()
            || self.ribbon_mark_dry()
            || self.ribbon_mark_sandstorm()
            || self.ribbon_mark_misty()
    }

    fn ribbon_bits(&self) -> Vec<bool> {
        vec![
            self.ribbon_mark_lunchtime(),
            self.ribbon_mark_sleepy_time(),
            self.ribbon_mark_dusk(),
            self.ribbon_mark_dawn(),
            self.ribbon_mark_cloudy(),
            self.ribbon_mark_rainy(),
            self.ribbon_mark_stormy(),
            self.ribbon_mark_snowy(),
            self.ribbon_mark_blizzard(),
            self.ribbon_mark_dry(),
            self.ribbon_mark_sandstorm(),
            self.ribbon_mark_misty(),
            self.ribbon_mark_destiny(),
            self.ribbon_mark_fishing(),
            self.ribbon_mark_curry(),
            self.ribbon_mark_uncommon(),
            self.ribbon_mark_rare(),
            self.ribbon_mark_rowdy(),
            self.ribbon_mark_absent_minded(),
            self.ribbon_mark_jittery(),
            self.ribbon_mark_excited(),
            self.ribbon_mark_charismatic(),
            self.ribbon_mark_calmness(),
            self.ribbon_mark_intense(),
            self.ribbon_mark_zoned_out(),
            self.ribbon_mark_joyful(),
            self.ribbon_mark_angry(),
            self.ribbon_mark_smiley(),
            self.ribbon_mark_teary(),
            self.ribbon_mark_upbeat(),
            self.ribbon_mark_peeved(),
            self.ribbon_mark_intellectual(),
            self.ribbon_mark_ferocious(),
            self.ribbon_mark_crafty(),
            self.ribbon_mark_scowling(),
            self.ribbon_mark_kindly(),
            self.ribbon_mark_flustered(),
            self.ribbon_mark_pumped_up(),
            self.ribbon_mark_zero_energy(),
            self.ribbon_mark_prideful(),
            self.ribbon_mark_unsure(),
            self.ribbon_mark_humble(),
            self.ribbon_mark_thorny(),
            self.ribbon_mark_vigor(),
            self.ribbon_mark_slump(),
        ]
    }
}

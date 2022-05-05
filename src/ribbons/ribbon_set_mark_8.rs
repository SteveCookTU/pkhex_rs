pub const RIBBON_SET_MARK_8_NAMES: [&str; 45] = [
    "RibbonMarkLunchtime",
    "RibbonMarkSleepytime",
    "RibbonMarkDusk",
    "RibbonMarkDawn",
    "RibbonMarkCloudy",
    "RibbonMarkEainy",
    "RibbonMarkStormy",
    "RibbonMarkSnowy",
    "RibbonMarkBlizzard",
    "RibbonMarkDry",
    "RibbonMarkSandstorm",
    "RibbonMarkMisty",
    "RibbonMarkDestiny",
    "RibbonMarkFishing",
    "RibbonMarkCurry",
    "RibbonMarkUncommon",
    "RibbonMarkRare",
    "RibbonMarkRowdy",
    "RibbonMarkAbsentMinded",
    "RibbonMarkJittery",
    "RibbonMarkExcited",
    "RibbonMarkCharismatic",
    "RibbonMarkCalmness",
    "RibbonMarkIntense",
    "RibbonMarkZonedOut",
    "RibbonMarkJoyful",
    "RibbonMarkAngry",
    "RibbonMarkSmiley",
    "RibbonMarkTeary",
    "RibbonMarkUpbeat",
    "RibbonMarkPeeved",
    "RibbonMarkIntellectual",
    "RibbonMarkFerocious",
    "RibbonMarkCrafty",
    "RibbonMarkScowling",
    "RibbonMarkKindly",
    "RibbonMarkFlustered",
    "RibbonMarkPumpedUp",
    "RibbonMarkZeroEnergy",
    "RibbonMarkPrideful",
    "RibbonMarkUnsure",
    "RibbonMarkHumble",
    "RibbonMarkThorny",
    "RibbonMarkVigor",
    "RibbonMarkSlump",
];

pub trait RibbonSetMark8 {
    fn get_ribbon_mark_lunchtime(&self) -> bool;
    fn set_ribbon_mark_lunchtime(&mut self, flag: bool);
    fn get_ribbon_mark_sleepytime(&self) -> bool;
    fn set_ribbon_mark_sleepytime(&mut self, flag: bool);
    fn get_ribbon_mark_dusk(&self) -> bool;
    fn set_ribbon_mark_dusk(&mut self, flag: bool);
    fn get_ribbon_mark_dawn(&self) -> bool;
    fn set_ribbon_mark_dawn(&mut self, flag: bool);
    fn get_ribbon_mark_cloudy(&self) -> bool;
    fn set_ribbon_mark_cloudy(&mut self, flag: bool);
    fn get_ribbon_mark_rainy(&self) -> bool;
    fn set_ribbon_mark_rainy(&mut self, flag: bool);
    fn get_ribbon_mark_stormy(&self) -> bool;
    fn set_ribbon_mark_stormy(&mut self, flag: bool);
    fn get_ribbon_mark_snowy(&self) -> bool;
    fn set_ribbon_mark_snowy(&mut self, flag: bool);
    fn get_ribbon_mark_blizzard(&self) -> bool;
    fn set_ribbon_mark_blizzard(&mut self, flag: bool);
    fn get_ribbon_mark_dry(&self) -> bool;
    fn set_ribbon_mark_dry(&mut self, flag: bool);
    fn get_ribbon_mark_sandstorm(&self) -> bool;
    fn set_ribbon_mark_sandstorm(&mut self, flag: bool);
    fn get_ribbon_mark_misty(&self) -> bool;
    fn set_ribbon_mark_misty(&mut self, flag: bool);
    fn get_ribbon_mark_destiny(&self) -> bool;
    fn set_ribbon_mark_destiny(&mut self, flag: bool);
    fn get_ribbon_mark_fishing(&self) -> bool;
    fn set_ribbon_mark_fishing(&mut self, flag: bool);
    fn get_ribbon_mark_curry(&self) -> bool;
    fn set_ribbon_mark_curry(&mut self, flag: bool);
    fn get_ribbon_mark_uncommon(&self) -> bool;
    fn set_ribbon_mark_uncommon(&mut self, flag: bool);
    fn get_ribbon_mark_rare(&self) -> bool;
    fn set_ribbon_mark_rare(&mut self, flag: bool);
    fn get_ribbon_mark_rowdy(&self) -> bool;
    fn set_ribbon_mark_rowdy(&mut self, flag: bool);
    fn get_ribbon_mark_absent_minded(&self) -> bool;
    fn set_ribbon_mark_absent_minded(&mut self, flag: bool);
    fn get_ribbon_mark_jittery(&self) -> bool;
    fn set_ribbon_mark_jittery(&mut self, flag: bool);
    fn get_ribbon_mark_excited(&self) -> bool;
    fn set_ribbon_mark_excited(&mut self, flag: bool);
    fn get_ribbon_mark_charismatic(&self) -> bool;
    fn set_ribbon_mark_charismatic(&mut self, flag: bool);
    fn get_ribbon_mark_calmness(&self) -> bool;
    fn set_ribbon_mark_calmness(&mut self, flag: bool);
    fn get_ribbon_mark_intense(&self) -> bool;
    fn set_ribbon_mark_intense(&mut self, flag: bool);
    fn get_ribbon_mark_zoned_out(&self) -> bool;
    fn set_ribbon_mark_zoned_out(&mut self, flag: bool);
    fn get_ribbon_mark_joyful(&self) -> bool;
    fn set_ribbon_mark_joyful(&mut self, flag: bool);
    fn get_ribbon_mark_angry(&self) -> bool;
    fn set_ribbon_mark_angry(&mut self, flag: bool);
    fn get_ribbon_mark_smiley(&self) -> bool;
    fn set_ribbon_mark_smiley(&mut self, flag: bool);
    fn get_ribbon_mark_teary(&self) -> bool;
    fn set_ribbon_mark_teary(&mut self, flag: bool);
    fn get_ribbon_mark_upbeat(&self) -> bool;
    fn set_ribbon_mark_upbeat(&mut self, flag: bool);
    fn get_ribbon_mark_peeved(&self) -> bool;
    fn set_ribbon_mark_peeved(&mut self, flag: bool);
    fn get_ribbon_mark_intellectual(&self) -> bool;
    fn set_ribbon_mark_intellectual(&mut self, flag: bool);
    fn get_ribbon_mark_ferocious(&self) -> bool;
    fn set_ribbon_mark_ferocious(&mut self, flag: bool);
    fn get_ribbon_mark_crafty(&self) -> bool;
    fn set_ribbon_mark_crafty(&mut self, flag: bool);
    fn get_ribbon_mark_scowling(&self) -> bool;
    fn set_ribbon_mark_scowling(&mut self, flag: bool);
    fn get_ribbon_mark_kindly(&self) -> bool;
    fn set_ribbon_mark_kindly(&mut self, flag: bool);
    fn get_ribbon_mark_flustered(&self) -> bool;
    fn set_ribbon_mark_flustered(&mut self, flag: bool);
    fn get_ribbon_mark_pumped_up(&self) -> bool;
    fn set_ribbon_mark_pumped_up(&mut self, flag: bool);
    fn get_ribbon_mark_zero_energy(&self) -> bool;
    fn set_ribbon_mark_zero_energy(&mut self, flag: bool);
    fn get_ribbon_mark_prideful(&self) -> bool;
    fn set_ribbon_mark_prideful(&mut self, flag: bool);
    fn get_ribbon_mark_unsure(&self) -> bool;
    fn set_ribbon_mark_unsure(&mut self, flag: bool);
    fn get_ribbon_mark_humble(&self) -> bool;
    fn set_ribbon_mark_humble(&mut self, flag: bool);
    fn get_ribbon_mark_thorny(&self) -> bool;
    fn set_ribbon_mark_thorny(&mut self, flag: bool);
    fn get_ribbon_mark_vigor(&self) -> bool;
    fn set_ribbon_mark_vigor(&mut self, flag: bool);
    fn get_ribbon_mark_slump(&self) -> bool;
    fn set_ribbon_mark_slump(&mut self, flag: bool);

    fn has_mark(&self) -> bool;

    fn has_weather_mark(&self) -> bool {
        self.get_ribbon_mark_cloudy()
            || self.get_ribbon_mark_rainy()
            || self.get_ribbon_mark_stormy()
            || self.get_ribbon_mark_snowy()
            || self.get_ribbon_mark_blizzard()
            || self.get_ribbon_mark_dry()
            || self.get_ribbon_mark_sandstorm()
            || self.get_ribbon_mark_misty()
    }

    fn ribbon_bits(&self) -> [bool; 45] {
        [
            self.get_ribbon_mark_lunchtime(),
            self.get_ribbon_mark_sleepytime(),
            self.get_ribbon_mark_dusk(),
            self.get_ribbon_mark_dawn(),
            self.get_ribbon_mark_cloudy(),
            self.get_ribbon_mark_rainy(),
            self.get_ribbon_mark_stormy(),
            self.get_ribbon_mark_snowy(),
            self.get_ribbon_mark_blizzard(),
            self.get_ribbon_mark_dry(),
            self.get_ribbon_mark_sandstorm(),
            self.get_ribbon_mark_misty(),
            self.get_ribbon_mark_destiny(),
            self.get_ribbon_mark_fishing(),
            self.get_ribbon_mark_curry(),
            self.get_ribbon_mark_uncommon(),
            self.get_ribbon_mark_rare(),
            self.get_ribbon_mark_rowdy(),
            self.get_ribbon_mark_absent_minded(),
            self.get_ribbon_mark_jittery(),
            self.get_ribbon_mark_excited(),
            self.get_ribbon_mark_charismatic(),
            self.get_ribbon_mark_calmness(),
            self.get_ribbon_mark_intense(),
            self.get_ribbon_mark_zoned_out(),
            self.get_ribbon_mark_joyful(),
            self.get_ribbon_mark_angry(),
            self.get_ribbon_mark_smiley(),
            self.get_ribbon_mark_teary(),
            self.get_ribbon_mark_upbeat(),
            self.get_ribbon_mark_peeved(),
            self.get_ribbon_mark_intellectual(),
            self.get_ribbon_mark_ferocious(),
            self.get_ribbon_mark_crafty(),
            self.get_ribbon_mark_scowling(),
            self.get_ribbon_mark_kindly(),
            self.get_ribbon_mark_flustered(),
            self.get_ribbon_mark_pumped_up(),
            self.get_ribbon_mark_zero_energy(),
            self.get_ribbon_mark_prideful(),
            self.get_ribbon_mark_unsure(),
            self.get_ribbon_mark_humble(),
            self.get_ribbon_mark_thorny(),
            self.get_ribbon_mark_vigor(),
            self.get_ribbon_mark_slump(),
        ]
    }
}

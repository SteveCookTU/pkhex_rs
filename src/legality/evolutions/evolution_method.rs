use std::fmt::{Display, Formatter};
use crate::{EvolutionType, PersonalInfo, Pkm, PKMType};
use crate::game_strings::SPECIES_EN;

pub struct EvolutionMethod {
    method: usize,
    species: usize,
    argument: usize,
    level: u8,
    form: Option<usize>,
    requires_level_up: bool
}

impl EvolutionMethod {
    pub fn new(method: usize, species: usize, argument: usize, level: u8, form: Option<usize>) -> Self {
        Self {
            method,
            species,
            argument,
            level,
            form,
            requires_level_up: false
        }
    }

    pub fn get_destination_form(&self, form: usize) -> usize {
        if self.method == EvolutionType::LevelUpFormFemale1 as usize {
            1
        } else {
            self.form.unwrap_or(form)
        }
    }

    pub fn valid<I: PersonalInfo, T: Pkm<I>>(&mut self, pkm: &T, lvl: usize, skip_checks: bool) -> bool {
        self.requires_level_up = false;
        let et: EvolutionType = (self.method as u8).into();
        match et {
            EvolutionType::UseItem | EvolutionType::UseItemWormhole | EvolutionType::UseItemFullMoon | EvolutionType::CriticalHitsInBattle | EvolutionType::HitPointsLostInBattle | EvolutionType::Spin | EvolutionType::UseAgileStyleMoves | EvolutionType::UseStrongStyleMoves | EvolutionType::TowerOfWaters | EvolutionType::TowerOfDarkness => true,
            EvolutionType::UseItemMale | EvolutionType::RecoilDamageMale => pkm.get_gender() == 0,
            EvolutionType::UseItemFemale | EvolutionType::RecoilDamageFemale => pkm.get_gender() == 1,
            EvolutionType::Trade | EvolutionType::TradeHeldItem | EvolutionType::TradeShelmetKarrablast => !pkm.is_untraded() || skip_checks,
            EvolutionType::LevelUpNatureAmped | EvolutionType::LevelUpNatureLowKey if EvolutionMethod::get_amp_lowkey_result(pkm.get_nature()) != pkm.get_form() && !skip_checks => false,
            EvolutionType::LevelUpBeauty if {
                if let Some(c) = pkm.get_contest_stats() {
                    c.get_cnt_beauty() < self.argument as u8
                } else {
                    false
                }
            } => skip_checks,
            EvolutionType::LevelUpMale if pkm.get_gender() != 0 => false,
            EvolutionType::LevelUpFemale if pkm.get_gender() != 1 => false,
            EvolutionType::LevelUpFormFemale1 if pkm.get_gender() != 1 || pkm.get_form() != 1 => false,
            EvolutionType::LevelUpVersion | EvolutionType::LevelUpVersionDay | EvolutionType::LevelUpVersionNight if ((pkm.get_version() & 1) != (self.argument & 1) && pkm.is_untraded()) || skip_checks => skip_checks,
            _ => {
                if self.is_threshold_check(pkm) {
                    lvl >= self.level as usize
                } else if (self.level == 0 && lvl < 2) || lvl < self.level as usize {
                    false
                } else {
                    self.requires_level_up = true;
                    if skip_checks {
                        lvl >= self.level as usize
                    } else {
                        self.has_met_level_increased(pkm, lvl)
                    }
                }
            }
        }
    }

    fn is_threshold_check<I: PersonalInfo, T: Pkm<I>>(&self, pkm: &T) -> bool {
        pkm.get_type() == PKMType::PA8
    }

    fn has_met_level_increased<I: PersonalInfo, T: Pkm<I>>(&self, pkm: &T, lvl: usize) -> bool {
        let origin = pkm.generation();
        match origin {
            1 | 2 => true,
            3 | 4 => pkm.format() > origin || pkm.get_met_level() < lvl,
            _ if origin >= 5 => lvl >= self.level as usize && (!pkm.is_native() || pkm.get_met_level() < lvl),
            _ => false
        }
    }

    pub fn get_amp_lowkey_result(n: usize) -> usize {
        let index = n - 1;
        if index > 22 {
            0
        } else{
            (0b_0101_1011_1100_1010_0101_0001 >> index) & 1
        }
    }
}

impl Display for EvolutionMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{} [{}] @ {}{}", SPECIES_EN[self.species], self.form.unwrap_or(0), self.argument, self.level, if self.requires_level_up { "X" } else { "" })
    }
}

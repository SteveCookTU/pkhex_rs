use crate::legality::evolutions::{EvoCriteria, EvolutionType};
use crate::pkm::traits::metadata::SpeciesForm;
use crate::pkm::traits::templates::{ContestStatsReadOnly, NatureReadOnly};
use crate::pkm::{Pkm, PKM};

#[derive(Copy, Clone)]
pub struct EvolutionMethod {
    pub species: u16,
    pub argument: u16,
    pub method: EvolutionType,
    pub form: u8,
    pub level: u8,
    pub level_up: u8,
}

impl EvolutionMethod {
    const ANY_FORM: u8 = u8::MAX;

    pub fn requires_level_up(&self) -> bool {
        self.level_up != 0
    }

    pub fn get_destination_form(&self, form: u8) -> u8 {
        if self.method == EvolutionType::LevelUpFormFemale1 {
            1
        } else if self.form == EvolutionMethod::ANY_FORM {
            form
        } else {
            self.form
        }
    }

    pub fn valid(&self, pk: &PKM, lvl: u8, skip_checks: bool) -> bool {
        if !self.method.is_level_up_required() {
            return self.valid_not_level_up(pk.as_pkm(), skip_checks);
        }

        if !self.is_level_up_method_secondary_satisfied(pk, skip_checks) {
            return false;
        }

        if !self.requires_level_up() {
            return lvl >= self.level;
        }

        if (self.level == 0 && lvl < 2) || lvl < self.level {
            return false;
        }

        if skip_checks {
            return lvl >= self.level;
        }

        self.has_met_level_increased(pk.as_pkm(), lvl)
    }

    fn is_level_up_method_secondary_satisfied(&self, pk: &PKM, skip_checks: bool) -> bool {
        match self.method {
            EvolutionType::LevelUpMale if pk.as_pkm().gender() != 0 => false,
            EvolutionType::LevelUpFemale if pk.as_pkm().gender() != 1 => false,
            EvolutionType::LevelUpFormFemale1
                if pk.as_pkm().gender() != 1 || pk.as_pkm().form() != 0 =>
            {
                false
            }
            EvolutionType::LevelUpBeauty
                if pk.as_contest_stats_read_only().is_some()
                    && pk.as_contest_stats_read_only().unwrap().cnt_beauty()
                        < self.argument as u8 =>
            {
                skip_checks
            }
            EvolutionType::LevelUpNatureAmped | EvolutionType::LevelUpNatureLowKey
                if EvolutionMethod::get_amp_low_key_result(pk.as_pkm().nature())
                    != pk.as_pkm().form() =>
            {
                skip_checks
            }
            EvolutionType::LevelUpVersion
            | EvolutionType::LevelUpVersionDay
            | EvolutionType::LevelUpVersionNight
                if (pk.as_pkm().version() & 1) != (self.argument as u8 & 1)
                    && pk.as_pkm().is_untraded() =>
            {
                skip_checks
            }
            EvolutionType::LevelUpKnowMoveEC100 if pk.as_pkm().encryption_constant() % 100 != 0 => {
                skip_checks
            }
            EvolutionType::LevelUpKnowMoveECElse
                if pk.as_pkm().encryption_constant() % 100 == 0 =>
            {
                skip_checks
            }
            EvolutionType::LevelUpInBattleEC100 if pk.as_pkm().encryption_constant() % 100 != 0 => {
                skip_checks
            }
            EvolutionType::LevelUpInBattleECElse
                if pk.as_pkm().encryption_constant() % 100 == 0 =>
            {
                skip_checks
            }
            _ => true,
        }
    }

    fn valid_not_level_up(&self, pk: &impl Pkm, skip_checks: bool) -> bool {
        match self.method {
            EvolutionType::UseItemMale | EvolutionType::LevelUpRecoilDamageMale => pk.gender() == 0,
            EvolutionType::UseItemFemale | EvolutionType::LevelUpRecoilDamageFemale => {
                pk.gender() == 1
            }
            EvolutionType::Trade
            | EvolutionType::TradeHeldItem
            | EvolutionType::TradeShelmetKarrablast => !pk.is_untraded() || skip_checks,
            _ => true,
        }
    }

    fn has_met_level_increased(&self, pk: &impl Pkm, lvl: u8) -> bool {
        let origin = pk.generation();
        match origin {
            1 | 2 => true,
            3 | 4 => pk.format().unwrap_or_default() > origin || lvl > pk.met_level(),
            i if i >= 5 => lvl >= self.level && (!pk.is_native() || lvl > pk.met_level()),
            _ => false,
        }
    }

    pub fn get_evo_criteria(&self, species: u16, form: u8, lvl: u8) -> EvoCriteria {
        EvoCriteria {
            species,
            form,
            level_up_required: 0,
            level_max: lvl,
            level_min: 0,
            method: self.method,
        }
    }

    pub fn get_amp_low_key_result(n: u8) -> u8 {
        let index = n.wrapping_sub(1);
        if index > 22 {
            0
        } else {
            ((0b_0101_1011_1100_1010_0101_0001 >> index) & 1) as u8
        }
    }
}

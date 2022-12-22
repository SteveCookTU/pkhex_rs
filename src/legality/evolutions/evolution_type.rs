#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum EvolutionType {
    None = 0,
    LevelUpFriendship = 1,
    LevelUpFriendshipMorning = 2,
    LevelUpFriendshipNight = 3,
    LevelUp = 4,
    Trade = 5,
    TradeHeldItem = 6,
    TradeShelmetKarrablast = 7,
    UseItem = 8,
    LevelUpATK = 9,
    LevelUpAeqD = 10,
    LevelUpDEF = 11,
    LevelUpECl5 = 12,
    LevelUpECgeq5 = 13,
    LevelUpNinjask = 14,
    LevelUpShedinja = 15,
    LevelUpBeauty = 16,
    UseItemMale = 17,
    UseItemFemale = 18,
    LevelUpHeldItemDay = 19,
    LevelUpHeldItemNight = 20,
    LevelUpKnowMove = 21,
    LevelUpWithTeammate = 22,
    LevelUpMale = 23,
    LevelUpFemale = 24,
    LevelUpElectric = 25,
    LevelUpForest = 26,
    LevelUpCold = 27,
    LevelUpInverted = 28,
    LevelUpAffection50MoveType = 29,
    LevelUpMoveType = 30,
    LevelUpWeather = 31,
    LevelUpMorning = 32,
    LevelUpNight = 33,
    LevelUpFormFemale1 = 34,
    Unused = 35,
    LevelUpVersion = 36,
    LevelUpVersionDay = 37,
    LevelUpVersionNight = 38,
    LevelUpSummit = 39,
    LevelUpDusk = 40,
    LevelUpWormhole = 41,
    UseItemWormhole = 42,
    CriticalHitsInBattle = 43,  // Sirfetch'd
    HitPointsLostInBattle = 44, // Runerigus
    Spin = 45,                  // Alcremie
    LevelUpNatureAmped = 46,    // Toxtricity
    LevelUpNatureLowKey = 47,   // Toxtricity
    TowerOfDarkness = 48,       // Urshifu
    TowerOfWaters = 49,         // Urshifu

    LevelUpWalkStepsWith = 50,
    LevelUpUnionCircle = 51,    // Palafin
    LevelUpInBattleEC100 = 52,  // Maushold-0
    LevelUpInBattleECElse = 53, // Maushold-1
    LevelUpCollect999 = 54,     // Gimmighoul formarg 999
    LevelUpDefeatEquals = 55,   // Kingambit
    LevelUpUseMoveSpecial = 56, // Annihilape
    LevelUpKnowMoveECElse = 57, // Dudunsparce-0
    LevelUpKnowMoveEC100 = 58,  // Dudunsparce-1

    LevelUpRecoilDamageMale = 59,   // Basculegion-0
    LevelUpRecoilDamageFemale = 60, // Basculegion-1

    Hisui = 61,

    UseItemFullMoon = 90,    // Ursaluna
    UseMoveAgileStyle = 91,  // Wyrdeer
    UseMoveStrongStyle = 92, // Overqwil
}

impl EvolutionType {
    pub fn is_trade(&self) -> bool {
        self == &EvolutionType::Trade
            || self == &EvolutionType::TradeHeldItem
            || self == &EvolutionType::TradeShelmetKarrablast
    }

    pub fn is_level_up_required(&self) -> bool {
        match self {
            EvolutionType::None => false,
            EvolutionType::LevelUpFriendship => true,
            EvolutionType::LevelUpFriendshipMorning => true,
            EvolutionType::LevelUpFriendshipNight => true,
            EvolutionType::LevelUp => true,
            EvolutionType::Trade => false,
            EvolutionType::TradeHeldItem => false,
            EvolutionType::TradeShelmetKarrablast => false,
            EvolutionType::UseItem => false,
            EvolutionType::LevelUpATK => true,
            EvolutionType::LevelUpAeqD => true,
            EvolutionType::LevelUpDEF => true,
            EvolutionType::LevelUpECl5 => true,
            EvolutionType::LevelUpECgeq5 => true,
            EvolutionType::LevelUpNinjask => true,
            EvolutionType::LevelUpShedinja => true,
            EvolutionType::LevelUpBeauty => true,
            EvolutionType::UseItemMale => false,
            EvolutionType::UseItemFemale => false,
            EvolutionType::LevelUpHeldItemDay => true,
            EvolutionType::LevelUpHeldItemNight => true,
            EvolutionType::LevelUpKnowMove => true,
            EvolutionType::LevelUpWithTeammate => true,
            EvolutionType::LevelUpMale => true,
            EvolutionType::LevelUpFemale => true,
            EvolutionType::LevelUpElectric => true,
            EvolutionType::LevelUpForest => true,
            EvolutionType::LevelUpCold => true,
            EvolutionType::LevelUpInverted => true,
            EvolutionType::LevelUpAffection50MoveType => true,
            EvolutionType::LevelUpMoveType => true,
            EvolutionType::LevelUpWeather => true,
            EvolutionType::LevelUpMorning => true,
            EvolutionType::LevelUpNight => true,
            EvolutionType::LevelUpFormFemale1 => true,
            EvolutionType::Unused => false,
            EvolutionType::LevelUpVersion => true,
            EvolutionType::LevelUpVersionDay => true,
            EvolutionType::LevelUpVersionNight => true,
            EvolutionType::LevelUpSummit => true,
            EvolutionType::LevelUpDusk => true,
            EvolutionType::LevelUpWormhole => true,
            EvolutionType::UseItemWormhole => false,
            EvolutionType::CriticalHitsInBattle => false,
            EvolutionType::HitPointsLostInBattle => false,
            EvolutionType::Spin => false,
            EvolutionType::LevelUpNatureAmped => true,
            EvolutionType::LevelUpNatureLowKey => true,
            EvolutionType::TowerOfDarkness => false,
            EvolutionType::TowerOfWaters => false,
            EvolutionType::LevelUpWalkStepsWith => true,
            EvolutionType::LevelUpUnionCircle => true,
            EvolutionType::LevelUpInBattleEC100 => true,
            EvolutionType::LevelUpInBattleECElse => true,
            EvolutionType::LevelUpCollect999 => true,
            EvolutionType::LevelUpDefeatEquals => true,
            EvolutionType::LevelUpUseMoveSpecial => true,
            EvolutionType::LevelUpKnowMoveECElse => true,
            EvolutionType::LevelUpKnowMoveEC100 => true,
            EvolutionType::LevelUpRecoilDamageMale => true,
            EvolutionType::LevelUpRecoilDamageFemale => true,
            EvolutionType::Hisui => false, // stubbed

            EvolutionType::UseItemFullMoon => false,
            EvolutionType::UseMoveAgileStyle => false,
            EvolutionType::UseMoveStrongStyle => false,
        }
    }
}

#[derive(PartialEq)]
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
    CriticalHitsInBattle = 43, // Sirfetch'd
    HitPointsLostInBattle = 44, // Runerigus
    Spin = 45, // Alcremie
    LevelUpNatureAmped = 46, // Toxtricity
    LevelUpNatureLowKey = 47, // Toxtricity
    TowerOfDarkness = 48, // Urshifu
    TowerOfWaters = 49, // Urshifu
    UseItemFullMoon = 50, // Ursaluna
    UseAgileStyleMoves = 51, // Wyrdeer
    UseStrongStyleMoves = 52, // Overqwil
    RecoilDamageMale = 53, // Basculegion-0
    RecoilDamageFemale = 54, // Basculegion-1
}

impl From<u8> for EvolutionType {
    fn from(val: u8) -> Self {
        match val {
            0 => EvolutionType::None,
            1 => EvolutionType::LevelUpFriendship,
            2 => EvolutionType::LevelUpFriendshipMorning,
            3 => EvolutionType::LevelUpFriendshipNight,
            4 => EvolutionType::LevelUp,
            5 => EvolutionType::Trade,
            6 => EvolutionType::TradeHeldItem,
            7 => EvolutionType::TradeShelmetKarrablast,
            8 => EvolutionType::UseItem,
            9 => EvolutionType::LevelUpATK,
            10 => EvolutionType::LevelUpAeqD,
            11 => EvolutionType::LevelUpDEF,
            12 => EvolutionType::LevelUpECl5,
            13 => EvolutionType::LevelUpECgeq5,
            14 => EvolutionType::LevelUpNinjask,
            15 => EvolutionType::LevelUpShedinja,
            16 => EvolutionType::LevelUpBeauty,
            17 => EvolutionType::UseItemMale,
            18 => EvolutionType::UseItemFemale,
            19 => EvolutionType::LevelUpHeldItemDay,
            20 => EvolutionType::LevelUpHeldItemNight,
            21 => EvolutionType::LevelUpKnowMove,
            22 => EvolutionType::LevelUpWithTeammate,
            23 => EvolutionType::LevelUpMale,
            24 => EvolutionType::LevelUpFemale,
            25 => EvolutionType::LevelUpElectric,
            26 => EvolutionType::LevelUpForest,
            27 => EvolutionType::LevelUpCold,
            28 => EvolutionType::LevelUpInverted,
            29 => EvolutionType::LevelUpAffection50MoveType,
            30 => EvolutionType::LevelUpMoveType,
            31 => EvolutionType::LevelUpWeather,
            32 => EvolutionType::LevelUpMorning,
            33 => EvolutionType::LevelUpNight,
            34 => EvolutionType::LevelUpFormFemale1,
            35 => EvolutionType::Unused,
            36 => EvolutionType::LevelUpVersion,
            37 => EvolutionType::LevelUpVersionDay,
            38 => EvolutionType::LevelUpVersionNight,
            39 => EvolutionType::LevelUpSummit,
            40 => EvolutionType::LevelUpDusk,
            41 => EvolutionType::LevelUpWormhole,
            42 => EvolutionType::UseItemWormhole,
            43 => EvolutionType::CriticalHitsInBattle,
            44 => EvolutionType::HitPointsLostInBattle,
            45 => EvolutionType::Spin,
            46 => EvolutionType::LevelUpNatureAmped,
            47 => EvolutionType::LevelUpNatureLowKey,
            48 => EvolutionType::TowerOfDarkness,
            49 => EvolutionType::None,
            50 => EvolutionType::TowerOfWaters,
            51 => EvolutionType::UseItemFullMoon,
            52 => EvolutionType::UseAgileStyleMoves,
            53 => EvolutionType::UseStrongStyleMoves,
            54 => EvolutionType::RecoilDamageMale,
            _ => EvolutionType::RecoilDamageFemale
        }
    }
}

impl EvolutionType {
    pub fn is_traded(&self) -> bool {
        matches!(self, EvolutionType::TradeHeldItem | EvolutionType::TradeShelmetKarrablast)
    }
}
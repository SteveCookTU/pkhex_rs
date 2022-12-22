use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum CheckIdentifier {
    CurrentMove,

    RelearnMove,

    Encounter,

    Shiny,

    EC,

    PID,

    Gender,

    EVs,

    Language,

    Nickname,

    Trainer,

    IVs,

    Level,

    Ball,

    Memory,

    Geography,

    Form,

    Egg,

    Misc,

    Fateful,

    Ribbon,

    Training,

    Ability,

    Evolution,

    Nature,

    GameOrigin,

    HeldItem,

    RibbonMark,

    GVs,

    Marking,

    AVs,
}

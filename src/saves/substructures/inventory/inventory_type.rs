#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum InventoryType {
    None,
    Items,
    KeyItems,
    TMHMs,
    Medicine,
    Berries,
    Balls,
    BattleItems,
    MailItems,
    PCItems,
    FreeSpace,
    ZCrystals,
    Candy,
    Treasure,
    Ingredients,
}

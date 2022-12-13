#![allow(dead_code)]

use crate::game::enums::{Ball, GameVersion, Move, Species};
use crate::legality::tables8::MAX_ABILITY_ID_8_R2;

pub(crate) const MAX_SPECIES_ID_8A: u16 = Species::Enamorus as u16;
pub(crate) const MAX_MOVE_ID_8A: u16 = Move::TakeHeart as u16;
pub(crate) const MAX_ITEM_ID_8A: u16 = 1828;
pub(crate) const MAX_BALL_ID_8A: u16 = Ball::LAOrigin as u16;
pub(crate) const MAX_GAME_ID_8A: u16 = GameVersion::SP as u16;
pub(crate) const MAX_ABILITY_ID_8A: u16 = MAX_ABILITY_ID_8_R2;

pub(crate) const HELD_ITEMS_LA: [u16; 0] = [];

#[allow(clippy::zero_prefixed_literal)]
pub(crate) const POUCH_HELD_ITEMS_LA: [u16; 166] = [
    017, 023, 024, 025, 026, 027, 028, 029, 039, 041, 050, 054, 072, 073, 075, 080, 081, 082, 083,
    084, 085, 090, 091, 092, 107, 108, 109, 110, 149, 150, 151, 152, 153, 154, 155, 157, 158, 159,
    160, 161, 162, 163, 164, 166, 168, 233, 252, 321, 322, 323, 324, 325, 326, 327, 583, 849, 1125,
    1126, 1127, 1128, 1231, 1232, 1233, 1234, 1235, 1236, 1237, 1238, 1239, 1240, 1241, 1242, 1243,
    1244, 1245, 1246, 1247, 1248, 1249, 1250, 1251, 1611, 1613, 1614, 1615, 1616, 1617, 1618, 1619,
    1620, 1621, 1628, 1630, 1631, 1632, 1633, 1634, 1635, 1636, 1637, 1638, 1651, 1679, 1681, 1682,
    1684, 1686, 1687, 1688, 1689, 1690, 1691, 1692, 1693, 1694, 1695, 1696, 1699, 1700, 1701, 1702,
    1703, 1704, 1705, 1706, 1707, 1708, 1709, 1710, 1711, 1712, 1713, 1716, 1717, 1720, 1724, 1725,
    1726, 1727, 1728, 1732, 1733, 1734, 1735, 1736, 1738, 1739, 1740, 1741, 1742, 1746, 1747, 1748,
    1749, 1750, 1754, 1755, 1756, 1757, 1758, 1759, 1760, 1761, 1762, 1764, 1785,
];

pub(crate) const POUCH_RECIPE_LA: [u16; 43] = [
    1640, 1641, 1642, 1643, 1644, 1646, 1647, 1648, 1649, 1650, 1652, 1653, 1654, 1655, 1656, 1657,
    1658, 1659, 1660, 1661, 1662, 1663, 1664, 1665, 1666, 1667, 1668, 1669, 1670, 1671, 1673, 1674,
    1675, 1676, 1677, 1729, 1730, 1731, 1751, 1752, 1753, 1783, 1784,
];

pub(crate) const POUCH_KEY_LA: [u16; 77] = [
    111, 298, 299, 300, 301, 302, 303, 304, 305, 306, 307, 308, 309, 310, 311, 312, 313, 441, 455,
    466, 632, 638, 644, 1608, 1609, 1610, 1612, 1622, 1624, 1625, 1626, 1627, 1629, 1639, 1678,
    1721, 1722, 1723, 1737, 1743, 1744, 1745, 1763, 1765, 1766, 1767, 1768, 1769, 1771, 1776, 1777,
    1778, 1779, 1780, 1782, 1786, 1787, 1788, 1789, 1790, 1792, 1793, 1794, 1795, 1796, 1797, 1798,
    1799, 1800, 1801, 1802, 1803, 1804, 1805, 1806, 1807, 1828,
];

pub(crate) const MOVE_SHOP_LA: [u16; 61] = [
    Move::FalseSwipe as u16,
    Move::FireFang as u16,
    Move::ThunderFang as u16,
    Move::IceFang as u16,
    Move::IceBall as u16,
    Move::RockSmash as u16,
    Move::Spikes as u16,
    Move::Bulldoze as u16,
    Move::AerialAce as u16,
    Move::StealthRock as u16,
    Move::Swift as u16,
    Move::TriAttack as u16,
    Move::MagicalLeaf as u16,
    Move::OminousWind as u16,
    Move::PowerShift as u16,
    Move::FocusEnergy as u16,
    Move::BulkUp as u16,
    Move::CalmMind as u16,
    Move::Rest as u16,
    Move::BabyDollEyes as u16,
    Move::FirePunch as u16,
    Move::ThunderPunch as u16,
    Move::IcePunch as u16,
    Move::DrainPunch as u16,
    Move::PoisonJab as u16,
    Move::PsychoCut as u16,
    Move::ZenHeadbutt as u16,
    Move::LeechLife as u16,
    Move::XScissor as u16,
    Move::RockSlide as u16,
    Move::ShadowClaw as u16,
    Move::IronHead as u16,
    Move::IronTail as u16,
    Move::MysticalFire as u16,
    Move::WaterPulse as u16,
    Move::ChargeBeam as u16,
    Move::EnergyBall as u16,
    Move::IcyWind as u16,
    Move::SludgeBomb as u16,
    Move::EarthPower as u16,
    Move::ShadowBall as u16,
    Move::Snarl as u16,
    Move::FlashCannon as u16,
    Move::DazzlingGleam as u16,
    Move::GigaImpact as u16,
    Move::AquaTail as u16,
    Move::WildCharge as u16,
    Move::HighHorsepower as u16,
    Move::Megahorn as u16,
    Move::StoneEdge as u16,
    Move::Outrage as u16,
    Move::PlayRough as u16,
    Move::HyperBeam as u16,
    Move::Flamethrower as u16,
    Move::Thunderbolt as u16,
    Move::IceBeam as u16,
    Move::Psychic as u16,
    Move::DarkPulse as u16,
    Move::DracoMeteor as u16,
    Move::SteelBeam as u16,
    Move::VoltTackle as u16,
];

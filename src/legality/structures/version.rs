use crate::game::enums::GameVersion;
use crate::game::game_util;
use rand::{thread_rng, Rng};

fn can_be_received_by(ver: &dyn Version, game: GameVersion) -> bool {
    ver.version().contains(&game)
}

fn get_single_version(ver: &dyn Version) -> GameVersion {
    let max = game_util::HIGHEST_GAME_ID;
    if ver.version() <= max {
        return ver.version();
    }
    let mut rnd = thread_rng();
    loop {
        let game = GameVersion::from(rnd.gen_range(1u8..(max as u8)));
        if game == GameVersion::BU {
            continue;
        }
        if can_be_received_by(ver, game) {
            return game;
        }
    }
}

pub trait Version {
    fn version(&self) -> GameVersion;

    fn get_compatible_version(&self, prefer: GameVersion) -> GameVersion
    where
        Self: Sized,
    {
        if can_be_received_by(self, prefer) || self.version() == GameVersion::Any {
            prefer
        } else {
            get_single_version(self)
        }
    }
}

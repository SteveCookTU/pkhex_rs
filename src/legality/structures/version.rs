use crate::{game_util, GameVersion};
use rand::{thread_rng, Rng};

pub trait Version {
    fn version(&self) -> GameVersion;

    fn get_compatible_version(&self, prefer: GameVersion) -> GameVersion {
        if can_be_received_by(self, prefer) || self.version() == GameVersion::Any {
            return prefer;
        }

        get_single_version(self)
    }
}

fn can_be_received_by<T: Version + ?Sized>(ver: &T, prefer: GameVersion) -> bool {
    ver.version().contains(&prefer)
}

fn get_single_version<T: Version + ?Sized>(ver: &T) -> GameVersion {
    const MAX: u8 = game_util::HIGHEST_GAME_ID;
    if ver.version() as u8 <= MAX {
        return ver.version();
    }
    let mut rnd = thread_rng();
    loop {
        let game = GameVersion::from(rnd.gen_range(1..MAX) as usize);
        if game == GameVersion::BU {
            continue;
        }
        if can_be_received_by(ver, game) {
            return game;
        }
    }
}

use crate::traits::trainer_id::TrainerId;
use crate::traits::RegionOrigin;
use crate::{EntityContext, GameVersion, PersonalInfo, PKM};

pub trait TrainerInfo: TrainerId {
    fn ot(&self) -> String;
    fn gender(&self) -> u8;
    fn game(&self) -> u8;
    fn language(&self) -> u8;

    fn generation(&self) -> u8;

    fn context(&self) -> EntityContext;

    fn apply_to<Personal: PersonalInfo + 'static, T: PKM<Personal>>(&self, pk: &mut T) {
        pk.set_ot_name(&self.ot());
        pk.set_tid(self.get_tid());
        pk.set_sid(if pk.format() < 3 || pk.vc() {
            0
        } else {
            self.get_sid()
        });
        pk.set_ot_gender(self.gender());
        pk.set_language(self.language());
        pk.set_version(self.game());
    }

    fn apply_to_region_origin<Personal: PersonalInfo + 'static, T: PKM<Personal>>(&self, pk: &mut T)
    where
        T: RegionOrigin,
        Self: RegionOrigin,
    {
        self.apply_to(pk);
        pk.set_country(self.get_country());
        pk.set_region(self.get_region());
        pk.set_console_region(self.get_console_region());
    }

    fn apply_handling_trainer_info<Personal: PersonalInfo + 'static, T: PKM<Personal>>(
        &self,
        pk: &mut T,
        force: bool,
    ) {
        if pk.format() == self.generation() && !force {
            return;
        }

        pk.set_ht_name(&self.ot());
        pk.set_ht_gender(self.gender());
        pk.set_ht_friendship(pk.get_ot_friendship());
        pk.set_current_handler(1);
    }

    fn apply_handling_trainer_info_pk6(&self) {
        todo!()
    }

    fn apply_handling_trainer_info_pk8(&self) {
        todo!()
    }

    fn is_from_trainer<Personal: PersonalInfo + 'static, T: PKM<Personal>>(&self, pk: &T) -> bool {
        if self.game() == GameVersion::Any as u8 {
            return true;
        }

        if self.get_tid() != pk.get_tid() {
            return false;
        }

        !(self.get_tid() != pk.get_tid()
            || self.ot() != pk.get_ot_name()
            || pk.format() <= 3
            || self.get_sid() != pk.get_sid()
            || self.gender() != pk.get_ot_gender())
            || is_match_version(self, pk)
    }
}

fn is_match_version<Personal: PersonalInfo + 'static, T: PKM<Personal>, K: TrainerInfo + ?Sized>(
    tr: &K,
    pk: &T,
) -> bool {
    if tr.game() == pk.get_version() {
        return true;
    }
    if pk.go_lgpe() {
        return tr.game() == GameVersion::GP as u8 || tr.game() == GameVersion::GE as u8;
    }
    false
}

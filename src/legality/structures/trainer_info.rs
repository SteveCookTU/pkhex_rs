use crate::{GameVersion, PersonalInfo, Pkm, TrainerId};

pub trait TrainerInfo: TrainerId {
    fn get_ot(&self) -> String;
    fn get_gender(&self) -> usize;
    fn get_game(&self) -> usize;
    fn get_language(&self) -> usize;
    fn get_generation(&self) -> usize;

    fn apply_to<I: PersonalInfo, P: Pkm<I> + ?Sized>(&self, pk: &mut P) {
        pk.set_ot_name(self.get_ot());
        pk.set_tid(self.get_tid());
        pk.set_sid(if pk.format() < 3 || pk.vc() {
            0
        } else {
            self.get_sid()
        });
        pk.set_ot_gender(self.get_gender());
        pk.set_language(self.get_language());
        pk.set_version(self.get_generation());

        //TODO: Apply country, region, console region
    }

    fn apply_handling_trainer_info<I: PersonalInfo, P: Pkm<I> + ?Sized>(
        &self,
        pk: &mut P,
        force: bool,
    ) {
        if pk.format() == self.get_generation() && !force {
            return;
        }

        pk.set_ht_name(self.get_ot());
        pk.set_ht_gender(self.get_gender());
        pk.set_ht_friendship(pk.get_ot_friendship());
        pk.set_current_handler(1);

        // TODO: pk6 and pk8 trading
    }

    fn is_from_trainer<I: PersonalInfo, P: Pkm<I> + ?Sized>(&self, pk: &P) -> bool {
        if self.get_game() == GameVersion::Any as usize {
            true
        } else if self.get_tid() != pk.get_tid() {
            false
        } else if self.get_ot() != pk.get_ot_name() {
            false
        } else if pk.format() <= 2 {
            false
        } else if self.get_sid() != pk.get_sid() {
            false
        } else if pk.format() == 3 {
            false
        } else if self.get_gender() != pk.get_ot_gender() {
            false
        } else if self.get_game() != pk.get_version() {
            false
        } else {
            true
        }
    }
}

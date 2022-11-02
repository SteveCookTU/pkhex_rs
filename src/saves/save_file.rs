use crate::structures::{TrainerInfo, Version};
use crate::substructures::misc::{BoxDetailName, BoxDetailWallpaper};
use crate::traits::GameValueLimit;
use crate::{pkm, PersonalInfo, StringConverterOption};

pub trait SaveFile<Personal: PersonalInfo + 'static>:
    TrainerInfo
    + GameValueLimit
    + BoxDetailWallpaper
    + BoxDetailName
    + Version
    + Clone
    + Copy
    + Into<Vec<u8>>
{
    fn short_summary(&self) -> String;
    fn extension(&self) -> String;

    fn play_time_string(&self) -> String {
        format!(
            "{}ː{}ː{}",
            self.get_played_hours(),
            self.get_played_minutes(),
            self.get_played_seconds()
        )
    }

    fn pkm_extensions(&self) -> Vec<String> {
        pkm::extensions()
            .into_iter()
            .filter(|f| {
                let gen = f.as_bytes().last().unwrap() - 0x30;
                (3..=self.generation()).contains(&gen)
            })
            .collect()
    }

    fn get_final_data(&mut self) -> Vec<u8> {
        todo!()
    }

    fn misc_save_info(&self) -> String {
        String::new()
    }
    fn checksums_valid(&self) -> bool;
    fn checksum_info(&self) -> String;

    fn get_string(&self, data: &[u8]) -> String;
    fn set_string(
        &mut self,
        dest: &mut [u8],
        value: &str,
        max_length: usize,
        option: StringConverterOption,
    );

    fn personal(&self) -> &Personal;

    fn get_ot(&self) -> String {
        "PKHeX".to_string()
    }

    fn set_ot(&mut self, ot: &str);

    fn get_played_hours(&self) -> u16;
    fn set_played_hours(&mut self, hours: u16);
    fn get_played_minutes(&self) -> u8;
    fn set_played_minutes(&mut self, minutes: u8);
    fn get_played_seconds(&self) -> u8;
    fn set_played_seconds(&mut self, seconds: u8);
    fn get_seconds_to_start(&self) -> usize;
    fn set_seconds_to_start(&mut self, seconds: usize);
    fn get_seconds_to_fame(&self) -> usize;
    fn set_seconds_to_fame(&mut self, seconds: usize);
    fn get_money(&self) -> usize;
    fn set_money(&mut self, money: usize);
    fn box_count(&self) -> usize;
    fn slot_count(&self) -> usize {
        self.box_count() * 30
    }
    fn get_trainer_id_7(&self) -> u32 {
        (self.get_tid() as u32 | ((self.get_sid() as u32) << 16)) % 1000000
    }
    fn set_trainer_id_7(&mut self, id: u32) {
        self.set_id_7(self.get_trainer_sid_7(), id);
    }
    fn get_trainer_sid_7(&self) -> u32 {
        (self.get_tid() as u32 | ((self.get_sid() as u32) << 16)) / 1000000
    }
    fn set_trainer_sid_7(&mut self, id: u32) {
        self.set_id_7(id, self.get_trainer_id_7());
    }
    fn get_display_tid(&self) -> u32 {
        if self.generation() >= 7 {
            self.get_trainer_id_7()
        } else {
            self.get_tid() as u32
        }
    }
    fn set_display_tid(&mut self, id: u32) {
        if self.generation() >= 7 {
            self.set_trainer_id_7(id);
        } else {
            self.set_tid(id as u16);
        }
    }
    fn get_display_sid(&self) -> u32 {
        if self.generation() >= 7 {
            self.get_trainer_sid_7()
        } else {
            self.get_sid() as u32
        }
    }
    fn set_display_sid(&mut self, id: u32) {
        if self.generation() >= 7 {
            self.set_trainer_sid_7(id);
        } else {
            self.set_sid(id as u16);
        }
    }
    fn set_id_7(&mut self, sid: u32, tid: u32) {
        let oid = sid.wrapping_mul(1000000).wrapping_add(tid % 1000000);
        self.set_tid(oid as u16);
        self.set_sid((oid >> 16) as u16);
    }

    fn party_count(&self) -> u8;
}

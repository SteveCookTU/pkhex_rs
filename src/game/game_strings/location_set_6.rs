use crate::game::game_strings::LocationSet;

pub struct LocationSet6 {
    met_0: Vec<String>,
    met_3: Vec<String>,
    met_4: Vec<String>,
    met_6: Vec<String>,
}

impl LocationSet6 {
    fn get(names: &[String], index: usize) -> &str {
        if index >= names.len() {
            ""
        } else {
            &names[index]
        }
    }
}

impl LocationSet for LocationSet6 {
    fn get_location_names(&self, bank_id: u16) -> &[String] {
        match bank_id {
            0 => &self.met_0,
            3 => &self.met_3,
            4 => &self.met_4,
            6 => &self.met_6,
            _ => &[],
        }
    }

    fn get_location_name(&self, location_id: u16) -> &str {
        match location_id {
            i if i >= 60000 => LocationSet6::get(&self.met_6, location_id as usize - 60000),
            i if i >= 40000 => LocationSet6::get(&self.met_4, location_id as usize - 40000),
            i if i >= 30000 => LocationSet6::get(&self.met_3, location_id as usize - 30000),
            _ => LocationSet6::get(&self.met_0, location_id as usize),
        }
    }
}

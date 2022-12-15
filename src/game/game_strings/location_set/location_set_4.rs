use crate::game::game_strings::location_set::LocationSet;

#[derive(Clone)]
pub struct LocationSet4 {
    pub(crate) met_0: Vec<String>,
    pub(crate) met_2: Vec<String>,
    pub(crate) met_3: Vec<String>,
}

impl LocationSet4 {
    pub fn new(met_0: Vec<String>, met_2: Vec<String>, met_3: Vec<String>) -> Self {
        Self {
            met_0,
            met_2,
            met_3,
        }
    }

    fn get(names: &[String], index: usize) -> &str {
        if index >= names.len() {
            ""
        } else {
            &names[index]
        }
    }
}

impl LocationSet for LocationSet4 {
    fn get_location_names(&self, bank_id: u16) -> &[String] {
        match bank_id {
            0 => &self.met_0,
            2 => &self.met_2,
            3 => &self.met_3,
            _ => &[],
        }
    }

    fn get_location_name(&self, location_id: u16) -> &str {
        match location_id {
            i if i >= 3000 => LocationSet4::get(&self.met_3, location_id as usize - 3000),
            i if i >= 2000 => LocationSet4::get(&self.met_2, location_id as usize - 2000),
            _ => LocationSet4::get(&self.met_0, location_id as usize),
        }
    }
}

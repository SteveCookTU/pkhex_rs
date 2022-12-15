use crate::game::game_strings::location_set::LocationSet;

pub struct LocationSet0 {
    pub(crate) met_0: Vec<String>,
}

impl LocationSet0 {
    pub fn new(met_0: Vec<String>) -> Self {
        Self { met_0 }
    }
}

impl LocationSet for LocationSet0 {
    fn get_location_names(&self, bank_id: u16) -> &[String] {
        match bank_id {
            0 => &self.met_0,
            _ => &[],
        }
    }

    fn get_location_name(&self, location_id: u16) -> &str {
        if location_id as usize >= self.met_0.len() {
            ""
        } else {
            &self.met_0[location_id as usize]
        }
    }
}

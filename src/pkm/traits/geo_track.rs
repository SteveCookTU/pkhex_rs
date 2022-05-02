use crate::RegionOrigin;

#[derive(PartialEq)]
#[repr(u8)]
enum GeoValid {
    Valid,
    CountryAfterPreviousEmpty,
    RegionWithoutCountry,
}

pub trait GeoTrack: RegionOrigin {
    fn get_geo_1_region(&self) -> u8;
    fn set_geo_1_region(&mut self, region: u8);
    fn get_geo_2_region(&self) -> u8;
    fn set_geo_2_region(&mut self, region: u8);
    fn get_geo_3_region(&self) -> u8;
    fn set_geo_3_region(&mut self, region: u8);
    fn get_geo_4_region(&self) -> u8;
    fn set_geo_4_region(&mut self, region: u8);
    fn get_geo_5_region(&self) -> u8;
    fn set_geo_5_region(&mut self, region: u8);

    fn get_geo_1_country(&self) -> u8;
    fn set_geo_1_country(&mut self, country: u8);
    fn get_geo_2_country(&self) -> u8;
    fn set_geo_2_country(&mut self, country: u8);
    fn get_geo_3_country(&self) -> u8;
    fn set_geo_3_country(&mut self, country: u8);
    fn get_geo_4_country(&self) -> u8;
    fn set_geo_4_country(&mut self, country: u8);
    fn get_geo_5_country(&self) -> u8;
    fn set_geo_5_country(&mut self, country: u8);

    fn clear_geo_location_data(&mut self) {
        self.set_geo_1_country(0);
        self.set_geo_2_country(0);
        self.set_geo_3_country(0);
        self.set_geo_4_country(0);
        self.set_geo_5_country(0);

        self.set_geo_1_region(0);
        self.set_geo_2_region(0);
        self.set_geo_3_region(0);
        self.set_geo_4_region(0);
        self.set_geo_5_region(0);
    }

    fn trade_geo_location(&mut self, geo_country: u8, geo_region: u8) {
        self.set_geo_5_country(self.get_geo_4_country());
        self.set_geo_5_region(self.get_geo_4_region());

        self.set_geo_4_country(self.get_geo_3_country());
        self.set_geo_4_region(self.get_geo_3_region());

        self.set_geo_3_country(self.get_geo_2_country());
        self.set_geo_3_region(self.get_geo_2_region());

        self.set_geo_2_country(self.get_geo_2_country());
        self.set_geo_2_region(self.get_geo_2_region());

        self.set_geo_1_country(geo_country);
        self.set_geo_1_region(geo_region);
    }

    fn sanitize_geo_location_data(&mut self) {
        if self.get_geo_1_country() == 0 {
            self.set_geo_1_region(0);
        }
        if self.get_geo_2_country() == 0 {
            self.set_geo_2_region(0);
        }
        if self.get_geo_3_country() == 0 {
            self.set_geo_3_region(0);
        }
        if self.get_geo_4_country() == 0 {
            self.set_geo_4_region(0);
        }
        if self.get_geo_5_country() == 0 {
            self.set_geo_5_region(0);
        }

        loop {
            if self.get_geo_5_country() != 0 && self.get_geo_4_country() == 0 {
                self.set_geo_4_country(self.get_geo_5_country());
                self.set_geo_4_region(self.get_geo_5_region());
                self.set_geo_5_country(0);
                self.set_geo_5_region(0);
            }
            if self.get_geo_4_country() != 0 && self.get_geo_3_country() == 0 {
                self.set_geo_3_country(self.get_geo_4_country());
                self.set_geo_3_region(self.get_geo_4_region());
                self.set_geo_4_country(0);
                self.set_geo_4_region(0);
                continue;
            }
            if self.get_geo_3_country() != 0 && self.get_geo_2_country() == 0 {
                self.set_geo_2_country(self.get_geo_3_country());
                self.set_geo_2_region(self.get_geo_3_region());
                self.set_geo_3_country(0);
                self.set_geo_3_region(0);
                continue;
            }
            if self.get_geo_2_country() != 0 && self.get_geo_1_country() == 0 {
                self.set_geo_1_country(self.get_geo_2_country());
                self.set_geo_1_region(self.get_geo_2_region());
                self.set_geo_2_country(0);
                self.set_geo_2_region(0);
                continue;
            }
            break;
        }
    }

    fn is_valid(&self) -> bool {
        get_validity(self) == GeoValid::Valid
    }
}

fn get_validity<T: GeoTrack + ?Sized>(g: &T) -> GeoValid {
    let mut end = false;
    let mut update = |c: u8, r: u8| -> GeoValid {
        if end && c != 0 {
            return GeoValid::CountryAfterPreviousEmpty;
        }
        if c != 0 {
            return GeoValid::Valid;
        }
        if r != 0 {
            return GeoValid::RegionWithoutCountry;
        }
        end = true;
        GeoValid::Valid
    };
    let mut result = update(g.get_geo_1_country(), g.get_geo_1_region());
    if result != GeoValid::Valid {
        return result;
    }
    result = update(g.get_geo_2_country(), g.get_geo_2_region());
    if result != GeoValid::Valid {
        return result;
    }
    result = update(g.get_geo_3_country(), g.get_geo_3_region());
    if result != GeoValid::Valid {
        return result;
    }
    result = update(g.get_geo_4_country(), g.get_geo_4_region());
    if result != GeoValid::Valid {
        return result;
    }
    result = update(g.get_geo_5_country(), g.get_geo_5_region());

    result
}

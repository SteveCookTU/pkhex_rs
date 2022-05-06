use crate::personal_info_g1::PersonalInfoG1;
use crate::poke_crypto::{SIZE_1PARTY, SIZE_1STORED};
use crate::string_converter_12::G1_TERMINATOR_CODE;
use crate::{species_converter, GBPkml, Pkm, PokeList, PokeListType, SpeciesForm, PK1};
use std::ops::{Index, IndexMut};

const SLOT_NONE: u8 = u8::MAX;

pub struct PokeList1 {
    string_length: usize,
    data: Vec<u8>,
    capacity: u8,
    entry_size: usize,
    japanese: bool,
    pokemon: Vec<PK1>,
}

impl PokeList1 {
    fn get_empty_list(&self, c: PokeListType, jp: bool) -> Vec<u8> {
        let capacity = c as u8;

        let size_intro = (capacity + 1) as usize;
        let size_pkm = self.get_entry_size() * capacity as usize;
        let size_str = 2 * self.get_string_length(jp) * capacity as usize;

        let mut result = vec![0u8; 1 + size_intro + size_pkm + size_str];

        for val in result.iter_mut().skip(1).take(size_intro) {
            *val = SLOT_NONE;
        }

        let len = result.len() - 1;

        for val in result.iter_mut().skip(1 + size_intro + size_pkm).take(len) {
            *val = G1_TERMINATOR_CODE;
        }

        result
    }

    fn get_offset_pkm_data(&self, base_ofs: usize, index: usize) -> usize {
        base_ofs + (self.entry_size * index)
    }

    fn get_offset_pkm_ot(&self, base_ofs: usize, index: usize) -> usize {
        self.get_offset_pkm_data(base_ofs, self.capacity as usize) + (self.string_length * index)
    }

    fn get_offset_pkm_nickname(&self, base_ofs: usize, index: usize) -> usize {
        self.get_offset_pkm_ot(base_ofs, self.capacity as usize) + (self.string_length * index)
    }

    fn get_string_length(&self, jp: bool) -> usize {
        if jp {
            PK1::STRING_LENGTH_JAPANESE
        } else {
            PK1::STRING_LENGTH_NOT_JAPANESE
        }
    }

    fn read(&self) -> Vec<PK1> {
        let mut arr = Vec::with_capacity(self.capacity as usize);
        let base_ofs = 2 + self.capacity as usize;
        for i in 0..self.capacity {
            arr.push(self.get_entry(base_ofs, i as usize));
        }
        arr
    }

    fn get_entry(&self, base_ofs: usize, index: usize) -> PK1 {
        let pk_ofs = self.get_offset_pkm_data(base_ofs, index);
        let ot_ofs = self.get_offset_pkm_ot(base_ofs, index);
        let nk_ofs = self.get_offset_pkm_nickname(base_ofs, index);

        let dat = &self.data[pk_ofs..(pk_ofs + self.entry_size)];
        let ot_name = &self.data[ot_ofs..(ot_ofs + self.string_length)];
        let nick = &self.data[nk_ofs..(nk_ofs + self.string_length)];

        self.get_entry_from_info(dat.to_vec(), ot_name, nick, self.data[1 + index] == 0xFD)
    }

    fn set_entry(&mut self, base_ofs: usize, index: usize) {
        let pk_ofs = self.get_offset_pkm_data(base_ofs, index);
        let ot_ofs = self.get_offset_pkm_ot(base_ofs, index);
        let nk_ofs = self.get_offset_pkm_nickname(base_ofs, index);

        let pk = self.pokemon()[index].clone();
        let entry_size = self.entry_size;
        self.data.splice(
            pk_ofs..(pk_ofs + entry_size),
            pk.get_data()[0..entry_size].to_vec(),
        );
        self.data
            .splice(ot_ofs..(ot_ofs + pk.ot_trash().len()), pk.ot_trash());
        self.data.splice(
            nk_ofs..(nk_ofs + pk.nickname_trash().len()),
            pk.nickname_trash(),
        );
    }

    fn get_entry_size_party(&self, party: bool) -> usize {
        if party {
            SIZE_1PARTY
        } else {
            SIZE_1STORED
        }
    }

    pub fn get_data_length(&self, c: PokeListType, jp: bool) -> usize {
        self.get_data_size(c, jp, self.get_entry_size_party(self.is_capacity_format(c)))
    }
}

impl Index<usize> for PokeList1 {
    type Output = PK1;

    fn index(&self, index: usize) -> &Self::Output {
        &self.pokemon[index]
    }
}

impl IndexMut<usize> for PokeList1 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.pokemon[index]
    }
}

impl PokeList<PersonalInfoG1, PK1> for PokeList1 {
    fn pokemon(&self) -> &Vec<PK1> {
        &self.pokemon
    }

    fn get_count(&self) -> u8 {
        self.data[0]
    }

    fn set_count(&mut self, count: u8) {
        self.data[0] = if count > self.capacity {
            self.capacity
        } else {
            count
        };
    }

    fn new(d: Option<Vec<u8>>, c: PokeListType, jp: bool) -> Self {
        let mut new = Self {
            string_length: 0,
            japanese: jp,
            capacity: c as u8,
            data: vec![],
            entry_size: 0,
            pokemon: vec![],
        };

        new.entry_size = new.get_entry_size();
        new.string_length = new.get_string_length(jp);
        let mut data = d.unwrap_or_else(|| new.get_empty_list(c, jp));
        let data_size =
            1 + 1 + ((new.capacity as usize) * (new.entry_size + 1 + (2 * new.string_length)));
        data.resize(data_size, 0);
        new.data = data;
        new.pokemon = new.read();
        new
    }

    fn new_empty(c: PokeListType, jp: bool) -> Self {
        let mut new = Self::new(None, c, jp);
        new.set_count(1);
        new
    }

    fn new_from_pk(pk: PK1) -> Self {
        let mut new = Self::new_empty(PokeListType::Single, pk.japanese());
        new[0] = pk;
        new.set_count(1);
        new
    }

    fn get_entry_size(&self) -> usize {
        self.get_entry_size_party(self.is_format_party())
    }

    fn is_format_party(&self) -> bool {
        self.is_capacity_format(self.capacity.into())
    }

    fn is_capacity_format(&self, capacity: PokeListType) -> bool {
        capacity == PokeListType::Single || capacity == PokeListType::Party
    }

    fn get_data_size(&self, c: PokeListType, jp: bool, entry_size: usize) -> usize {
        let entry_length = 1 + entry_size + (2 * self.get_string_length(jp));
        2 + ((c as usize) * entry_length)
    }

    fn get_species_box_identifier(&self, pk: PK1) -> u8 {
        species_converter::set_g1_species(pk.get_species())
    }

    fn get_entry_from_info(&self, dat: Vec<u8>, _ot_name: &[u8], _nick: &[u8], _egg: bool) -> PK1 {
        <PK1 as GBPkml<PersonalInfoG1>>::new(dat, self.japanese)
    }

    fn write(&mut self) -> Vec<u8> {
        let count = self.pokemon.iter().position(|p| p.get_species() == 0);
        self.set_count(count.unwrap_or(self.capacity as usize) as u8);
        let base_ofs = 1 + 1 + self.capacity;
        for i in 0..self.get_count() {
            self.data[1 + i as usize] =
                self.get_species_box_identifier(self.pokemon[i as usize].clone());
            self.set_entry(base_ofs as usize, i as usize);
        }
        let count = self.get_count();
        self.data[1 + count as usize] = SLOT_NONE;
        self.data.clone()
    }
}

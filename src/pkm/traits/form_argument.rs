use crate::form_argument_util::{get_form_argument_max, is_form_argument_type_date_pair};

pub trait FormArgument {
    fn get_form_argument(&self) -> usize;
    fn set_form_argument(&mut self, arg: usize);
    fn get_form_argument_remain(&self) -> u8;
    fn set_form_argument_remain(&mut self, remain: u8);
    fn get_form_argument_elapsed(&self) -> u8;
    fn set_form_argument_elapsed(&mut self, elapsed: u8);
    fn get_form_argument_maximum(&self) -> u8;
    fn set_form_argument_maximum(&mut self, maximum: u8);

    fn change_form_argument(
        &mut self,
        species: usize,
        form: usize,
        generation: usize,
        value: usize,
    ) {
        if !is_form_argument_type_date_pair(species, form) {
            self.set_form_argument(value);
            return;
        }

        let max = get_form_argument_max(species, form, generation);
        self.set_form_argument_remain(value as u8);
        if value == max {
            self.set_form_argument_elapsed(0);
            self.set_form_argument_maximum(0);
            return;
        }

        let elapsed = if max < value {
            0u8
        } else {
            (max - value) as u8
        };
        self.set_form_argument_elapsed(elapsed);
        todo!()
    }
}

pub mod form_argument_util {
    use super::FormArgument;

    pub fn set_suggested_form_argument<T: FormArgument>(_pkm: &mut T, _original_species: usize) {
        todo!()
    }

    pub fn change_form_argument<T: FormArgument>(_pkm: &mut T, _value: usize) {
        todo!()
    }

    pub fn get_form_argument_max(_species: usize, _form: usize, generation: usize) -> usize {
        if generation <= 5 {
            return 0;
        }

        todo!()
    }

    pub fn is_form_argument_type_date_pair(_species: usize, _form: usize) -> bool {
        todo!()
    }
}

use crate::pkx::{get_pkm_extensions, GENERATION};
use crate::{GameValueLimit, Generation, LangNick, Nature, Shiny, SpeciesForm, TrainerId};

pub trait Pkm:
    SpeciesForm + TrainerId + Generation + Shiny + LangNick + GameValueLimit + Nature
{
    fn extensions(&self) -> Vec<String> {
        get_pkm_extensions(GENERATION)
    }

    fn size_party(&self) -> usize;
    fn size_stored(&self) -> usize;

    fn get_type_name(&self) -> String;

    fn extension(&self) -> String {
        self.get_type_name().to_lowercase()
    }

    fn get_personal_info(&self) {
        todo!()
    }
}

use crate::traits::SanityChecksum;
use crate::{poke_crypto, PersonalInfo, PKM};

pub fn get_name_regular_with_checksum<
    Personal: PersonalInfo + 'static,
    T: PKM<Personal> + SanityChecksum,
>(
    pk: &T,
) -> String {
    let form = if pk.form() > 0 {
        format!("-{:0>2}", pk.form())
    } else {
        String::new()
    };
    let star = if pk.is_shiny() {
        " ★".to_string()
    } else {
        String::new()
    };
    let chk = pk.get_checksum();
    format!(
        "{:0>3}{form}{star} - {} - {chk:0>4X}{:0>8X}",
        pk.species(),
        pk.nickname(),
        pk.get_encryption_constant()
    )
}

pub fn get_name_regular_without_checksum<
    Personal: PersonalInfo + 'static,
    T: PKM<Personal> + Into<Vec<u8>> + Clone + Copy,
>(
    pk: &T,
) -> String {
    let form = if pk.form() > 0 {
        format!("-{:0>2}", pk.form())
    } else {
        String::new()
    };
    let star = if pk.is_shiny() {
        " ★".to_string()
    } else {
        String::new()
    };
    let data = (*pk).into();
    let chk = poke_crypto::get_chk(&data, pk.size_stored());
    format!(
        "{:0>3}{form}{star} - {} - {chk:0>4X}{:0>8X}",
        pk.species(),
        pk.nickname(),
        pk.get_encryption_constant()
    )
}

// TODO: GBPKM Name

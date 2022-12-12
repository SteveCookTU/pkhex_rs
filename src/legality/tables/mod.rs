pub mod tables1;
pub mod tables2;
pub mod tables3;
pub mod tables4;
pub mod tables5;
pub mod tables6;

const fn get_permit_list<const SIZE: usize>(allowed: &[u16]) -> [bool; SIZE] {
    let mut result = [false; SIZE];
    let mut index = 0;
    while index < allowed.len() {
        result[allowed[index] as usize] = true;
        index += 1;
    }
    result
}

const fn get_permit_list_disallowed<const SIZE: usize>(
    allowed: &[u16],
    disallow: &[u16],
) -> [bool; SIZE] {
    let mut result = get_permit_list(allowed);
    let mut index = 0;
    while index < disallow.len() {
        result[disallow[index] as usize] = false;
        index += 1;
    }
    result
}

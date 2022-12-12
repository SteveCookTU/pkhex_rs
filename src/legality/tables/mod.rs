pub mod tables1;
pub mod tables2;

const fn get_permit_list<const SIZE: usize>(allowed: &[u16]) -> [bool; SIZE] {
    let mut result = [false; SIZE];
    let mut index = 0;
    while index < allowed.len() {
        result[allowed[index] as usize] = true;
        index += 1;
    }
    result
}

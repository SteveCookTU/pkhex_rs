pub fn get_flag(arr: &[u8], offset: usize, mut bit_index: usize) -> bool {
    bit_index &= 7;
    ((arr[offset] >> bit_index) & 1) != 0
}

pub fn set_flag(arr: &mut [u8], offset: usize, mut bit_index: usize, value: bool) {
    bit_index &= 7;
    let current = arr[offset] & !(1 << bit_index);
    let new_value = current | (if value { 1 } else { 0 } << bit_index);
    arr[offset] = new_value;
}

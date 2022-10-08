pub fn get_flag(arr: &[u8], offset: usize, mut bit_index: usize) -> bool {
    bit_index &= 7;
    ((arr[offset] >> bit_index) & 1) != 0
}

pub fn get_flag_from_u8(flags: u8, mut bit_index: usize) -> bool {
    bit_index &= 7;
    ((flags >> bit_index) & 1) != 0
}

pub fn set_flag(arr: &mut [u8], offset: usize, mut bit_index: usize, value: bool) {
    bit_index &= 7;
    let current = arr[offset] & !(1 << bit_index);
    let new_value = current | (u8::from(value) << bit_index);
    arr[offset] = new_value;
}

pub fn set_flag_in_u8(flags: &mut u8, mut bit_index: usize, value: bool) {
    bit_index &= 7;
    let current = *flags & !(1 << bit_index);
    let new_value = current | (u8::from(value) << bit_index);
    *flags = new_value;
}

pub fn get_bit_flag_array<const SIZE: usize>(data: &[u8]) -> [bool; SIZE] {
    let mut result = [false; SIZE];
    for (i, val) in result.iter_mut().enumerate() {
        *val = ((data[i >> 3] >> (i & 7)) & 1) == 1;
    }
    result
}

pub fn set_bit_flag_array(data: &mut [u8], value: &[bool]) {
    for (i, value) in value.iter().enumerate() {
        let ofs = i >> 3;
        let mask = 1 << (i & 7);
        if *value {
            data[ofs] |= mask as u8;
        } else {
            data[ofs] &= (!mask) as u8;
        }
    }
}

pub(crate) const fn concat_all<const SIZE: usize, T: Copy>(params: &[&[T]], init: T) -> [T; SIZE] {
    let mut result = [init; SIZE];

    let mut current_array = 0;
    let mut passed_len = 0;
    let mut index = 0;
    while index < SIZE {
        if (index - passed_len) < params[current_array].len() {
            result[index] = params[current_array][index - passed_len];
            index += 1;
        } else {
            passed_len += params[current_array].len();
            current_array += 1;
        }
    }

    result
}

pub(crate) const fn concat_two<const SIZE: usize, T: Copy>(
    arr1: &[T],
    arr2: &[T],
    init: T,
) -> [T; SIZE] {
    let mut result = [init; SIZE];
    let mut index = 0;
    while index < SIZE {
        if index < arr1.len() {
            result[index] = arr1[index];
        } else {
            result[index] = arr2[index - arr1.len()]
        }
        index += 1;
    }
    result
}

pub(crate) const fn concat_three<const SIZE: usize, T: Copy>(
    arr1: &[T],
    arr2: &[T],
    arr3: &[T],
    init: T,
) -> [T; SIZE] {
    let mut result = [init; SIZE];
    let mut index = 0;
    while index < SIZE {
        if index < arr1.len() {
            result[index] = arr1[index];
        } else if (index - arr1.len()) < arr2.len() {
            result[index] = arr2[index - arr1.len()]
        } else {
            result[index] = arr3[index - arr1.len() - arr2.len()]
        }
        index += 1;
    }
    result
}

pub(crate) const fn concat_four<const SIZE: usize, T: Copy>(
    arr1: &[T],
    arr2: &[T],
    arr3: &[T],
    arr4: &[T],
    init: T,
) -> [T; SIZE] {
    let mut result = [init; SIZE];
    let mut index = 0;
    while index < SIZE {
        if index < arr1.len() {
            result[index] = arr1[index];
        } else if (index - arr1.len()) < arr2.len() {
            result[index] = arr2[index - arr1.len()]
        } else if (index - arr1.len() - arr2.len()) < arr3.len() {
            result[index] = arr3[index - arr1.len() - arr2.len()]
        } else {
            result[index] = arr4[index - arr1.len() - arr2.len() - arr3.len()]
        }
        index += 1;
    }
    result
}

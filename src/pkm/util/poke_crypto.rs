use no_std_io::Reader;

pub(crate) const SIZE_8STORED: usize = 8 + (4 * SIZE_8BLOCK);
pub(crate) const SIZE_8PARTY: usize = SIZE_8STORED + 0x10;
const SIZE_8BLOCK: usize = 80;

pub(crate) const SIZE_9STORED: usize = SIZE_8STORED;
pub(crate) const SIZE_9PARTY: usize = SIZE_8PARTY;
const SIZE_9BLOCK: usize = SIZE_8BLOCK;

static BLOCK_POSITION: [u8; 128] = [
    0, 1, 2, 3, 0, 1, 3, 2, 0, 2, 1, 3, 0, 3, 1, 2, 0, 2, 3, 1, 0, 3, 2, 1, 1, 0, 2, 3, 1, 0, 3, 2,
    2, 0, 1, 3, 3, 0, 1, 2, 2, 0, 3, 1, 3, 0, 2, 1, 1, 2, 0, 3, 1, 3, 0, 2, 2, 1, 0, 3, 3, 1, 0, 2,
    2, 3, 0, 1, 3, 2, 0, 1, 1, 2, 3, 0, 1, 3, 2, 0, 2, 1, 3, 0, 3, 1, 2, 0, 2, 3, 1, 0, 3, 2, 1, 0,
    // duplicates of 0-7 to eliminate modulus
    0, 1, 2, 3, 0, 1, 3, 2, 0, 2, 1, 3, 0, 3, 1, 2, 0, 2, 3, 1, 0, 3, 2, 1, 1, 0, 2, 3, 1, 0, 3, 2,
];

static BLOCK_POSITION_INVERT: [u8; 32] = [
    0, 1, 2, 4, 3, 5, 6, 7, 12, 18, 13, 19, 8, 10, 14, 20, 16, 22, 9, 11, 15, 21, 17, 23, 0, 1, 2,
    4, 3, 5, 6, 7, // duplicates of 0-7 to eliminate modulus
];

fn shuffle_array(data: &[u8], sv: u32, block_size: usize) -> Vec<u8> {
    let mut sdata = data.to_vec();
    let index = (sv * 4) as usize;
    let start = 8;
    for block in 0..4 {
        let ofs = BLOCK_POSITION[index + block] as usize;
        let s1 = start + (block_size * ofs);
        let src = &data[s1..(s1 + block_size)];
        let s2 = start + (block_size * block);
        let dest = &mut sdata[s2..(s2 + block_size)];
        dest.copy_from_slice(src);
    }
    sdata
}

fn decrypt_array_8(ekm: &mut [u8]) -> Vec<u8> {
    let pv = ekm.default_read_le::<u32>(0);
    let sv = (pv >> 13) & 31;
    crypt_pkm(ekm, pv, SIZE_8BLOCK);
    shuffle_array(ekm, sv, SIZE_8BLOCK)
}

fn decrypt_array_9(ekm: &mut [u8]) -> Vec<u8> {
    let pv = ekm.default_read_le::<u32>(0);
    let sv = (pv >> 13) & 31;
    crypt_pkm(ekm, pv, SIZE_9BLOCK);
    shuffle_array(ekm, sv, SIZE_9BLOCK)
}

pub fn encrypt_array_8(pk: &[u8]) -> Vec<u8> {
    let pv = pk.default_read_le::<u32>(0);
    let sv = (pv >> 13) & 31;

    let mut ekm = shuffle_array(pk, BLOCK_POSITION_INVERT[sv as usize] as u32, SIZE_8BLOCK);
    crypt_pkm(&mut ekm, pv, SIZE_8BLOCK);
    ekm
}

pub fn encrypt_array_9(pk: &[u8]) -> Vec<u8> {
    let pv = pk.default_read_le::<u32>(0);
    let sv = (pv >> 13) & 31;

    let mut ekm = shuffle_array(pk, BLOCK_POSITION_INVERT[sv as usize] as u32, SIZE_9BLOCK);
    crypt_pkm(&mut ekm, pv, SIZE_9BLOCK);
    ekm
}

fn crypt_pkm(data: &mut [u8], pv: u32, block_size: usize) {
    let start = 8;
    let end = (4 * block_size) + start;
    crypt_array(&mut data[start..end], pv);
    if data.len() > end {
        crypt_array(&mut data[end..], pv);
    }
}

pub fn crypt_array(data: &mut [u8], mut seed: u32) {
    let mut i = 0;
    let end = data.len();
    while i < end {
        seed = seed.wrapping_mul(0x41C64E6D).wrapping_add(0x00006073);
        data[i] ^= (seed >> 16) as u8;
        i += 1;
        data[i] ^= (seed >> 24) as u8;
        i += 1;
    }
}

pub fn decrypt_if_encrypted_8(pk: &mut Vec<u8>) {
    if pk.default_read_le::<u16>(0x70) != 0 || pk.default_read_le::<u16>(0x110) != 0 {
        *pk = decrypt_array_8(pk);
    }
}

pub fn decrypt_if_encrypted_9(pk: &mut Vec<u8>) {
    if pk.default_read_le::<u16>(0x70) != 0 || pk.default_read_le::<u16>(0x110) != 0 {
        *pk = decrypt_array_9(pk);
    }
}

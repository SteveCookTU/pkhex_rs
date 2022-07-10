pub const SIZE_1ULIST: usize = 69;
pub const SIZE_1JLIST: usize = 59;
pub const SIZE_1PARTY: usize = 44;
pub const SIZE_1STORED: usize = 33;

pub const SIZE_2ULIST: usize = 73;
pub const SIZE_2JLIST: usize = 63;
pub const SIZE_2PARTY: usize = 48;
pub const SIZE_2STORED: usize = 32;
pub const SIZE_2STADIUM: usize = 60;

pub const SIZE_3CSTORED: usize = 312;
pub const SIZE_3XSTORED: usize = 196;
pub const SIZE_3PARTY: usize = 100;
pub const SIZE_3STORED: usize = 80;
pub const SIZE_3BLOCK: usize = 12;

pub const SIZE_4PARTY: usize = 236;
pub const SIZE_4STORED: usize = 136;
pub const SIZE_4BLOCK: usize = 32;

pub const SIZE_5PARTY: usize = 220;
pub const SIZE_5STORED: usize = 136;
pub const SIZE_5BLOCK: usize = 32;

pub const SIZE_6PARTY: usize = 0x104;
pub const SIZE_6STORED: usize = 0xE8;
pub const SIZE_6BLOCK: usize = 56;

pub const SIZE_8STORED: usize = 8 + (4 * SIZE_8BLOCK); // 0x148
pub const SIZE_8PARTY: usize = SIZE_8STORED + 0x10; // 0x158
pub const SIZE_8BLOCK: usize = 80; // 0x50

pub const SIZE_8ASTORED: usize = 8 + (4 * SIZE_8ABLOCK); // 0x168
pub const SIZE_8APARTY: usize = SIZE_8ASTORED + 0x10; // 0x178
pub const SIZE_8ABLOCK: usize = 88; // 0x58

const BLOCK_POSITION: [u8; 128] = [
    0, 1, 2, 3, 0, 1, 3, 2, 0, 2, 1, 3, 0, 3, 1, 2, 0, 2, 3, 1, 0, 3, 2, 1, 1, 0, 2, 3, 1, 0, 3, 2,
    2, 0, 1, 3, 3, 0, 1, 2, 2, 0, 3, 1, 3, 0, 2, 1, 1, 2, 0, 3, 1, 3, 0, 2, 2, 1, 0, 3, 3, 1, 0, 2,
    2, 3, 0, 1, 3, 2, 0, 1, 1, 2, 3, 0, 1, 3, 2, 0, 2, 1, 3, 0, 3, 1, 2, 0, 2, 3, 1, 0, 3, 2, 1, 0,
    // duplicates of 0-7 to eliminate modulus
    0, 1, 2, 3, 0, 1, 3, 2, 0, 2, 1, 3, 0, 3, 1, 2, 0, 2, 3, 1, 0, 3, 2, 1, 1, 0, 2, 3, 1, 0, 3, 2,
];

const BLOCK_POSITION_INVERT: [u8; 32] = [
    0, 1, 2, 4, 3, 5, 6, 7, 12, 18, 13, 19, 8, 10, 14, 20, 16, 22, 9, 11, 15, 21, 17, 23, 0, 1, 2,
    4, 3, 5, 6, 7,
];

pub fn shuffle_array(data: &mut Vec<u8>, sv: usize, block_size: usize) -> Vec<u8> {
    let idx = 4 * sv;
    let sdata = data.clone();
    for block in 0..4 {
        let ofs = BLOCK_POSITION[idx + block] as usize;
        data.splice(
            (8 + block_size * block)..(8 + block_size * (block + 1)),
            sdata[(8 + block_size * ofs)..(8 + block_size * (ofs + 1))]
                .iter()
                .cloned(),
        );
    }
    data.clone()
}

pub fn crypt_pkm(data: &mut [u8], pv: usize, block_size: usize) {
    let start = 8;
    let end = (4 * block_size) + start;
    crypt_array(data, pv, start, end);
    if data.len() > end {
        let len = data.len();
        crypt_array(data, pv, end, len);
    }
}

fn crypt_array(data: &mut [u8], mut seed: usize, start: usize, end: usize) {
    let mut i = start;
    while i < end {
        seed = seed.wrapping_mul(0x41C64E6D).wrapping_add(0x00006073);
        data[i] ^= (seed >> 16) as u8;
        i += 1;
        data[i] ^= (seed >> 24) as u8;
        i += 1;
    }
}

pub fn decrypt_array_6(mut ekm: Vec<u8>) -> Vec<u8> {
    let seed = u32::from_le_bytes(ekm[0..4].try_into().unwrap()) as usize;
    let sv = seed >> 13 & 31;
    crypt_pkm(&mut ekm, seed, SIZE_6BLOCK);
    shuffle_array(&mut ekm, sv, SIZE_6BLOCK)
}

pub fn decrypt_array_8(mut ekm: Vec<u8>) -> Vec<u8> {
    let seed = u32::from_le_bytes(ekm[0..4].try_into().unwrap()) as usize;
    let sv = seed >> 13 & 31;
    crypt_pkm(&mut ekm, seed, SIZE_8BLOCK);
    shuffle_array(&mut ekm, sv, SIZE_8BLOCK)
}

pub fn decrypt_array_8a(mut ekm: Vec<u8>) -> Vec<u8> {
    let seed = u32::from_le_bytes(ekm[0..4].try_into().unwrap()) as usize;
    let sv = seed >> 13 & 31;
    crypt_pkm(&mut ekm, seed, SIZE_8ABLOCK);
    shuffle_array(&mut ekm, sv, SIZE_8ABLOCK)
}

pub fn encrypt_array_6(pkm: &[u8]) -> Vec<u8> {
    let pv = u32::from_le_bytes(pkm[0..4].try_into().unwrap());
    let sv = pv >> 13 & 31;

    let mut ekm = shuffle_array(
        &mut pkm.to_owned(),
        BLOCK_POSITION_INVERT[sv as usize] as usize,
        SIZE_6BLOCK,
    );
    crypt_pkm(&mut ekm, pv as usize, SIZE_6BLOCK);
    ekm
}

pub fn encrypt_array_8(pkm: &[u8]) -> Vec<u8> {
    let pv = u32::from_le_bytes(pkm[0..4].try_into().unwrap());
    let sv = pv >> 13 & 31;

    let mut ekm = shuffle_array(
        &mut pkm.to_owned(),
        BLOCK_POSITION_INVERT[sv as usize] as usize,
        SIZE_8BLOCK,
    );
    crypt_pkm(&mut ekm, pv as usize, SIZE_8BLOCK);
    ekm
}

pub fn encrypt_array_8a(pkm: &[u8]) -> Vec<u8> {
    let pv = u32::from_le_bytes(pkm[0..4].try_into().unwrap());
    let sv = pv >> 13 & 31;

    let mut ekm = shuffle_array(
        &mut pkm.to_owned(),
        BLOCK_POSITION_INVERT[sv as usize] as usize,
        SIZE_8ABLOCK,
    );
    crypt_pkm(&mut ekm, pv as usize, SIZE_8ABLOCK);
    ekm
}

pub fn decrypt_if_encrypted_67(pkm: &mut Vec<u8>) {
    if u16::from_le_bytes(pkm[0xC8..0xCA].try_into().unwrap()) != 0
        || u16::from_le_bytes(pkm[0x58..0x5A].try_into().unwrap()) != 0
    {
        *pkm = decrypt_array_6(pkm.clone());
    }
}

pub fn decrypt_if_encrypted_8(pkm: &mut Vec<u8>) {
    if u16::from_le_bytes(pkm[0x70..0x72].try_into().unwrap()) != 0
        || u16::from_le_bytes(pkm[0x110..0x112].try_into().unwrap()) != 0
    {
        *pkm = decrypt_array_8(pkm.clone());
    }
}

pub fn decrypt_if_encrypted_8a(pkm: &mut Vec<u8>) {
    if u16::from_le_bytes(pkm[0x78..0x7A].try_into().unwrap()) != 0
        || u16::from_le_bytes(pkm[0x128..0x12A].try_into().unwrap()) != 0
    {
        *pkm = decrypt_array_8a(pkm.clone());
    }
}

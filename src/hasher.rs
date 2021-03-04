use std::num::Wrapping;

const PRIME: i32 = 269891;

pub fn string_fold_hash(key: &str) -> i32 {
    let mut sum: i64 = 0;
    let mut mul: i64 = 1;

    for (i, c) in key.chars().enumerate() {
        mul = if i % 4 == 0 { 1 } else { mul * 256 };
        sum += (c as i64 * mul) as i64;
    }
    (sum.abs() % PRIME as i64) as i32
}

pub fn pjw_hash(s: &str) -> i32 {
    let bits: u32 = (std::mem::size_of::<u32>() * 8) as u32;
    let three_quarters: u32 = (bits * 3) / 4;
    let one_eighth: u32 = bits / 8;
    let high_bits: u32 = 0xffffffff << (bits - one_eighth);

    let mut hash: u32 = 0;
    let mut test: u32;

    for c in s.chars() {
        hash = (hash << one_eighth) + c as u32;
        test = hash & high_bits;
        if test != 0 {
            hash = (hash ^ (test >> three_quarters)) & !high_bits;
        }
    }
    (hash as i32 & PRIME).abs()
}

pub fn elf_hash(s: &str) -> i32 {
    let mut hash: u32 = 0;
    let mut x: u32;

    for c in s.chars() {
        hash = (hash << 4) + c as u32;
        x = hash & 0xF0000000;
        if x != 0 {
            hash ^= x >> 24;
        }
        hash &= !x;
    }

    (hash as i32 % PRIME).abs()
}

pub fn sdbm_hash(s: &str) -> i32 {
    let mut hash = Wrapping(0i32);

    for c in s.chars() {
        let c_num = Wrapping(c as i32);
        hash = c_num + (hash << 6) + (hash << 16) - hash;
    }

    (hash.0 % PRIME).abs()
}

pub fn dek_hash(s: &str) -> i32 {
    let mut hash: u32 = s.len() as u32;

    for c in s.chars() {
        hash = ((hash << 5) ^ (hash >> 27)) ^ c as u32;
    }

    (hash as i32 % PRIME).abs()
}

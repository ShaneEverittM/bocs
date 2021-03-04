
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
    // const unsigned int Bits = (unsigned int) (sizeof(unsigned int) * 8);
    let bits: u32 = (std::mem::size_of::<u32>() * 8) as u32;
    // const unsigned int ThreeQuarters = (unsigned int) ((Bits * 3) / 4);
    let three_quarters: u32 = (bits * 3) / 4;
    // const unsigned int OneEighth = (unsigned int) (Bits / 8);
    let one_eighth: u32 = bits / 8;
    // const unsigned int HighBits = (unsigned int) (0xFFFFFFFF) << (Bits - OneEighth);
    let high_bits: u32 = 0xffffffff << (bits - one_eighth);

    // unsigned int hash{};
    let mut hash: u32 = 0;
    // unsigned int test{};
    let mut test: u32;

    // for (int i = 0; i < s.length(); i++) {
    for c in s.chars() {
        // hash = (hash << OneEighth) + s[i];
        hash = (hash << one_eighth) + c as u32;
        // if ((test = hash & HighBits) != 0) {
        test = hash & high_bits;
        if test != 0 {
            //  hash = ((hash ^ (test >> ThreeQuarters)) & (~HighBits));
            hash = (hash ^ (test >> three_quarters)) & !high_bits;
        }
    }
    // return abs((int) (hash % prime));
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
    let mut hash: u32 = 0;

    for c in s.chars() {
        hash = c as u32 + (hash << 6) + (hash << 16) - hash;
    }

    (hash as i32 % PRIME).abs()
}

pub fn dek_hash(s: &str) -> i32 {
    let mut hash: u32 = s.len() as u32;

    for c in s.chars() {
        hash = ((hash << 5) ^ (hash >> 27)) ^ c as u32;
    }

    (hash as i32 % PRIME).abs()
}

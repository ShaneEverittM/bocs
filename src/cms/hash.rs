//! A set of hash functions that take compresses u:v:o:p representations. These hash algorithms
//! were found to be effective on this representation.

use std::num::Wrapping;

const PRIME: i32 = 269891;

/// Performs string fold hashing on the given key. String fold hashing operates on 4 byte chunks
/// of the input string, folding them into sum then modulating the value into a reasonable range.
pub fn string_fold_hash(key: &str) -> i32 {
    let mut sum: i64 = 0;
    let mut mul: i64 = 1;

    for (i, c) in key.chars().enumerate() {
        mul = if i % 4 == 0 { 1 } else { mul * 256 };
        sum += (c as i64 * mul) as i64;
    }
    (sum.abs() % PRIME as i64) as i32
}

/// Performs PJW hash on the given key. PJW hash basically shifts the previous hash adding the
/// current byte then moves the high bits.
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

/// Performs ELF hash on the given key. ELF hash is very similar to [PJW hash](fn.pjw_hash.html) and
/// is used in unix ELF file generation.
pub fn elf_hash(s: &str) -> i32 {
    let mut hash: u32 = 0;
    let mut x: u32;

    for c in s.bytes() {
        hash = (hash << 4) + c as u32;
        x = hash & 0xF0000000;
        if x != 0 {
            hash ^= x >> 24;
        }
        hash &= !x;
    }

    (hash as i32 % PRIME).abs()
}

/// Performs SDBM hash on the given key. This hash function seems to have a good over-all
/// distribution for many different data sets. It seems to work well in situations where there is
/// a high variance in the MSBs of the elements in a data set.
pub fn sdbm_hash(s: &str) -> i32 {
    let mut hash = Wrapping(0i32);

    for c in s.chars() {
        let c_num = Wrapping(c as i32);
        hash = c_num + (hash << 6) + (hash << 16) - hash;
    }

    (hash.0 % PRIME).abs()
}

/// Performs DEK hash on the given key. This is the hash algorithm proposed by Donald Knuth in
/// The Art of Computer Programming Volume 3.
pub fn dek_hash(s: &str) -> i32 {
    let mut hash: u32 = s.len() as u32;

    for c in s.chars() {
        hash = ((hash << 5) ^ (hash >> 27)) ^ c as u32;
    }

    (hash as i32 % PRIME).abs()
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn string_fold() {
        let raw1 = "ENSG00000164164:ENSG00000175376:11:12";
        let raw2 = "ENSG00000006194:ENSG00000174851:6:6";
        let hash1 = string_fold_hash(raw1);
        let hash2 = string_fold_hash(raw2);
        assert_ne!(hash1, hash2);
    }
}

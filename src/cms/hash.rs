//! A set of hash functions that take compresses u:v:o:p representations. These hash algorithms
//! were found to be effective on this representation.

use std::num::Wrapping;

const PRIME: usize = 269891;

pub fn js_hash(key: &str) -> usize {
    let mut hash: usize = 378551;

    for c in key.chars() {
        hash ^= (hash << 5).wrapping_add(c as usize).wrapping_add(hash >> 2);
    }

    hash % PRIME
}

pub fn rs_hash(key: &str) -> usize {
    let b: usize = 378551;
    let mut a: usize = 63689;
    let mut hash: usize = 0;

    for c in key.chars() {
        hash = hash.wrapping_mul(a).wrapping_add(c as usize);
        a = a.wrapping_mul(b);
    }

    hash % PRIME
}

pub fn bkdr_hash(key: &str) -> usize {
    let seed: usize = 131;
    let mut hash: usize = 0;

    for c in key.chars() {
        hash = hash.wrapping_mul(seed).wrapping_add(c as usize);
    }

    hash % PRIME
}

/// Performs string fold hashing on the given key. String fold hashing operates on 4 byte chunks
/// of the input string, folding them into sum then modulating the value into a reasonable range.
pub fn string_fold_hash(key: &str) -> usize {
    let mut sum = 0;
    let mut mul = 1;

    for (i, c) in key.chars().enumerate() {
        mul = if i % 4 == 0 { 1 } else { mul * 256 };
        sum += c as usize * mul;
    }
    sum % PRIME
}

/// Performs PJW hash on the given key. PJW hash basically shifts the previous hash adding the
/// current byte then moves the high bits.
pub fn pjw_hash(s: &str) -> usize {
    let bits: usize = std::mem::size_of::<usize>() * 8;
    let three_quarters = (bits * 3) / 4;
    let one_eighth = bits / 8;
    let high_bits = 0xffffffff << (bits - one_eighth);

    let mut hash = 0;
    let mut test;

    for c in s.chars() {
        hash = (hash << one_eighth) + c as usize;
        test = hash & high_bits;
        if test != 0 {
            hash = (hash ^ (test >> three_quarters)) & !high_bits;
        }
    }
    hash & PRIME
}

/// Performs ELF hash on the given key. ELF hash is very similar to [PJW hash](fn.pjw_hash.html) and
/// is used in unix ELF file generation.
pub fn elf_hash(s: &str) -> usize {
    let mut hash: usize = 0;
    let mut x: usize;

    for c in s.bytes() {
        hash = (hash << 4) + c as usize;
        x = hash & 0xF0000000;
        if x != 0 {
            hash ^= x >> 24;
        }
        hash &= !x;
    }

    hash % PRIME
}

/// Performs SDBM hash on the given key. This hash function seems to have a good over-all
/// distribution for many different data sets. It seems to work well in situations where there is
/// a high variance in the MSBs of the elements in a data set.
pub fn sdbm_hash(s: &str) -> usize {
    let mut hash = Wrapping(0usize);

    for c in s.chars() {
        let c_num = Wrapping(c as usize);
        hash = c_num + (hash << 6) + (hash << 16) - hash;
    }

    hash.0 % PRIME
}

#[cfg(test)]
mod tests {
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

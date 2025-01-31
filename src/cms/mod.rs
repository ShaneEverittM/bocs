//! A specialized implementation of the
//! [Count-Min Sketch data structure](https://en.wikipedia.org/wiki/Count%E2%80%93min_sketch).

mod hash;

use hash::Hasher;

type Count = u16;

static PRIMES: &[usize] = &[
    32729,
    271927,
    2718409,
    33554467,
    271828199,
    4294967311,
    34359738421,
    271828182863,
];

/// A specialized implementation of the
/// [Count-Min Sketch data structure](https://en.wikipedia.org/wiki/Count%E2%80%93min_sketch).
pub struct CountMinSketch {
    depth: usize,
    width: usize,
    hasher: Hasher,
    table: Vec<Vec<Count>>,
}

impl CountMinSketch {
    /// Construct an empty CMS with the specified error rate and confidence. The parameters
    /// assert that any count returned from the CMS will be at most `error_rate` over the actual
    /// count `confidence` percent of the time.
    pub fn new(error_rate: f64, confidence: f64) -> Self {
        let depth = f64::ceil(f64::ln(1.0 / (1.0 - confidence / 100.0))) as usize;
        let width = f64::ceil(std::f64::consts::E / error_rate) as usize;

        let mut prime = 0;
        for &potential_prime in PRIMES.iter() {
            if width < potential_prime {
                prime = potential_prime;
                break;
            }
        }

        Self {
            depth,
            width,
            hasher: Hasher::with_prime(prime),
            table: vec![vec![0; width]; depth],
        }
    }

    /// Retrieves a value from the CMS. Expects a condensed format of the input to support
    /// hashing. This condensed value is the `raw` field in
    /// [`parser::MotifInfo`](../parser/struct.MotifInfo.html)
    pub fn get(&self, raw: &str) -> Option<Count> {
        let mut hashed_freq = Count::max_value();
        let mut hash_value;
        for i in 0..self.depth {
            hash_value = self.cms_hash(raw, i);
            hashed_freq = hashed_freq.min(self.table[i][(hash_value % self.width)])
        }

        if hashed_freq > 0 {
            Some(hashed_freq)
        } else {
            None
        }
    }

    /// Inserts a value into the CMS.
    pub fn put(&mut self, raw: &str) {
        let mut hash_value: usize;
        for i in 0..self.depth {
            hash_value = self.cms_hash(raw, i);
            self.table[i][(hash_value % self.width)] += 1;
        }
    }

    fn cms_hash(&self, raw: &str, idx: usize) -> usize {
        match idx {
            0 => self.hasher.js_hash(raw),
            1 => self.hasher.bkdr_hash(raw),
            2 => self.hasher.rs_hash(raw),
            3 => self.hasher.string_fold_hash(raw),
            4 => self.hasher.pjw_hash(raw),
            5 => self.hasher.elf_hash(raw),
            6 => self.hasher.sdbm_hash(raw),
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_get() {
        let s = "cool_value";
        let c = "other_value";
        let k = "not_value";

        let mut cms = CountMinSketch::new(1e-5, 99.0);

        cms.put(s);
        cms.put(s);
        cms.put(s);
        cms.put(c);
        cms.put(c);

        assert_eq!(cms.get(s).unwrap(), 3);
        assert_eq!(cms.get(c).unwrap(), 2);
        assert_eq!(cms.get(k), None);
    }
}

//! A specialized implementation of the
//! [Count-Min Sketch data structure](https://en.wikipedia.org/wiki/Count%E2%80%93min_sketch).

pub mod hash;

/// A specialized implementation of the
/// [Count-Min Sketch data structure](https://en.wikipedia.org/wiki/Count%E2%80%93min_sketch).
pub struct CountMinSketch {
    depth: usize,
    width: usize,
    table: Vec<Vec<usize>>,
}

impl CountMinSketch {
    /// Construct an empty CMS with the specified error rate and confidence. The parameters
    /// assert that any count returned from the CMS will be at most `error_rate` over the actual
    /// count `confidence` percent of the time.
    pub fn new(error_rate: f64, confidence: f64) -> Self {
        let depth = f64::ceil(f64::ln(1.0 / (100.0 - confidence))) as usize;
        let width = f64::ceil(std::f64::consts::E / error_rate) as usize;
        Self {
            depth,
            width,
            table: vec![vec![0; width]; depth],
        }
    }

    /// Retrieves a value from the CMS. Expects a condensed format of the input to support
    /// hashing. This condensed value is the `raw` field in
    /// [`parser::MotifInfo`](../parser/struct.MotifInfo.html)
    pub fn get(&self, raw: &str) -> Option<usize> {
        let mut hashed_freq = usize::max_value();
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
            0 => hash::string_fold_hash(raw),
            1 => hash::pjw_hash(raw),
            2 => hash::rs_hash(raw),
            3 => hash::js_hash(raw),
            4 => hash::bkdr_hash(raw),
            5 => hash::elf_hash(raw),
            6 => hash::sdbm_hash(raw),
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

        let mut cms = CountMinSketch::new(1e-5, 99.99);

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

use crate::hasher;
use crate::parser;

pub struct CountMinSketch {
    error_rate: f64,
    confidence: f64,
    depth: usize,
    width: usize,
    table: Vec<Vec<i32>>,
}

impl CountMinSketch {
    pub fn new(error_rate: f64, confidence: f64) -> Self {
        let depth: usize = f64::ceil(f64::ln(1.0 / (100.0 - confidence))) as usize;
        let width: usize = f64::ceil(std::f64::consts::E / error_rate) as usize;
        Self {
            error_rate,
            confidence,
            depth,
            width,
            table: vec![vec![0; width]; depth],
        }
    }

    fn cms_hash(&self, motif_info: &parser::MotifInfo, idx: i32) -> i32 {
        match idx {
            0 => hasher::string_fold_hash(&motif_info.raw),
            1 => hasher::pjw_hash(&motif_info.raw),
            2 => hasher::elf_hash(&motif_info.raw),
            3 => hasher::sdbm_hash(&motif_info.raw),
            4 => hasher::dek_hash(&motif_info.raw),
            _ => 0,
        }
    }

    pub fn get(&self, mi: &parser::MotifInfo) -> i32 {
        let mut hashed_freq: i32 = i32::max_value();
        let mut hash_value: i32;
        for i in 0..self.depth {
            hash_value = self.cms_hash(mi, i as i32);
            hashed_freq = hashed_freq.min(self.table[i][(hash_value % self.width as i32) as usize])
        }
        hashed_freq
    }

    pub fn put(&mut self, mi: &parser::MotifInfo) {
        let mut hash_value: i32;
        for i in 0..self.depth {
            hash_value = self.cms_hash(mi, i as i32);
            self.table[i][(hash_value % self.width as i32) as usize] += 1;
        }
    }
}

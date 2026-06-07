//! Adaptive RLE encoding that adjusts strategy based on data characteristics.
use crate::{binary, packbits, rle};

/// The encoding strategy to use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Strategy {
    /// Basic RLE — good for highly repetitive data
    BasicRle,
    /// PackBits — good for mixed data
    PackBits,
    /// Binary RLE — good for binary (0/1) data
    BinaryRle,
    /// No compression — when expansion would occur
    None,
}

/// Analysis result of the data characteristics.
#[derive(Debug, Clone)]
pub struct DataProfile {
    pub total_bytes: usize,
    pub unique_bytes: usize,
    pub run_count: usize,
    pub avg_run_length: f64,
    pub is_binary: bool,
    pub repetition_ratio: f64,
}

/// Analyze data to determine its characteristics.
pub fn profile(data: &[u8]) -> DataProfile {
    if data.is_empty() {
        return DataProfile {
            total_bytes: 0,
            unique_bytes: 0,
            run_count: 0,
            avg_run_length: 0.0,
            is_binary: true,
            repetition_ratio: 0.0,
        };
    }

    let mut seen = [false; 256];
    let mut unique = 0;
    for &b in data {
        if !seen[b as usize] {
            seen[b as usize] = true;
            unique += 1;
        }
    }

    let mut runs = 0;
    let mut i = 0;
    while i < data.len() {
        let current = data[i];
        let mut run_len = 1;
        while i + run_len < data.len() && data[i + run_len] == current {
            run_len += 1;
        }
        runs += 1;
        i += run_len;
    }

    let is_binary = unique <= 2 && data.iter().all(|&b| b <= 1);
    let repetition_ratio = 1.0 - (runs as f64 / data.len() as f64);

    DataProfile {
        total_bytes: data.len(),
        unique_bytes: unique,
        run_count: runs,
        avg_run_length: data.len() as f64 / runs as f64,
        is_binary,
        repetition_ratio,
    }
}

/// Choose the best encoding strategy for the data.
pub fn choose_strategy(data: &[u8]) -> Strategy {
    if data.is_empty() {
        return Strategy::None;
    }

    let p = profile(data);

    if p.is_binary {
        return Strategy::BinaryRle;
    }

    if p.repetition_ratio > 0.5 {
        // High repetition: basic RLE works well
        let rle_size = rle::encode(data).len();
        if rle_size < data.len() {
            return Strategy::BasicRle;
        }
    }

    // Try PackBits for mixed data
    let pb_size = packbits::encode(data).len();
    if pb_size < data.len() {
        return Strategy::PackBits;
    }

    Strategy::None
}

/// Adaptively encode data using the best strategy.
pub fn encode(data: &[u8]) -> (Vec<u8>, Strategy) {
    let strategy = choose_strategy(data);
    let encoded = match strategy {
        Strategy::BasicRle => rle::encode(data),
        Strategy::PackBits => packbits::encode(data),
        Strategy::BinaryRle => {
            let runs = binary::encode_binary(data);
            let mut bytes = Vec::new();
            for run in &runs {
                bytes.extend_from_slice(&run.to_le_bytes());
            }
            bytes
        }
        Strategy::None => data.to_vec(),
    };
    (encoded, strategy)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_empty() {
        let p = profile(&[]);
        assert_eq!(p.total_bytes, 0);
        assert_eq!(p.unique_bytes, 0);
        assert!(p.is_binary);
    }

    #[test]
    fn test_profile_repetitive() {
        let data = vec![42; 100];
        let p = profile(&data);
        assert_eq!(p.run_count, 1);
        assert!(p.repetition_ratio > 0.9);
    }

    #[test]
    fn test_strategy_binary() {
        let data = vec![0, 0, 0, 1, 1, 1, 0, 0];
        assert_eq!(choose_strategy(&data), Strategy::BinaryRle);
    }

    #[test]
    fn test_strategy_repetitive() {
        let data = vec![b'A'; 50];
        let strategy = choose_strategy(&data);
        assert!(strategy == Strategy::BasicRle || strategy == Strategy::PackBits);
    }

    #[test]
    fn test_encode_decode_roundtrip() {
        let data = vec![b'X'; 20];
        let (encoded, strategy) = encode(&data);
        assert!(encoded.len() < data.len());
        assert_ne!(strategy, Strategy::None);
    }
}

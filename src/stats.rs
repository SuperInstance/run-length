//! Compression statistics and analysis utilities.
pub fn compression_ratio(original_len: usize, compressed_len: usize) -> f64 {
    if compressed_len == 0 {
        return f64::INFINITY;
    }
    original_len as f64 / compressed_len as f64
}

/// Calculate the space savings percentage.
pub fn space_savings(original_len: usize, compressed_len: usize) -> f64 {
    if original_len == 0 {
        return 0.0;
    }
    (1.0 - compressed_len as f64 / original_len as f64) * 100.0
}

/// Calculate bits per symbol.
pub fn bits_per_symbol(total_bits: usize, symbol_count: usize) -> f64 {
    if symbol_count == 0 {
        return 0.0;
    }
    total_bits as f64 / symbol_count as f64
}

/// Compression result summary.
#[derive(Debug, Clone)]
pub struct CompressionResult {
    pub original_size: usize,
    pub compressed_size: usize,
    pub ratio: f64,
    pub savings_pct: f64,
}

impl CompressionResult {
    pub fn new(original_size: usize, compressed_size: usize) -> Self {
        Self {
            original_size,
            compressed_size,
            ratio: compression_ratio(original_size, compressed_size),
            savings_pct: space_savings(original_size, compressed_size),
        }
    }
}

/// Estimate the worst-case expansion factor for RLE.
pub fn worst_case_expansion(original_len: usize) -> usize {
    // Worst case: every byte is different, encoded as 2 bytes each
    original_len * 2
}

/// Calculate the entropy of a byte distribution.
pub fn byte_entropy(data: &[u8]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }

    let mut counts = [0usize; 256];
    for &b in data {
        counts[b as usize] += 1;
    }

    let total = data.len() as f64;
    let mut entropy = 0.0;
    for &count in &counts {
        if count > 0 {
            let p = count as f64 / total;
            entropy -= p * p.log2();
        }
    }
    entropy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression_ratio() {
        assert!((compression_ratio(100, 50) - 2.0).abs() < f64::EPSILON);
        assert!((compression_ratio(100, 200) - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_space_savings() {
        assert!((space_savings(100, 50) - 50.0).abs() < f64::EPSILON);
        assert!((space_savings(100, 100)).abs() < f64::EPSILON);
    }

    #[test]
    fn test_bits_per_symbol() {
        assert!((bits_per_symbol(800, 100) - 8.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_compression_result() {
        let result = CompressionResult::new(1000, 400);
        assert!((result.ratio - 2.5).abs() < f64::EPSILON);
        assert!((result.savings_pct - 60.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_worst_case_expansion() {
        assert_eq!(worst_case_expansion(100), 200);
    }

    #[test]
    fn test_byte_entropy() {
        // Single byte repeated: entropy = 0
        let data = vec![42; 100];
        assert!(byte_entropy(&data).abs() < f64::EPSILON);

        // Uniform distribution over 256 values: entropy = 8
        let data: Vec<u8> = (0..=255).cycle().take(2560).collect();
        assert!((byte_entropy(&data) - 8.0).abs() < 0.01);
    }
}

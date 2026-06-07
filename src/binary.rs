//! Modified RLE optimized for binary (0/1) data.
//! Uses bit-packing for efficient storage of binary sequences.
pub fn encode_binary(data: &[u8]) -> Vec<u32> {
    if data.is_empty() {
        return Vec::new();
    }

    let mut runs = Vec::new();
    let mut current_val = 0u8; // We always start counting 0s first
    let mut count = 0u32;

    for &byte in data {
        if byte <= 1 {
            let val = byte;
            if val == current_val {
                count += 1;
            } else {
                runs.push(count);
                current_val = val;
                count = 1;
            }
        }
    }
    if count > 0 {
        runs.push(count);
    }

    runs
}

/// Decode binary RLE data back to binary bytes.
/// `first_value` is the value of the first run (0 or 1).
pub fn decode_binary(runs: &[u32], first_value: u8) -> Vec<u8> {
    let mut result = Vec::new();
    let mut current_val = first_value.min(1);

    for &count in runs {
        for _ in 0..count {
            result.push(current_val);
        }
        current_val = 1 - current_val;
    }

    result
}

/// Encode a binary image (2D grid) row by row.
pub fn encode_image(width: usize, height: usize, data: &[u8]) -> Vec<Vec<u32>> {
    let mut rows = Vec::new();
    for y in 0..height {
        let start = y * width;
        let end = (y + 1) * width.min(data.len() - start + start);
        let row_data = if start < data.len() {
            &data[start..end.min(data.len())]
        } else {
            &[]
        };
        rows.push(encode_binary(row_data));
    }
    rows
}

/// Decode a binary image row by row.
pub fn decode_image(rows: &[Vec<u32>], width: usize) -> Vec<u8> {
    let mut result = Vec::new();
    for row_runs in rows {
        let mut row = decode_binary(row_runs, 0);
        row.truncate(width);
        result.extend(row);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert!(encode_binary(&[]).is_empty());
        assert!(decode_binary(&[], 0).is_empty());
    }

    #[test]
    fn test_all_zeros() {
        let data = vec![0; 10];
        let runs = encode_binary(&data);
        assert_eq!(runs, vec![10]);
        let decoded = decode_binary(&runs, 0);
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_alternating() {
        let data = vec![0, 1, 0, 1, 0, 1];
        let runs = encode_binary(&data);
        assert_eq!(runs, vec![1, 1, 1, 1, 1, 1]);
    }

    #[test]
    fn test_binary_roundtrip() {
        let data = vec![0, 0, 0, 1, 1, 0, 0, 0, 0];
        let runs = encode_binary(&data);
        let decoded = decode_binary(&runs, 0);
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_image_encode_decode() {
        let width = 4;
        let height = 3;
        let data = vec![
            0, 0, 1, 1,
            1, 1, 0, 0,
            0, 0, 0, 0,
        ];
        let rows = encode_image(width, height, &data);
        assert_eq!(rows.len(), 3);
        let decoded = decode_image(&rows, width);
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_image_compression_ratio() {
        // Uniform rows should compress well
        let width = 8;
        let mut data = Vec::new();
        for _ in 0..8 {
            data.extend_from_slice(&vec![0; 8]);
        }
        let rows = encode_image(width, 8, &data);
        let total_runs: usize = rows.iter().map(|r| r.len()).sum();
        // Each row is a single run of 8 zeros
        assert_eq!(total_runs, 8);
    }
}

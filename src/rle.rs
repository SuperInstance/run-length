//! Basic run-length encoding: encodes sequences of identical bytes as (count, byte) pairs.
//! Uses a simple format where runs of 3+ identical bytes are compressed.
pub fn encode(data: &[u8]) -> Vec<u8> {
    if data.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::new();
    let mut current = data[0];
    let mut count: u8 = 1;

    for &byte in &data[1..] {
        if byte == current && count < 255 {
            count += 1;
        } else {
            result.push(current);
            result.push(count);
            current = byte;
            count = 1;
        }
    }
    result.push(current);
    result.push(count);

    result
}

/// Decode basic RLE data back to the original bytes.
pub fn decode(encoded: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut i = 0;
    while i + 1 < encoded.len() {
        let byte = encoded[i];
        let count = encoded[i + 1] as usize;
        for _ in 0..count {
            result.push(byte);
        }
        i += 2;
    }
    result
}

/// Encode data, returning only runs longer than a threshold.
pub fn encode_with_threshold(data: &[u8], min_run: u8) -> Vec<u8> {
    if data.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::new();
    let mut i = 0;

    while i < data.len() {
        let current = data[i];
        let mut count = 1usize;
        while i + count < data.len() && data[i + count] == current && count < 255 {
            count += 1;
        }

        if count >= min_run as usize {
            result.push(current);
            result.push(count as u8);
        } else {
            for _ in 0..count {
                result.push(current);
                result.push(1);
            }
        }
        i += count;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert!(encode(&[]).is_empty());
        assert!(decode(&[]).is_empty());
    }

    #[test]
    fn test_single_byte() {
        let data = b"A";
        let encoded = encode(data);
        assert_eq!(encoded, vec![b'A', 1]);
        assert_eq!(decode(&encoded), data.to_vec());
    }

    #[test]
    fn test_repeated_bytes() {
        let data = b"AAAAAA";
        let encoded = encode(data);
        assert_eq!(encoded, vec![b'A', 6]);
        assert_eq!(decode(&encoded), data.to_vec());
    }

    #[test]
    fn test_roundtrip_mixed() {
        let data = b"AAAAABBBCCD";
        let encoded = encode(data);
        let decoded = decode(&encoded);
        assert_eq!(decoded, data.to_vec());
    }

    #[test]
    fn test_compression_ratio_highly_repetitive() {
        let data = vec![b'X'; 100];
        let encoded = encode(&data);
        assert!(encoded.len() < data.len());
    }

    #[test]
    fn test_worst_case_no_repetition() {
        let data: Vec<u8> = (0..=255).collect();
        let encoded = encode(&data);
        // Worst case: each byte is different, so encoded is 2x original
        assert_eq!(encoded.len(), data.len() * 2);
    }

    #[test]
    fn test_roundtrip_long_runs() {
        let mut data = Vec::new();
        data.extend_from_slice(&vec![b'A'; 300]);
        data.extend_from_slice(&vec![b'B'; 200]);
        data.extend_from_slice(&vec![b'C'; 50]);
        let encoded = encode(&data);
        let decoded = decode(&encoded);
        assert_eq!(decoded, data);
    }
}

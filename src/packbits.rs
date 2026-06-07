//! PackBits encoding algorithm (Apple variant).
//! Efficient for data with mixed runs and literal sequences.
pub fn encode(data: &[u8]) -> Vec<u8> {
    if data.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::new();
    let len = data.len();
    let mut i = 0;

    while i < len {
        // Check for a run of identical bytes
        let mut run_len = 1;
        while i + run_len < len && data[i + run_len] == data[i] && run_len < 128 {
            run_len += 1;
        }

        if run_len >= 2 {
            // Encode as a run
            result.push((257 - run_len) as u8);
            result.push(data[i]);
            i += run_len;
        } else {
            // Literal sequence: collect bytes until we find a run of 2+
            let start = i;
            let mut lit_len = 0;

            while i < len && lit_len < 128 {
                // Check if a run starts here
                let mut peek_run = 1;
                while i + peek_run < len && data[i + peek_run] == data[i] && peek_run < 2 {
                    peek_run += 1;
                }
                if peek_run >= 2 {
                    break; // Start a run instead
                }
                lit_len += 1;
                i += 1;
            }

            if lit_len > 0 {
                result.push((lit_len - 1) as u8);
                result.extend_from_slice(&data[start..start + lit_len]);
            }
        }
    }

    result
}

/// Decode PackBits-encoded data.
pub fn decode(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < data.len() {
        let header = data[i] as i8 as i16;
        i += 1;

        if (0..=127).contains(&header) {
            // Copy next header+1 literal bytes
            let count = (header + 1) as usize;
            if i + count > data.len() {
                break;
            }
            result.extend_from_slice(&data[i..i + count]);
            i += count;
        } else if (-127..=-1).contains(&header) {
            // Repeat next byte (1-header) times
            let count = (1 - header) as usize;
            if i >= data.len() {
                break;
            }
            let val = data[i];
            for _ in 0..count {
                result.push(val);
            }
            i += 1;
        }
        // header == -128 (128 unsigned): nop
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
    fn test_all_same() {
        let data = vec![0xAA; 10];
        let encoded = encode(&data);
        let decoded = decode(&encoded);
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_all_different() {
        let data: Vec<u8> = (0..50).collect();
        let encoded = encode(&data);
        let decoded = decode(&encoded);
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_mixed_runs_and_literals() {
        let mut data = Vec::new();
        data.extend_from_slice(&[1, 2, 3]); // literal
        data.extend_from_slice(&vec![4; 20]); // run
        data.extend_from_slice(&[5, 6]); // literal
        let encoded = encode(&data);
        let decoded = decode(&encoded);
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_roundtrip_random() {
        let data = vec![0xAA, 0xAA, 0xAA, 0x00, 0x01, 0x02, 0xFF, 0xFF, 0xFF, 0xFF];
        let encoded = encode(&data);
        let decoded = decode(&encoded);
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_compression_all_repeated() {
        let data = vec![42; 100];
        let encoded = encode(&data);
        assert!(encoded.len() < data.len());
    }
}

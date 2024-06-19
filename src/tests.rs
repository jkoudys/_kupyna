use super::*;

#[test]
fn test_pad_message() {
    let message = b"hello";
    let padded_message = pad_message(message, L512);
    assert_eq!(padded_message.len() % (L512 / 8), 0);
}

#[test]
fn test_pad_message_length() {
    let message = b"hello";
    let padded_message = pad_message(message, L512);
    let length_field = &padded_message[padded_message.len() - 12..];
    let length_bits = u128::from_le_bytes({
        let mut temp = [0u8; 16];
        temp[0..12].copy_from_slice(length_field);
        temp
    });
    assert_eq!(length_bits, (message.len() * 8) as u128);
}

#[test]
fn test_divide_into_blocks() {
    let message = b"hello";
    let padded_message = pad_message(message, L512);
    let blocks = divide_into_blocks(&padded_message, L512);
    assert_eq!(blocks.len(), padded_message.len() / (L512 / 8));
}

#[test]
fn test_xor_bytes() {
    let a = [0b10101010, 0b11110000];
    let b = [0b01010101, 0b00001111];
    let result = xor_bytes(&a, &b);
    assert_eq!(result, vec![0b11111111, 0b11111111]);
}

#[test]
fn test_r_l_n() {
    let block = vec![0xFF; 64]; // 512 bits
    let truncated = r_l_n(&block, 256);
    assert_eq!(truncated.len(), 256 / 8);
}

// Source: https://en.wikipedia.org/wiki/Kupyna
#[test]
fn test_kupyna_hash_empty_256() {
    let message = b"";
    let hash = kupyna_hash(message, 256);
    assert_eq!(
            hash,
            vec![
                0xcd, 0x51, 0x01, 0xd1, 0xcc, 0xdf, 0x0d, 0x1d, 0x1f, 0x4a, 0xda, 0x56, 0xe8, 0x88,
                0xcd, 0x72, 0x4c, 0xa1, 0xa0, 0x83, 0x8a, 0x35, 0x21, 0xe7, 0x13, 0x1d, 0x4f, 0xb7,
                0x8d, 0x0f, 0x5e, 0xb6
            ],
        );
}

#[test]
fn test_kupyna_hash_empty_512() {
    let message = b"";
    let hash = kupyna_hash(message, 512);
    // Source: https://en.wikipedia.org/wiki/Kupyna
    assert_eq!(
            hash,
            vec![
                0x65, 0x6b, 0x2f, 0x4c, 0xd7, 0x14, 0x62, 0x38, 0x8b, 0x64, 0xa3, 0x70, 0x43, 0xea,
                0x55, 0xdb, 0xe4, 0x45, 0xd4, 0x52, 0xae, 0xcd, 0x46, 0xc3, 0x29, 0x83, 0x43, 0x31,
                0x4e, 0xf0, 0x40, 0x19
            ],
        );
}

#[test]
fn test_kupyna_hash_dog() {
    let message = b"The quick brown fox jumps over the lazy dog.";
    let hash = kupyna_hash(message, 256);
    // Source: https://en.wikipedia.org/wiki/Kupyna
    assert_eq!(
            hash,
            vec![
                0x99, 0x68, 0x99, 0xf2, 0xd7, 0x42, 0x2c, 0xea, 0xf5, 0x52, 0x47, 0x50, 0x36, 0xb2,
                0xdc, 0x12, 0x06, 0x07, 0xef, 0xf5, 0x38, 0xab, 0xf2, 0xb8, 0xdf, 0xf4, 0x71, 0xa9,
                0x8a, 0x47, 0x40, 0xc6
            ],
        );
}

#[test]
fn test_kupyna_hash_dog_period() {
    let message = b"The quick brown fox jumps over the lazy dog.";
    let hash = kupyna_hash(message, 256);
    // Source: https://en.wikipedia.org/wiki/Kupyna
    assert_eq!(
            hash,
            vec![
                0x88, 0xea, 0x8c, 0xe9, 0x88, 0xfe, 0x67, 0xeb, 0x83, 0x96, 0x8c, 0xdc, 0x0f, 0x6f,
                0x3c, 0xa6, 0x93, 0xba, 0xa5, 0x02, 0x61, 0x20, 0x86, 0xc0, 0xdc, 0xec, 0x76, 0x1a,
                0x98, 0xe2, 0xfb, 0x1f
            ],
        );
}
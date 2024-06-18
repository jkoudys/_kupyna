const L512: usize = 512;
const L1024: usize = 1024;
const T_512: usize = 10;
const T_1024: usize = 14;

/// Pads the input message according to the Kupyna padding scheme.
///
/// # Arguments
///
/// * `message` - A byte slice representing the message to be hashed.
/// * `l` - The length of the blocks in bits.
///
/// # Returns
///
/// * A `Vec<u8>` containing the padded message.
fn pad_message(message: &[u8], l: usize) -> Vec<u8> {
    let n = message.len() * 8; // length in bits
    let d = ((-((n + 97) as isize) % (l as isize)) + l as isize) as usize;
    // We set the padded message size upfront to reduce allocs
    let paddedlen = message.len() + (d / 8) + 12;
    let mut padded_message = vec![0x00; paddedlen];

    // Copy the input message
    padded_message[0..message.len()].copy_from_slice(message);
    // Set the high bit
    padded_message[message.len()] = 0b10000000;

    // Convert the length to a byte array and copy it into the padded message
    let n_bytes = (n as u128).to_le_bytes(); // message length in little-endian
    padded_message[paddedlen - 12..].copy_from_slice(&n_bytes[0..12]);

    padded_message
}

/// Divides the padded message into blocks of length `l`.
///
/// # Arguments
///
/// * `padded_message` - A byte slice representing the padded message.
/// * `l` - The length of the blocks in bits.
///
/// # Returns
///
/// * A `Vec<&[u8]>` containing references to the blocks.
fn divide_into_blocks(padded_message: &[u8], l: usize) -> Vec<&[u8]> {
    padded_message.chunks(l / 8).collect()
}

/// Placeholder for the T⊕l transformation.
///
/// # Arguments
///
/// * `block` - A byte slice representing the block to be transformed.
/// * `_rounds` - The number of rounds to perform.
///
/// # Returns
///
/// * A `Vec<u8>` containing the transformed block.
fn t_xor_l(block: &[u8], _rounds: usize) -> Vec<u8> {
    // Implement the T⊕l transformation (placeholder)
    block.to_vec()
}

/// Placeholder for the T+l transformation.
///
/// # Arguments
///
/// * `block` - A byte slice representing the block to be transformed.
/// * `_rounds` - The number of rounds to perform.
///
/// # Returns
///
/// * A `Vec<u8>` containing the transformed block.
fn t_plus_l(block: &[u8], _rounds: usize) -> Vec<u8> {
    // Implement the T+l transformation (placeholder)
    block.to_vec()
}

/// Truncates the block to the first `n` bits.
///
/// # Arguments
///
/// * `block` - A byte slice representing the block to be truncated.
/// * `n` - The number of bits to keep.
///
/// # Returns
///
/// * A `Vec<u8>` containing the truncated block.
fn r_l_n(block: &[u8], n: usize) -> Vec<u8> {
    block[0..(n / 8)].to_vec()
}

/// Computes the Kupyna hash of the input message.
///
/// # Arguments
///
/// * `message` - A byte slice representing the message to be hashed.
/// * `n` - The length of the hash code in bits.
///
/// # Returns
///
/// * A `Vec<u8>` containing the hash code.
fn kupyna_hash(message: &[u8], n: usize) -> Vec<u8> {
    let (l, t, iv) = if 8 <= n && n <= 256 {
        (L512, T_512, vec![0x01; 510 / 8])
    } else {
        (L1024, T_1024, vec![0x01; 1023 / 8])
    };

    let padded_message = pad_message(message, l);
    let blocks = divide_into_blocks(&padded_message, l);

    let mut h = iv;

    for block in blocks {
        let m_vec = block.to_vec();
        h = t_xor_l(&xor_bytes(&h, &m_vec), t);
        h = xor_bytes(&h, &t_plus_l(&m_vec, t));
        h = xor_bytes(&h, &m_vec);
    }

    r_l_n(&t_xor_l(&h, t), n)
}

/// XORs two byte slices.
///
/// # Arguments
///
/// * `a` - A byte slice representing the first operand.
/// * `b` - A byte slice representing the second operand.
///
/// # Returns
///
/// * A `Vec<u8>` containing the result of the XOR operation.
fn xor_bytes(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b).map(|(x, y)| x ^ y).collect()
}

fn main() {
    let message = b"hello world";
    let hash_code_length = 256;

    let hash = kupyna_hash(message, hash_code_length);

    println!("Hash: {:?}", hash);
}

#[cfg(test)]
mod tests {
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
}

mod t_xor_plus;
mod tables;
#[cfg(test)]
mod tests;

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
    let padded_len = message.len() + (d / 8) + 13;
    let mut padded_message = vec![0x00; padded_len];

    // Copy the input message
    padded_message[0..message.len()].copy_from_slice(message);
    // Set the high bit
    padded_message[message.len()] = 0b10000000;

    // Convert the length to a byte array and copy it into the padded message
    let n_bytes = (n as u128).to_le_bytes(); // message length in little-endian
    padded_message[padded_len - 12..].copy_from_slice(&n_bytes[0..12]);

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
        h = t_xor_plus::t_xor_l(&xor_bytes(&h, &m_vec), t);
        h = xor_bytes(&h, &t_xor_plus::t_plus_l(&m_vec, t));
        h = xor_bytes(&h, &m_vec);
    }

    r_l_n(&t_xor_plus::t_xor_l(&h, t), n)
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

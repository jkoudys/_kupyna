
// TODO: this is still half a pile of LLM generated noise. It needs a lot of cleanup,
// especially around its bizarre tendency to tun all bits and bytes into iterators
// isntead of masking directly.

const L512: usize = 512;
const L1024: usize = 1024;
const T_512: usize = 10;
const T_1024: usize = 14;

fn pad_message(message: &[u8], l: usize) -> Vec<u8> {
    let mut padded_message = Vec::from(message);
    let n = message.len() * 8; // length in bits
    let d = ((-((n + 97) as isize) % (l as isize)) + l as isize) as usize;

    padded_message.push(0x80); // single '1' bit

    // Zero out the end pad.
    padded_message.resize(padded_message.len() + (d / 8), 0x00);

    let n_bytes = (n as u128).to_le_bytes(); // message length in little-endian
    padded_message.extend_from_slice(&n_bytes[0..12]);

    padded_message
}

fn divide_into_blocks(padded_message: &[u8], l: usize) -> Vec<&[u8]> {
    padded_message.chunks(l / 8).collect()
}

// TODO
fn t_xor_l(block: &[u8], _rounds: usize) -> Vec<u8> {
    // Implement the T+l transformation (placeholder)
    block.to_vec()
}

// TODO
fn t_plus_l(block: &[u8], _rounds: usize) -> Vec<u8> {
    // Implement the T+l transformation (placeholder)
    block.to_vec()
}

fn r_l_n(block: &[u8], n: usize) -> Vec<u8> {
    block[0..(n / 8)].to_vec()
}

fn kupyna_hash(message: &[u8], n: usize) -> Vec<u8> {
    let l = if (8..=256).contains(&n) { L512 } else { L1024 };
    let t = if l == L512 { T_512 } else { T_1024 };

    let iv = if l == L512 {
        vec![0x01; 510 / 8]
    } else {
        vec![0x01; 1023 / 8]
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

fn xor_bytes(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b).map(|(x, y)| x ^ y).collect()
}

fn main() {
    let message = b"hello world";
    let hash_code_length = 256;

    let hash = kupyna_hash(message, hash_code_length);

    println!("Hash: {:?}", hash);
}

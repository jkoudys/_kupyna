const ROWS: usize = 8;
const COLS: usize = 8; // For 512-bit state, adjust if needed

type Matrix = [[u8; COLS]; ROWS];

use crate::tables::SBOXES;

const TRANSFORM_VECTOR: [u8; 8] = [0x01, 0x01, 0x05, 0x01, 0x08, 0x06, 0x07, 0x04];

fn block_to_matrix(block: &[u8]) -> Matrix {
    let mut matrix = [[0u8; COLS]; ROWS];
    for i in 0..ROWS {
        for j in 0..COLS {
            matrix[i][j] = block[i * COLS + j];
        }
    }
    matrix
}

fn matrix_to_block(matrix: Matrix) -> Vec<u8> {
    let mut block = vec![0u8; ROWS * COLS];
    for i in 0..ROWS {
        for j in 0..COLS {
            block[i * COLS + j] = matrix[i][j];
        }
    }
    block
}

fn add_constant_xor(mut state: Matrix, round: usize) -> Matrix {
    for j in 0..COLS {
        let constant = ((j * 0x10) ^ round) as u8;
        state[j][0] ^= constant;
    }
    state
}

fn s_box_layer(mut state: Matrix) -> Matrix {
    for i in 0..ROWS {
        for j in 0..COLS {
            state[i][j] = SBOXES[i%4][state[i][j] as usize];
        }
    }
    state
}

fn rotate_rows(mut state: Matrix) -> Matrix {
    for i in 0..ROWS {
        state[i].rotate_right(i);
    }
    state
}

fn galois_multiply(mut a: u8, b: u8) -> u8 {
    let mut p = 0;
    let mut hi_bit_set;
    let mut b = b;
    for _ in 0..8 {
        if (b & 1) != 0 {
            p ^= a;
        }
        hi_bit_set = (a & 0x80) != 0;
        a <<= 1;
        if hi_bit_set {
            a ^= 0x1d; // Irreducible polynomial
        }
        b >>= 1;
    }
    p
}

fn linear_transform(mut state: Matrix) -> Matrix {
    let mut new_state = [[0u8; COLS]; ROWS];
    for j in 0..COLS {
        for i in 0..ROWS {
            new_state[i][j] = 0;
            for k in 0..ROWS {
                new_state[i][j] ^= galois_multiply(TRANSFORM_VECTOR[(i + k) % ROWS], state[k][j]);
            }
        }
    }
    new_state
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
pub fn t_xor_l(block: &[u8], rounds: usize) -> Vec<u8> {
    let mut state = block_to_matrix(block);
    for nu in 0..rounds {
        println!("round[{}].input: {:02x?}", nu, state);
        state = add_constant_xor(state, nu);
        println!("round[{}].add_c: {:02x?}", nu, state);
        state = s_box_layer(state);
        println!("round[{}].s_box: {:02x?}", nu, state);
        state = rotate_rows(state);
        println!("round[{}].s_byt: {:02x?}", nu, state);
        state = linear_transform(state);
        println!("round[{}].m_col: {:02x?}", nu, state);
    }
    matrix_to_block(state)
}

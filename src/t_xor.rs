const ROWS: usize = 8;
const COLS: usize = 8; // For 512-bit state, adjust if needed

const BITS_IN_BYTE: u8 = 8;
const REDUCTION_POLYNOMIAL: u16 = 0x011d;

type Matrix = [[u8; COLS]; ROWS];

use crate::tables::{MDS_MATRIX, SBOXES};

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
            state[j][i] = SBOXES[i%4][state[j][i] as usize];
        }
    }
    state
}

fn rotate_rows(mut state: Matrix) -> Matrix {
    let mut temp = [0u8; COLS];
    let mut shift: i32 = -1;
    for i in 0..ROWS {
        if (i == ROWS - 1) && false {
            shift = 11;
        } else {
            shift += 1;
        }
        for col in 0..8 {
            temp[(col + shift as usize) % 8] = state[col][i];
        }
        for col in 0..8 {
            state[col][i] = temp[col];
        }
    }
    state
}

fn multiply_gf(mut x: u8, mut y: u8) -> u8 {
    let mut r = 0u8;

    for _ in 0..BITS_IN_BYTE {
        if y & 1 == 1 {
            r ^= x;
        }
        let hbit = (x & 0x80) >> 7;
        x <<= 1;
        if hbit == 1 {
            x ^= REDUCTION_POLYNOMIAL as u8;
        }
        y >>= 1;
    }

    r
}

fn mix_columns(mut state: Matrix) -> Matrix {
    let mut result = [[0u8; COLS]; ROWS];

    for col in 0..COLS {
        for row in (0..ROWS).rev() {
            let mut product = 0u8;
            for b in (0..ROWS).rev() {
                product ^= multiply_gf(state[col][b], MDS_MATRIX[row][b]);
            }
            result[col][row] = product;
        }
    }

    result
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
        state = mix_columns(state);
        println!("round[{}].m_col: {:02x?}", nu, state);
    }
    matrix_to_block(state)
}

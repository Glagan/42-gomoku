use bitvec::{array::BitArray, bitarr, order::Lsb0};

pub fn generate_swap_anti_diagonal() -> [usize; 361] {
    let mut swap_vec_anti_diag: [usize; 361] = [0; 361];
    let mut index = 0;
    let mut length = 1;
    for i in 0..=18 {
        for j in 0..length {
            swap_vec_anti_diag[i + (j * (19 - 1))] = index;
            index += 1;
        }
        length += 1;
    }
    length = 18;
    for i in 0..18 {
        for j in 0..length {
            swap_vec_anti_diag[(37 + (i * 19)) + (j * (19 - 1))] = index;
            index += 1;
        }
        length -= 1;
    }
    swap_vec_anti_diag
}

pub fn display_swap_anti_diagonal(swap: &[usize; 361]) {
    println!("\n// Map an horizontal index to a anti-diagonal index");
    print!("pub static ANTI_DIAGONAL_TRANSPOSE: [usize; 361] = [");
    for index in swap {
        print!("{}, ", index);
    }
    println!("];");
}

pub fn display_swap_anti_diagonal_rev(swap: &[usize; 361]) {
    println!("\n// Reverse map to match a anti-diagonal index to an horizontal index");
    print!("pub static ANTI_DIAGONAL_TRANSPOSE_REV: [usize; 361] = [");
    for i in 0..swap.len() {
        let index = swap.iter().position(|&swapped| swapped == i).unwrap();
        print!("{}, ", index);
    }
    println!("];");
}

pub fn generate_anti_diagonal_slices(
    transpose: &[usize; 361],
    left: usize,
    right: usize,
) -> Vec<(usize, usize)> {
    let mut slices: Vec<(usize, usize)> = vec![(0, 0); 361];
    let mut offset = 0;
    let mut length = 1;
    let mut mov_length: i32 = 1;
    for d in 0..((19 * 2) - 1) {
        for j in 0..length {
            let horizontal_index = transpose
                .iter()
                .position(|&index| index == (offset + j))
                .unwrap();
            let left = if offset + j < left {
                0
            } else {
                (offset + j - left).max(offset)
            };
            let right = (offset + j + right).min(offset + length - 1);
            slices[horizontal_index] = (left, right);
        }
        offset += length;
        if d == 18 {
            mov_length = -1;
        }
        length = (length as i32 + mov_length) as usize;
    }
    slices
}

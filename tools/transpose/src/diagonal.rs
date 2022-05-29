use bitvec::{array::BitArray, bitarr, order::Lsb0};

// Start from the top right corner of the board (in 2D)
// -- and go from each diagonals from "top left" to "bottom right"
pub fn generate_swap_diagonal() -> [usize; 361] {
    let mut swap_vec_diag: [usize; 361] = [0; 361];
    let mut index = 0;
    let mut length = 1;
    for d in (0..19).rev() {
        for x in 0..length {
            swap_vec_diag[d + (x * (19 + 1))] = index;
            index += 1;
        }
        length += 1;
    }
    length = 18;
    for d in 1..=18 {
        for x in 0..length {
            swap_vec_diag[(d * 19) + (x * (19 + 1))] = index;
            index += 1;
        }
        length -= 1;
    }
    // println!("{:#?}", swap_vec_diag);
    swap_vec_diag
}

pub fn display_swap_diagonal(swap: &[usize; 361]) {
    println!("\n// Map an horizontal index to a diagonal index");
    print!("pub static DIAGONAL_TRANSPOSE: [usize; 361] = [");
    for index in swap {
        print!("{}, ", index);
    }
    println!("];");
}

pub fn display_swap_diagonal_rev(swap: &[usize; 361]) {
    println!("\n// Reverse map to match a diagonal index to an horizontal index");
    print!("pub static DIAGONAL_TRANSPOSE_REV: [usize; 361] = [");
    for i in 0..swap.len() {
        let index = swap.iter().position(|&swapped| swapped == i).unwrap();
        print!("{},", index);
    }
    println!("];");
}

pub fn generate_diagonal_slices(
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
                offset + j
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

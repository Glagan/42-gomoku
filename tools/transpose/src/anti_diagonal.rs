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
    println!("pub static ANTI_DIAGONAL_TRANSPOSE: [usize; 361] = [");
    for i in swap {
        println!("{},", i);
    }
    println!("];");
}

pub fn display_swap_anti_diagonal_rev(swap: &[usize; 361]) {
    println!("\n// Reverse map to match a anti-diagonal index to an horizontal index");
    println!("pub static ANTI_DIAGONAL_TRANSPOSE_REV: [usize; 361] = [");
    for i in 0..swap.len() {
        let index = swap.iter().position(|&swapped| swapped == i).unwrap();
        println!("{},", index);
    }
    println!("];");
}

pub fn display_anti_diagonal_window_five_slices(transpose: &[usize; 361]) {
    let mut window_anti_diagonal_slice_five: Vec<String> = vec!["".to_string(); 361];
    println!("\n// Slice for the largest anti-diagonal window of size 5 for an index");
    println!("pub static WINDOW_ANTI_DIAGONAL_SLICE_FIVE: [(usize, usize); 361] = [");
    let mut offset = 0;
    let mut length = 1;
    let mut mov_length: i32 = 1;
    for d in 0..((19 * 2) - 1) {
        for j in 0..length {
            let horizontal_index = transpose
                .iter()
                .position(|&index| index == (offset + j))
                .unwrap();
            let left = if offset + j < 2 {
                0
            } else {
                (offset + j - 2).max(offset)
            };
            let right = (offset + j + 2).min(offset + length - 1);
            window_anti_diagonal_slice_five[horizontal_index] = format!("({}, {})", left, right);
        }
        offset += length;
        if d == 18 {
            mov_length = -1;
        }
        length = (length as i32 + mov_length) as usize;
    }
    for window in window_anti_diagonal_slice_five {
        println!("{},", window);
    }
    println!("];");
}

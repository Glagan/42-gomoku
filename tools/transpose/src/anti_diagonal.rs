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
    println!("// Map an horizontal index to a anti-diagonal index");
    println!("pub static ANTI_DIAGONAL_TRANSPOSE: [usize; 361] = [");
    for i in swap {
        println!("{},", i);
    }
    println!("];");
}

pub fn display_anti_diagonal_window_five_slices(transpose: &[usize; 361]) {
    let mut window_anti_diagonal_slice_five: Vec<String> = vec!["".to_string(); 361];
    println!("\n// Slice for the largest anti-diagonal window of size 5 for an index");
    println!("pub static WINDOW_ANTI_DIAGONAL_SLICE_FIVE: [(usize, usize); 361] = [");
    let mut offset = 0;
    let mut length = 1;
    for i in 0..((19 * 2) - 1) {
        for j in 0..length {
            let horizontal_index = transpose
                .iter()
                .position(|&index| index == (offset + j))
                .unwrap();
            let left = if offset + j < 2 {
                offset + j
            } else {
                (offset + j - 2).max(offset)
            };
            let right = (offset + j + 2).min(offset + length - 1);
            window_anti_diagonal_slice_five[horizontal_index] = format!("({}, {})", left, right);
        }
        offset += length;
        if i < 18 {
            length += 1;
        } else {
            length -= 1;
        }
    }
    for window in window_anti_diagonal_slice_five {
        println!("{},", window);
    }
    println!("];");
}

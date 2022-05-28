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
    println!("// Map an horizontal index to a diagonal index");
    println!("pub static DIAGONAL_TRANSPOSE: [usize; 361] = [");
    for i in swap {
        println!("{},", i);
    }
    println!("];");
}

pub fn display_diagonal_window_five_slices(transpose: &[usize; 361]) {
    let mut window_diagonal_slice_five: Vec<String> = vec!["".to_string(); 361];
    println!("\n// Slice for the largest diagonal window of size 5 for an index");
    println!("pub static WINDOW_DIAGONAL_SLICE_FIVE: [(usize, usize); 361] = [");
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
            window_diagonal_slice_five[horizontal_index] = format!("({}, {})", left, right);
        }
        offset += length;
        if i < 18 {
            length += 1;
        } else {
            length -= 1;
        }
    }
    for window in window_diagonal_slice_five {
        println!("{},", window);
    }
    println!("];");
}

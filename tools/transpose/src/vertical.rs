use bitvec::{array::BitArray, bitarr, order::Lsb0};

pub fn generate_swap_vertical() -> [usize; 361] {
    let mut swap_vec_vertical: [usize; 361] = [0; 361];
    for i in 0..19 {
        for j in 0..19 {
            let index = i + (j * 19);
            swap_vec_vertical[index] = j + ((19 - i - 1) * 19);
        }
    }
    swap_vec_vertical
}

pub fn display_swap_vertical(swap: &[usize; 361]) {
    println!("// Map an horizontal index to a vertical index");
    println!("pub static VERTICAL_TRANSPOSE: [usize; 361] = [");
    for i in swap {
        println!("{},", i);
    }
    println!("];");
}

pub fn display_vertical_window_five_slices(transpose: &[usize; 361]) {
    println!("\n// Slice for the largest vertical window of size 5 for an index");
    println!("pub const WINDOW_VERTICAL_SLICE_FIVE: [(usize, usize); 361] = [");
    for y in 0..19 {
        for x in 0..19 {
            let index = transpose[x + y * 19];
            let swapped_y = index / 19;
            let left = if index < 2 {
                0
            } else {
                (index - 2).max(swapped_y * 19)
            };
            let right = (index + 2).min((swapped_y + 1) * 19 - 1);
            println!("({}, {}),", left, right);
        }
    }
    println!("];");
}

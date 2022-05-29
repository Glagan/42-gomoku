use bitvec::{array::BitArray, bitarr, order::Lsb0};

pub fn display_horizontal_window_five_slices() {
    println!("\n// Slice for the largest horizontal window of size 5 for an index");
    println!("pub static WINDOW_HORIZONTAL_SLICE_FIVE: [(usize, usize); 361] = [");
    for y in 0..19 {
        for x in 0..19 {
            let index = x + y * 19;
            let left = (index - 2).max(y * 19);
            let right = (index + 2).min((y + 1) * 19 - 1);
            println!("({}, {}),", left, right);
        }
    }
    println!("];");
}

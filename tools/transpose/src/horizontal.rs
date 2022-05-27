use bitvec::{array::BitArray, bitarr, order::Lsb0};

pub fn default_horizontal() -> BitArray<[usize; 6], Lsb0> {
    bitarr![0; 361]
}

pub fn draw_row(board: &mut BitArray<[usize; 6], Lsb0>, row: usize) {
    for i in (row * 19)..((row + 1) * 19) {
        board.set(i, true);
    }
}

pub fn draw_column(board: &mut BitArray<[usize; 6], Lsb0>, column: usize) {
    for i in 0..19 {
        board.set(column + i * 19, true);
    }
}

pub fn draw_diagonal(board: &mut BitArray<[usize; 6], Lsb0>) {
    for i in 0..19 {
        board.set(i * 19 + i, true);
    }
}

pub fn draw_anti_diagonal(board: &mut BitArray<[usize; 6], Lsb0>) {
    for i in 0..19 {
        board.set((18 - i) + (i * 19), true);
    }
}

pub fn display_horizontal_window_five_slices() {
    println!("\n// Slice for the largest horizontal window of size 5 for an index");
    println!("pub const WINDOW_HORIZONTAL_SLICE_FIVE: [(usize, usize); 361] = [");
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

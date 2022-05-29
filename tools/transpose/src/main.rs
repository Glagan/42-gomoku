use anti_diagonal::{
    display_anti_diagonal_window_five_slices, display_swap_anti_diagonal,
    display_swap_anti_diagonal_rev, generate_swap_anti_diagonal,
};
use bitvec::prelude::*;
use capture::display_capture_windows;
use diagonal::{
    display_diagonal_window_five_slices, display_swap_diagonal, display_swap_diagonal_rev,
    generate_swap_diagonal,
};
use horizontal::display_horizontal_window_five_slices;
use vertical::{
    display_swap_vertical, display_swap_vertical_rev, display_vertical_window_five_slices,
    generate_swap_vertical,
};

mod anti_diagonal;
mod capture;
mod diagonal;
mod horizontal;
mod vertical;

fn create_board() -> BitArray<[usize; 6], Lsb0> {
    bitarr![0; 361]
}

fn draw_row(board: &mut BitArray<[usize; 6], Lsb0>, row: usize, transpose: Option<&[usize; 361]>) {
    for i in (row * 19)..((row + 1) * 19) {
        if let Some(transpose) = transpose {
            board.set(transpose[i], true);
        } else {
            board.set(i, true);
        }
    }
}

fn draw_column(
    board: &mut BitArray<[usize; 6], Lsb0>,
    column: usize,
    transpose: Option<&[usize; 361]>,
) {
    for i in 0..19 {
        let index = column + i * 19;
        if let Some(transpose) = transpose {
            board.set(transpose[index], true);
        } else {
            board.set(index, true);
        }
    }
}

fn draw_diagonal(board: &mut BitArray<[usize; 6], Lsb0>, transpose: Option<&[usize; 361]>) {
    for i in 0..19 {
        let index = i * 19 + i;
        if let Some(transpose) = transpose {
            board.set(transpose[index], true);
        } else {
            board.set(index, true);
        }
    }
}

fn draw_anti_diagonal(board: &mut BitArray<[usize; 6], Lsb0>, transpose: Option<&[usize; 361]>) {
    for i in 0..19 {
        let index = (18 - i) + (i * 19);
        if let Some(transpose) = transpose {
            board.set(transpose[index], true);
        } else {
            board.set(index, true);
        }
    }
}

fn swap_board(
    board: &BitArray<[usize; 6], Lsb0>,
    transpose: &[usize; 361],
) -> BitArray<[usize; 6], Lsb0> {
    let mut swapped = bitarr![0; 361];
    for index in 0..(19 * 19) {
        let transposed_index = transpose[index];
        swapped.set(transposed_index, board[index]);
    }
    swapped
}

fn print_board(board: &BitArray<[usize; 6], Lsb0>) {
    for i in 0..19 {
        for j in 0..19 {
            print!("{}", if board[i * 19 + j] { 1 } else { 0 });
        }
        println!()
    }
    println!("---");
}

fn print_diagonals(board: &BitArray<[usize; 6], Lsb0>) {
    // let mut index = 0;
    // let mut index_2 = 0;
    let mut offset = 0;
    let mut length = 1;
    for i in 0..((19 * 2) - 1) {
        // for _ in 0..length {
        //     print!("{} ", swap_vec_anti_diag[index]);
        //     index += 1;
        // }
        // println!();
        // for _ in 0..length {
        //     print!("{} ", horizontal[swap_vec_anti_diag[index_2]]);
        //     index_2 += 1;
        // }
        // println!();
        println!("{}", board[offset..(offset + length)].to_string());
        offset += length;
        if i < 18 {
            length += 1;
        } else {
            length -= 1;
        }
    }
}

fn set_on_boards(
    index: usize,
    original_board: &mut BitArray<[usize; 6], Lsb0>,
    other_board: &mut BitArray<[usize; 6], Lsb0>,
    transpose: &[usize; 361],
) {
    original_board.set(index, true);
    other_board.set(transpose[index], true);
}

fn main() {
    //* Swap
    let transpose_vertical = generate_swap_vertical();
    display_swap_vertical(&transpose_vertical);
    display_swap_vertical_rev(&transpose_vertical);
    let transpose_diagonal = generate_swap_diagonal();
    display_swap_diagonal(&transpose_diagonal);
    display_swap_diagonal_rev(&transpose_diagonal);
    let transpose_anti_diagonal = generate_swap_anti_diagonal();
    display_swap_anti_diagonal(&transpose_anti_diagonal);
    display_swap_anti_diagonal_rev(&transpose_anti_diagonal);

    //* Slices
    display_horizontal_window_five_slices();
    display_vertical_window_five_slices(&transpose_vertical);
    display_diagonal_window_five_slices(&transpose_diagonal);
    display_anti_diagonal_window_five_slices(&transpose_anti_diagonal);

    //* Capture slices
    display_capture_windows();
}

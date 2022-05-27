use anti_diagonal::{
    display_anti_diagonal_window_five_slices, display_swap_anti_diagonal,
    generate_swap_anti_diagonal,
};
use bitvec::prelude::*;
use diagonal::{
    display_diagonal_window_five_slices, display_swap_diagonal, generate_swap_diagonal,
};
use horizontal::display_horizontal_window_five_slices;
use horizontal::{default_horizontal, draw_anti_diagonal, draw_column, draw_diagonal, draw_row};
use vertical::{
    display_swap_vertical, display_vertical_window_five_slices, generate_swap_vertical,
};

mod anti_diagonal;
mod diagonal;
mod horizontal;
mod vertical;

fn swap_board(
    board: &BitArray<[usize; 6], Lsb0>,
    transpose: &[usize; 361],
) -> BitArray<[usize; 6], Lsb0> {
    let mut swapped = bitarr![0; 361];
    for x in 0..19 {
        for y in 0..19 {
            let h = x + (y * 19);
            let v = transpose[h];
            swapped.set(h, board[v]);
        }
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

fn main() {
    // * Debug
    // let mut horizontal = default_horizontal();
    // draw_row(&mut horizontal, 0);
    // draw_column(&mut horizontal, 0);
    // draw_diagonal(&mut horizontal);
    // draw_anti_diagonal(&mut horizontal);
    // print_board(&horizontal);

    // Horizontal
    // display_horizontal_window_five_slices();

    // Vertical
    // let transpose = generate_swap_vertical();
    // display_swap_vertical(&transpose);
    // display_vertical_window_five_slices(&transpose);

    // Diagonal
    // let transpose = generate_swap_diagonal();
    // display_swap_diagonal(&transpose);
    // display_diagonal_window_five_slices(&transpose);

    // Anti-diagonal
    let transpose = generate_swap_anti_diagonal();
    display_swap_anti_diagonal(&transpose);
    // display_anti_diagonal_window_five_slices(&transpose);
}

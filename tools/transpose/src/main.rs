use anti_diagonal::{
    display_swap_anti_diagonal, display_swap_anti_diagonal_rev, generate_swap_anti_diagonal,
};
use bitvec::prelude::*;
use capture::display_capture_windows;
use diagonal::{display_swap_diagonal, display_swap_diagonal_rev, generate_swap_diagonal};
use horizontal::generate_horizontal_slices;
use vertical::{display_swap_vertical, display_swap_vertical_rev, generate_swap_vertical};

use crate::{
    anti_diagonal::generate_anti_diagonal_slices, diagonal::generate_diagonal_slices,
    vertical::generate_vertical_slices,
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

fn print_slice_2d_array(slices: &[Vec<(usize, usize)>; 4]) {
    println!("[");
    for slice_array in slices {
        print!("[");
        for (left, right) in slice_array {
            print!("({}, {}), ", left, right);
        }
        println!("], ");
    }
    println!("];");
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
    //* Four
    let slice_four = [
        generate_horizontal_slices(2, 1),
        generate_vertical_slices(&transpose_vertical, 2, 1),
        generate_diagonal_slices(&transpose_diagonal, 2, 1),
        generate_anti_diagonal_slices(&transpose_anti_diagonal, 2, 1),
    ];
    println!("\n// Slice for the largest vertical window of size 4 for an index (right)");
    println!("pub static WINDOW_SLICE_FOUR_RIGHT: [[(usize, usize); 361]; 4] =");
    print_slice_2d_array(&slice_four);
    let slice_four = [
        generate_horizontal_slices(1, 2),
        generate_vertical_slices(&transpose_vertical, 1, 2),
        generate_diagonal_slices(&transpose_diagonal, 1, 2),
        generate_anti_diagonal_slices(&transpose_anti_diagonal, 1, 2),
    ];
    println!("\n// Slice for the largest vertical window of size 4 for an index (left)");
    println!("pub static WINDOW_SLICE_FOUR_LEFT: [[(usize, usize); 361]; 4] =");
    print_slice_2d_array(&slice_four);
    //* Five
    let slice_five = [
        generate_horizontal_slices(1, 3),
        generate_vertical_slices(&transpose_vertical, 1, 3),
        generate_diagonal_slices(&transpose_diagonal, 1, 3),
        generate_anti_diagonal_slices(&transpose_anti_diagonal, 1, 3),
    ];
    println!("\n// Slice for the largest vertical window of size 5 for an index");
    println!("pub static WINDOW_SLICE_FIVE_1: [[(usize, usize); 361]; 4] =");
    print_slice_2d_array(&slice_five);
    let slice_five = [
        generate_horizontal_slices(2, 2),
        generate_vertical_slices(&transpose_vertical, 2, 2),
        generate_diagonal_slices(&transpose_diagonal, 2, 2),
        generate_anti_diagonal_slices(&transpose_anti_diagonal, 2, 2),
    ];
    println!("\n// Slice for the largest vertical window of size 5 for an index");
    println!("pub static WINDOW_SLICE_FIVE_2: [[(usize, usize); 361]; 4] =");
    print_slice_2d_array(&slice_five);
    let slice_five = [
        generate_horizontal_slices(3, 1),
        generate_vertical_slices(&transpose_vertical, 3, 1),
        generate_diagonal_slices(&transpose_diagonal, 3, 1),
        generate_anti_diagonal_slices(&transpose_anti_diagonal, 3, 1),
    ];
    println!("\n// Slice for the largest vertical window of size 5 for an index");
    println!("pub static WINDOW_SLICE_FIVE_3: [[(usize, usize); 361]; 4] =");
    print_slice_2d_array(&slice_five);
    //* Six
    let slice_six = [
        generate_horizontal_slices(1, 4),
        generate_vertical_slices(&transpose_vertical, 1, 4),
        generate_diagonal_slices(&transpose_diagonal, 1, 4),
        generate_anti_diagonal_slices(&transpose_anti_diagonal, 1, 4),
    ];
    println!("\n// Slice for the largest vertical window of size 6 for an index");
    println!("pub static WINDOW_SLICE_SIX_1: [[(usize, usize); 361]; 4] =");
    print_slice_2d_array(&slice_six);
    let slice_six = [
        generate_horizontal_slices(2, 3),
        generate_vertical_slices(&transpose_vertical, 2, 3),
        generate_diagonal_slices(&transpose_diagonal, 2, 3),
        generate_anti_diagonal_slices(&transpose_anti_diagonal, 2, 3),
    ];
    println!("\n// Slice for the largest vertical window of size 6 for an index");
    println!("pub static WINDOW_SLICE_SIX_2: [[(usize, usize); 361]; 4] =");
    print_slice_2d_array(&slice_six);
    let slice_six = [
        generate_horizontal_slices(3, 2),
        generate_vertical_slices(&transpose_vertical, 3, 2),
        generate_diagonal_slices(&transpose_diagonal, 3, 2),
        generate_anti_diagonal_slices(&transpose_anti_diagonal, 3, 2),
    ];
    println!("\n// Slice for the largest vertical window of size 6 for an index");
    println!("pub static WINDOW_SLICE_SIX_3: [[(usize, usize); 361]; 4] =");
    print_slice_2d_array(&slice_six);
    let slice_six = [
        generate_horizontal_slices(4, 1),
        generate_vertical_slices(&transpose_vertical, 4, 1),
        generate_diagonal_slices(&transpose_diagonal, 4, 1),
        generate_anti_diagonal_slices(&transpose_anti_diagonal, 4, 1),
    ];
    println!("\n// Slice for the largest vertical window of size 6 for an index");
    println!("pub static WINDOW_SLICE_SIX_4: [[(usize, usize); 361]; 4] =");
    print_slice_2d_array(&slice_six);

    //* Capture slices
    display_capture_windows();
}

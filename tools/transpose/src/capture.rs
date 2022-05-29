use crate::{
    anti_diagonal::generate_swap_anti_diagonal, diagonal::generate_swap_diagonal,
    vertical::generate_swap_vertical,
};

pub fn display_capture_windows() {
    const WINDOW_SIZE: usize = 3;
    println!("\n// Capture windows for each axis for a given index");
    println!("// -- [(horizontal left, right), (vertical left, right), (diagonal left, right), (anti-diagonal left, right)]");
    println!("pub static CAPTURE_SLICES: [((usize, usize), (usize, usize), (usize, usize), (usize, usize)); 361] = [");

    //* Horizontal
    let mut horizontal_slices: Vec<(usize, usize)> = vec![];
    for y in 0..19 {
        for x in 0..19 {
            let index = x + y * 19;
            let left = if index < WINDOW_SIZE {
                0
            } else {
                (index - WINDOW_SIZE).max(y * 19)
            };
            let right = (index + WINDOW_SIZE).min((y + 1) * 19 - 1);
            horizontal_slices.push((left, right));
        }
    }

    //* Vertical
    let mut vertical_slices: Vec<(usize, usize)> = vec![];
    let transpose = generate_swap_vertical();
    for y in 0..19 {
        for x in 0..19 {
            let index = transpose[x + y * 19];
            let swapped_y = index / 19;
            let left = if index < WINDOW_SIZE {
                0
            } else {
                (index - WINDOW_SIZE).max(swapped_y * 19)
            };
            let right = (index + WINDOW_SIZE).min((swapped_y + 1) * 19 - 1);
            vertical_slices.push((left, right));
        }
    }

    //* Diagonal
    let mut diagonal_slices: Vec<(usize, usize)> = vec![];
    let transpose = generate_swap_diagonal();
    for y in 0..19 {
        for x in 0..19 {
            let index = transpose[x + y * 19];
            let swapped_y = index / 19;
            let left = if index < WINDOW_SIZE {
                0
            } else {
                (index - WINDOW_SIZE).max(swapped_y * 19)
            };
            let right = (index + WINDOW_SIZE).min((swapped_y + 1) * 19 - 1);
            diagonal_slices.push((left, right));
        }
    }

    //* Anti-diagonal
    let mut anti_diagonal_slices: Vec<(usize, usize)> = vec![];
    let transpose = generate_swap_anti_diagonal();
    for y in 0..19 {
        for x in 0..19 {
            let index = transpose[x + y * 19];
            let swapped_y = index / 19;
            let left = if index < WINDOW_SIZE {
                0
            } else {
                (index - WINDOW_SIZE).max(swapped_y * 19)
            };
            let right = (index + WINDOW_SIZE).min((swapped_y + 1) * 19 - 1);
            anti_diagonal_slices.push((left, right));
        }
    }

    // Display all of them
    for i in 0..361 {
        println!(
            "(({}, {}), ({}, {}), ({}, {}), ({}, {})),",
            horizontal_slices[i].0,
            horizontal_slices[i].1,
            vertical_slices[i].0,
            vertical_slices[i].1,
            diagonal_slices[i].0,
            diagonal_slices[i].1,
            anti_diagonal_slices[i].0,
            anti_diagonal_slices[i].1,
        );
    }

    println!("];");
}

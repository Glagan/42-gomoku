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
    print!("pub static VERTICAL_TRANSPOSE: [usize; 361] = [");
    for index in swap {
        print!("{}, ", index);
    }
    println!("];");
}

pub fn display_swap_vertical_rev(swap: &[usize; 361]) {
    println!("\n// Reverse map to match a vertical index to an horizontal index");
    print!("pub static VERTICAL_TRANSPOSE_REV: [usize; 361] = [");
    for i in 0..swap.len() {
        let index = swap.iter().position(|&swapped| swapped == i).unwrap();
        print!("{}, ", index);
    }
    println!("];");
}

pub fn generate_vertical_slices(
    transpose: &[usize; 361],
    left: usize,
    right: usize,
) -> Vec<(usize, usize)> {
    // let size = (
    //     (size as f32 / 2.).ceil() as usize,
    //     (size as f32 / 2.).floor() as usize,
    // );
    let mut slices: Vec<(usize, usize)> = vec![];
    for y in 0..19 {
        for x in 0..19 {
            let index = transpose[x + y * 19];
            let swapped_y = index / 19;
            let left = if index < left {
                0
            } else {
                (index - left).max(swapped_y * 19)
            };
            let right = (index + right).min((swapped_y + 1) * 19 - 1);
            slices.push((left, right));
        }
    }
    slices
}

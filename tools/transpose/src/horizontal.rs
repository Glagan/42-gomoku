use bitvec::{array::BitArray, bitarr, order::Lsb0};

pub fn generate_horizontal_slices(left: usize, right: usize) -> Vec<(usize, usize)> {
    let mut slices: Vec<(usize, usize)> = vec![];
    for y in 0..19 {
        for x in 0..19 {
            let index = x + y * 19;
            let left = if index < left {
                0
            } else {
                (index - left).max(y * 19)
            };
            let right = (index + right).min((y + 1) * 19 - 1);
            slices.push((left, right));
        }
    }
    slices
}

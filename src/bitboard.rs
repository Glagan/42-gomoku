pub struct Transpose {
    pub vertical: [usize; 361],
    pub diagonal: [usize; 361],
    pub anti_diagonal: [usize; 361],
}

pub struct _BitBoard {
    pub transpose: Transpose,
    pub transpose_rev: Transpose,
    pub window_three: Vec<[[(usize, usize); 361]; 4]>,
    pub window_four: Vec<[[(usize, usize); 361]; 4]>,
    pub window_five: Vec<[[(usize, usize); 361]; 4]>,
    pub window_six: Vec<[[(usize, usize); 361]; 4]>,
}

// Generate tranpose for each directions
// -- and window slices for some sizes
impl _BitBoard {
    fn generate_tranpose_rev(transpose: &[usize; 361]) -> [usize; 361] {
        let mut new_transpose: [usize; 361] = [0; 361];
        for i in 0..transpose.len() {
            let index = transpose.iter().position(|&swapped| swapped == i).unwrap();
            new_transpose[i] = index;
        }
        new_transpose
    }

    fn generate_slice(
        vertical_transpose: &[usize; 361],
        diagonal_transpose: &[usize; 361],
        anti_diagonal_transpose: &[usize; 361],
        left: usize,
        right: usize,
    ) -> [[(usize, usize); 361]; 4] {
        [
            _BitBoard::generate_horizontal_slices(left, right),
            _BitBoard::generate_vertical_slices(&vertical_transpose, left, right),
            _BitBoard::generate_diagonal_slices(&diagonal_transpose, left, right),
            _BitBoard::generate_anti_diagonal_slices(&anti_diagonal_transpose, left, right),
        ]
    }

    // * Horizontal

    pub fn generate_horizontal_slices(left: usize, right: usize) -> [(usize, usize); 361] {
        let mut slices: [(usize, usize); 361] = [(0, 0); 361];
        for y in 0..19 {
            for x in 0..19 {
                let index = x + y * 19;
                let left = if index < left {
                    0
                } else {
                    (index - left).max(y * 19)
                };
                let right = (index + right).min((y + 1) * 19 - 1);
                slices[index] = (left, right);
            }
        }
        slices
    }

    // * Vertical

    fn generate_vertical_transpose() -> [usize; 361] {
        let mut transpose: [usize; 361] = [0; 361];
        for i in 0..19 {
            for j in 0..19 {
                let index = i + (j * 19);
                transpose[index] = j + ((19 - i - 1) * 19);
            }
        }
        transpose
    }

    fn generate_vertical_slices(
        transpose: &[usize; 361],
        left: usize,
        right: usize,
    ) -> [(usize, usize); 361] {
        let mut slices: [(usize, usize); 361] = [(0, 0); 361];
        let mut insert_index = 0;
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
                slices[insert_index] = (left, right);
                insert_index += 1;
            }
        }
        slices
    }

    // * Diagonal

    // Start from the top right corner of the board (in 2D)
    // -- and go from each diagonals from "top left" to "bottom right"
    pub fn generate_diagonal_transpose() -> [usize; 361] {
        let mut transpose: [usize; 361] = [0; 361];
        let mut index = 0;
        let mut length = 1;
        for d in (0..19).rev() {
            for x in 0..length {
                transpose[d + (x * (19 + 1))] = index;
                index += 1;
            }
            length += 1;
        }
        length = 18;
        for d in 1..=18 {
            for x in 0..length {
                transpose[(d * 19) + (x * (19 + 1))] = index;
                index += 1;
            }
            length -= 1;
        }
        transpose
    }

    fn generate_diagonal_slices(
        transpose: &[usize; 361],
        left: usize,
        right: usize,
    ) -> [(usize, usize); 361] {
        let mut slices: [(usize, usize); 361] = [(0, 0); 361];
        let mut offset = 0;
        let mut length = 1;
        let mut mov_length: i32 = 1;
        for d in 0..((19 * 2) - 1) {
            for j in 0..length {
                let horizontal_index = transpose
                    .iter()
                    .position(|&index| index == (offset + j))
                    .unwrap();
                let left = if offset + j < left {
                    offset + j
                } else {
                    (offset + j - left).max(offset)
                };
                let right = (offset + j + right).min(offset + length - 1);
                slices[horizontal_index] = (left, right);
            }
            offset += length;
            if d == 18 {
                mov_length = -1;
            }
            length = (length as i32 + mov_length) as usize;
        }
        slices
    }

    // * Anti-diagonal

    fn generate_anti_diagonal_transpose() -> [usize; 361] {
        let mut swap_vec_anti_diag: [usize; 361] = [0; 361];
        let mut index = 0;
        let mut length = 1;
        for i in 0..=18 {
            for j in 0..length {
                swap_vec_anti_diag[i + (j * (19 - 1))] = index;
                index += 1;
            }
            length += 1;
        }
        length = 18;
        for i in 0..18 {
            for j in 0..length {
                swap_vec_anti_diag[(37 + (i * 19)) + (j * (19 - 1))] = index;
                index += 1;
            }
            length -= 1;
        }
        swap_vec_anti_diag
    }

    fn generate_anti_diagonal_slices(
        transpose: &[usize; 361],
        left: usize,
        right: usize,
    ) -> [(usize, usize); 361] {
        let mut slices: [(usize, usize); 361] = [(0, 0); 361];
        let mut offset = 0;
        let mut length = 1;
        let mut mov_length: i32 = 1;
        for d in 0..((19 * 2) - 1) {
            for j in 0..length {
                let horizontal_index = transpose
                    .iter()
                    .position(|&index| index == (offset + j))
                    .unwrap();
                let left = if offset + j < left {
                    0
                } else {
                    (offset + j - left).max(offset)
                };
                let right = (offset + j + right).min(offset + length - 1);
                slices[horizontal_index] = (left, right);
            }
            offset += length;
            if d == 18 {
                mov_length = -1;
            }
            length = (length as i32 + mov_length) as usize;
        }
        slices
    }
}

impl Default for _BitBoard {
    fn default() -> Self {
        let vertical_transpose = _BitBoard::generate_vertical_transpose();
        let diagonal_transpose = _BitBoard::generate_diagonal_transpose();
        let anti_diagonal_transpose = _BitBoard::generate_anti_diagonal_transpose();
        _BitBoard {
            window_three: vec![
                _BitBoard::generate_slice(
                    &vertical_transpose,
                    &diagonal_transpose,
                    &anti_diagonal_transpose,
                    0,
                    2,
                ),
                _BitBoard::generate_slice(
                    &vertical_transpose,
                    &diagonal_transpose,
                    &anti_diagonal_transpose,
                    1,
                    1,
                ),
                _BitBoard::generate_slice(
                    &vertical_transpose,
                    &diagonal_transpose,
                    &anti_diagonal_transpose,
                    2,
                    0,
                ),
            ],
            window_four: vec![
                _BitBoard::generate_slice(
                    &vertical_transpose,
                    &diagonal_transpose,
                    &anti_diagonal_transpose,
                    0,
                    3,
                ),
                _BitBoard::generate_slice(
                    &vertical_transpose,
                    &diagonal_transpose,
                    &anti_diagonal_transpose,
                    1,
                    2,
                ),
                _BitBoard::generate_slice(
                    &vertical_transpose,
                    &diagonal_transpose,
                    &anti_diagonal_transpose,
                    2,
                    1,
                ),
                _BitBoard::generate_slice(
                    &vertical_transpose,
                    &diagonal_transpose,
                    &anti_diagonal_transpose,
                    3,
                    0,
                ),
            ],
            window_five: vec![
                _BitBoard::generate_slice(
                    &vertical_transpose,
                    &diagonal_transpose,
                    &anti_diagonal_transpose,
                    0,
                    4,
                ),
                _BitBoard::generate_slice(
                    &vertical_transpose,
                    &diagonal_transpose,
                    &anti_diagonal_transpose,
                    1,
                    3,
                ),
                _BitBoard::generate_slice(
                    &vertical_transpose,
                    &diagonal_transpose,
                    &anti_diagonal_transpose,
                    2,
                    2,
                ),
                _BitBoard::generate_slice(
                    &vertical_transpose,
                    &diagonal_transpose,
                    &anti_diagonal_transpose,
                    3,
                    1,
                ),
                _BitBoard::generate_slice(
                    &vertical_transpose,
                    &diagonal_transpose,
                    &anti_diagonal_transpose,
                    4,
                    0,
                ),
            ],
            window_six: vec![
                _BitBoard::generate_slice(
                    &vertical_transpose,
                    &diagonal_transpose,
                    &anti_diagonal_transpose,
                    0,
                    5,
                ),
                _BitBoard::generate_slice(
                    &vertical_transpose,
                    &diagonal_transpose,
                    &anti_diagonal_transpose,
                    1,
                    4,
                ),
                _BitBoard::generate_slice(
                    &vertical_transpose,
                    &diagonal_transpose,
                    &anti_diagonal_transpose,
                    2,
                    3,
                ),
                _BitBoard::generate_slice(
                    &vertical_transpose,
                    &diagonal_transpose,
                    &anti_diagonal_transpose,
                    3,
                    2,
                ),
                _BitBoard::generate_slice(
                    &vertical_transpose,
                    &diagonal_transpose,
                    &anti_diagonal_transpose,
                    4,
                    1,
                ),
                _BitBoard::generate_slice(
                    &vertical_transpose,
                    &diagonal_transpose,
                    &anti_diagonal_transpose,
                    5,
                    0,
                ),
            ],
            transpose_rev: Transpose {
                vertical: _BitBoard::generate_tranpose_rev(&vertical_transpose),
                diagonal: _BitBoard::generate_tranpose_rev(&diagonal_transpose),
                anti_diagonal: _BitBoard::generate_tranpose_rev(&anti_diagonal_transpose),
            },
            transpose: Transpose {
                vertical: vertical_transpose,
                diagonal: diagonal_transpose,
                anti_diagonal: anti_diagonal_transpose,
            },
        }
    }
}

lazy_static! {
    pub static ref BitBoard: _BitBoard = _BitBoard::default();
}

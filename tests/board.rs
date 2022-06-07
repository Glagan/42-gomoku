use std::collections::HashSet;

use gomoku::{
    board::{Board, Coordinates, Move},
    constants::{BOARD_SIZE, BOARD_SIZE_USIZE, DIRECTIONS},
    player::Player,
    rock::Rock,
    rules::RuleSet,
};

macro_rules! coord {
    ($x:expr, $y:expr) => {{
        use gomoku::board::Coordinates;
        Coordinates { x: $x, y: $y }
    }};
}

const CENTER: Coordinates = coord!(BOARD_SIZE / 2, BOARD_SIZE / 2);
const BORDER: i16 = BOARD_SIZE - 1;

#[test]
fn board_set_move() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: CENTER,
        },
    );
    let mut raw_board = [[Rock::None; BOARD_SIZE_USIZE]; BOARD_SIZE_USIZE];
    raw_board[BOARD_SIZE_USIZE / 2][BOARD_SIZE_USIZE / 2] = Rock::Black;
    assert_eq!(board.pieces, raw_board);
}

#[test]
fn open_intersections_empty_board() {
    let board = Board::default();
    assert_eq!(board.open_intersections(), vec![CENTER]);
}

#[test]
fn open_intersections_one_pawn() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: CENTER,
        },
    );
    assert_eq!(
        board.open_intersections(),
        DIRECTIONS
            .iter()
            .map(|(mov_x, mov_y)| { coord!(CENTER.x + mov_x, CENTER.y + mov_y) })
            .collect::<Vec<Coordinates>>(),
        "{:#?}",
        board.open_intersections()
    );
}

#[test]
fn board_save_black_moves_1() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: CENTER,
        },
    );
    let mut raw_rocks = HashSet::new();
    raw_rocks.insert(CENTER);
    assert_eq!(board.black.rocks, raw_rocks);
}

#[test]
fn board_save_black_moves_2() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: CENTER,
        },
    );
    let center_right = coord!(CENTER.x + 1, CENTER.y);
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: center_right,
        },
    );
    let mut raw_rocks = HashSet::new();
    raw_rocks.insert(CENTER);
    raw_rocks.insert(center_right);
    assert_eq!(board.black.rocks, raw_rocks);
}

#[test]
fn board_save_white_moves_1() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::White,
            coordinates: CENTER,
        },
    );
    let mut raw_rocks = HashSet::new();
    raw_rocks.insert(CENTER);
    assert_eq!(board.white.rocks, raw_rocks);
}

#[test]
fn board_save_white_moves_2() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::White,
            coordinates: CENTER,
        },
    );
    let center_right = coord!(CENTER.x + 1, CENTER.y);
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::White,
            coordinates: center_right,
        },
    );
    let mut raw_rocks = HashSet::new();
    raw_rocks.insert(CENTER);
    raw_rocks.insert(center_right);
    assert_eq!(board.white.rocks, raw_rocks);
}

#[test]
fn five_in_a_row_center() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: CENTER,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(CENTER.x + 1, CENTER.y),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(CENTER.x + 2, CENTER.y),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(CENTER.x + 3, CENTER.y),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(CENTER.x + 4, CENTER.y),
        },
    );
    assert!(board.has_five_in_a_row(Player::Black));
}

#[test]
fn five_in_a_row_top_left_horizontal() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(0, 0),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(1, 0),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(2, 0),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(3, 0),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(4, 0),
        },
    );
    assert!(board.has_five_in_a_row(Player::Black));
}

#[test]
fn five_in_a_row_top_left_diagonal() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(0, 0),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(1, 1),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(2, 2),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(3, 3),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(4, 4),
        },
    );
    assert!(board.has_five_in_a_row(Player::Black));
}

#[test]
fn five_in_a_row_top_left_vertical() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(0, 0),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(0, 1),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(0, 2),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(0, 3),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(0, 4),
        },
    );
    assert!(board.has_five_in_a_row(Player::Black));
}

#[test]
fn five_in_a_row_top_right_horizontal() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(BORDER, 0),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(BORDER - 1, 0),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(BORDER - 2, 0),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(BORDER - 3, 0),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(BORDER - 4, 0),
        },
    );
    assert!(board.has_five_in_a_row(Player::Black));
}

#[test]
fn five_in_a_row_top_right_diagonal() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(BORDER, 0),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(BORDER - 1, 1),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(BORDER - 2, 2),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(BORDER - 3, 3),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(BORDER - 4, 4),
        },
    );
    assert!(board.has_five_in_a_row(Player::Black));
}

#[test]
fn five_in_a_row_top_right_vertical() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(BORDER, 0),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(BORDER, 1),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(BORDER, 2),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(BORDER, 3),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(BORDER, 4),
        },
    );
    assert!(board.has_five_in_a_row(Player::Black));
}

#[test]
fn five_in_a_row_bottom_left_horizontal() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(0, BORDER),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(1, BORDER),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(2, BORDER),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(3, BORDER),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(4, BORDER),
        },
    );
    assert!(board.has_five_in_a_row(Player::Black));
}

#[test]
fn five_in_a_row_bottom_left_diagonal() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(0, BORDER),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(1, BORDER - 1),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(2, BORDER - 2),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(3, BORDER - 3),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(4, BORDER - 4),
        },
    );
    assert!(board.has_five_in_a_row(Player::Black));
}

#[test]
fn five_in_a_row_bottom_left_vertical() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(0, BORDER),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(0, BORDER - 1),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(0, BORDER - 2),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(0, BORDER - 3),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(0, BORDER - 4),
        },
    );
    assert!(board.has_five_in_a_row(Player::Black));
}

#[test]
fn five_in_a_row_bottom_right_horizontal() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(BORDER, BORDER),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(BORDER - 1, BORDER),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(BORDER - 2, BORDER),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(BORDER - 3, BORDER),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(BORDER - 4, BORDER),
        },
    );
    assert!(board.has_five_in_a_row(Player::Black));
}

#[test]
fn five_in_a_row_bottom_right_diagonal() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(BORDER, BORDER),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(BORDER - 1, BORDER - 1),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(BORDER - 2, BORDER - 2),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(BORDER - 3, BORDER - 3),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(BORDER - 4, BORDER - 4),
        },
    );
    assert!(board.has_five_in_a_row(Player::Black));
}

#[test]
fn five_in_a_row_bottom_right_vertical() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(BORDER, BORDER),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(BORDER, BORDER - 1),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(BORDER, BORDER - 2),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(BORDER, BORDER - 3),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(BORDER, BORDER - 4),
        },
    );
    assert!(board.has_five_in_a_row(Player::Black));
}

#[test]
fn four_in_a_row_no_match() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: CENTER,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(CENTER.x - 1, CENTER.y),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(CENTER.x - 2, CENTER.y),
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: coord!(CENTER.x + 1, CENTER.y),
        },
    );
    assert!(!board.has_five_in_a_row(Player::Black));
}

// Create a board with all possible blocked recursive capture patterns
// 2 0 0 2 0 0 2
// 0 x 0 x 0 x 0
// 0 0 1 1 1 0 0
// 2 x 1 2 1 x 2
// 0 0 1 1 1 0 0
// 0 x 0 x 0 x 0
// 2 0 0 2 0 0 2
fn create_board_with_recursive_capture_patterns(
    x_offset: i16,
    y_offset: i16,
) -> (Board, Vec<(i16, i16)>) {
    let rules = RuleSet::default();
    let mut board = Board::default();

    let capture_rocks = vec![
        (0, 0),
        (3, 0),
        (6, 0),
        (0, 3),
        (3, 3),
        (6, 3),
        (0, 6),
        (3, 6),
        (6, 6),
    ];
    let blocked_rocks = vec![
        (2, 2),
        (3, 2),
        (4, 2),
        (2, 3),
        (4, 3),
        (2, 4),
        (3, 4),
        (4, 4),
    ];
    let move_rocks = vec![
        (1 + x_offset, 1 + y_offset),
        (3 + x_offset, 1 + y_offset),
        (5 + x_offset, 1 + y_offset),
        (1 + x_offset, 3 + y_offset),
        (5 + x_offset, 3 + y_offset),
        (1 + x_offset, 5 + y_offset),
        (3 + x_offset, 5 + y_offset),
        (5 + x_offset, 5 + y_offset),
    ];

    for rock in capture_rocks {
        board.set_move(
            &rules,
            &Move {
                player: Player::Black,
                coordinates: coord!(rock.0 + x_offset, rock.1 + y_offset),
            },
        );
    }

    for rock in blocked_rocks {
        board.set_move(
            &rules,
            &Move {
                player: Player::White,
                coordinates: coord!(rock.0 + x_offset, rock.1 + y_offset),
            },
        );
    }

    (board, move_rocks)
}

fn assert_recursive_capture_are_illegal(board: Board, moves: Vec<(i16, i16)>) {
    let rules = RuleSet::default();
    for movement in moves {
        assert!(
            !board.is_move_legal(
                &rules,
                &Move {
                    player: Player::White,
                    coordinates: coord!(movement.0, movement.1),
                },
            ),
            "Expected {:#?} to be illegal",
            movement
        );
    }
}

#[test]
fn recursive_capture_all_directions_top_left() {
    let (board, moves) = create_board_with_recursive_capture_patterns(0, 0);
    assert_recursive_capture_are_illegal(board, moves);
}

#[test]
fn recursive_capture_all_directions_top_right() {
    let (board, moves) = create_board_with_recursive_capture_patterns(BOARD_SIZE - 7, 0);
    assert_recursive_capture_are_illegal(board, moves);
}

#[test]
fn recursive_capture_all_directions_center() {
    let (board, moves) =
        create_board_with_recursive_capture_patterns(BOARD_SIZE / 2, BOARD_SIZE / 2);
    assert_recursive_capture_are_illegal(board, moves);
}

#[test]
fn recursive_capture_all_directions_bottom_left() {
    let (board, moves) = create_board_with_recursive_capture_patterns(0, BOARD_SIZE - 7);
    assert_recursive_capture_are_illegal(board, moves);
}

#[test]
fn recursive_capture_all_directions_bottom_right() {
    let (board, moves) =
        create_board_with_recursive_capture_patterns(BOARD_SIZE - 7, BOARD_SIZE - 7);
    assert_recursive_capture_are_illegal(board, moves);
}

#[test]
fn free_three_1_detected_horizontal() {
    let rules = RuleSet::default();
    let mut board = Board::default();

    board.set_move(
        &rules,
        &Move {
            player: Player::Black,
            coordinates: coord!(1, 0),
        },
    );
    let second_move = Move {
        player: Player::Black,
        coordinates: coord!(2, 0),
    };
    board.set_move(&rules, &second_move);
    assert_eq!(board.move_create_free_three_direct_pattern(&second_move), 0);
    assert_eq!(
        board.move_create_free_three_secondary_pattern(&second_move),
        0
    );

    let free_three_move = Move {
        player: Player::Black,
        coordinates: coord!(3, 0),
    };
    assert_eq!(
        board.move_create_free_three_direct_pattern(&free_three_move),
        1
    );
    assert_eq!(
        board.move_create_free_three_secondary_pattern(&free_three_move),
        0
    );
}

#[test]
fn free_three_1_detected_vertical() {
    let rules = RuleSet::default();
    let mut board = Board::default();

    board.set_move(
        &rules,
        &Move {
            player: Player::Black,
            coordinates: coord!(0, 1),
        },
    );
    let second_move = Move {
        player: Player::Black,
        coordinates: coord!(0, 2),
    };
    board.set_move(&rules, &second_move);
    assert_eq!(board.move_create_free_three_direct_pattern(&second_move), 0);
    assert_eq!(
        board.move_create_free_three_secondary_pattern(&second_move),
        0
    );

    let free_three_move = Move {
        player: Player::Black,
        coordinates: coord!(0, 3),
    };
    assert_eq!(
        board.move_create_free_three_direct_pattern(&free_three_move),
        1
    );
    assert_eq!(
        board.move_create_free_three_secondary_pattern(&free_three_move),
        0
    );
}

#[test]
fn free_three_1_detected_diagonal_left() {
    let rules = RuleSet::default();
    let mut board = Board::default();

    board.set_move(
        &rules,
        &Move {
            player: Player::Black,
            coordinates: coord!(1, 1),
        },
    );
    let second_move = Move {
        player: Player::Black,
        coordinates: coord!(2, 2),
    };
    board.set_move(&rules, &second_move);
    assert_eq!(board.move_create_free_three_direct_pattern(&second_move), 0);
    assert_eq!(
        board.move_create_free_three_secondary_pattern(&second_move),
        0
    );

    let free_three_move = Move {
        player: Player::Black,
        coordinates: coord!(3, 3),
    };
    assert_eq!(
        board.move_create_free_three_direct_pattern(&free_three_move),
        1
    );
    assert_eq!(
        board.move_create_free_three_secondary_pattern(&free_three_move),
        0
    );
}

#[test]
fn free_three_1_detected_diagonal_right() {
    let rules = RuleSet::default();
    let mut board = Board::default();

    board.set_move(
        &rules,
        &Move {
            player: Player::Black,
            coordinates: coord!(BORDER - 1, BORDER - 1),
        },
    );
    let second_move = Move {
        player: Player::Black,
        coordinates: coord!(BORDER - 2, BORDER - 2),
    };
    board.set_move(&rules, &second_move);
    assert_eq!(board.move_create_free_three_direct_pattern(&second_move), 0);
    assert_eq!(
        board.move_create_free_three_secondary_pattern(&second_move),
        0
    );

    let free_three_move = Move {
        player: Player::Black,
        coordinates: coord!(BORDER - 3, BORDER - 3),
    };
    assert_eq!(
        board.move_create_free_three_direct_pattern(&free_three_move),
        1
    );
    assert_eq!(
        board.move_create_free_three_secondary_pattern(&free_three_move),
        0
    );
}

#[test]
fn free_three_2_detected_horizontal() {
    let rules = RuleSet::default();
    let mut board = Board::default();

    board.set_move(
        &rules,
        &Move {
            player: Player::Black,
            coordinates: coord!(1, 0),
        },
    );
    let second_move = Move {
        player: Player::Black,
        coordinates: coord!(2, 0),
    };
    board.set_move(&rules, &second_move);
    assert_eq!(board.move_create_free_three_direct_pattern(&second_move), 0);
    assert_eq!(
        board.move_create_free_three_secondary_pattern(&second_move),
        0
    );

    let free_three_move = Move {
        player: Player::Black,
        coordinates: coord!(4, 0),
    };
    assert_eq!(
        board.move_create_free_three_direct_pattern(&free_three_move),
        0
    );
    assert_eq!(
        board.move_create_free_three_secondary_pattern(&free_three_move),
        1
    );
}

#[test]
fn free_three_2_detected_vertical() {
    let rules = RuleSet::default();
    let mut board = Board::default();

    board.set_move(
        &rules,
        &Move {
            player: Player::Black,
            coordinates: coord!(0, 1),
        },
    );
    let second_move = Move {
        player: Player::Black,
        coordinates: coord!(0, 2),
    };
    board.set_move(&rules, &second_move);
    assert_eq!(board.move_create_free_three_direct_pattern(&second_move), 0);
    assert_eq!(
        board.move_create_free_three_secondary_pattern(&second_move),
        0
    );

    let free_three_move = Move {
        player: Player::Black,
        coordinates: coord!(0, 4),
    };
    assert_eq!(
        board.move_create_free_three_direct_pattern(&free_three_move),
        0
    );
    assert_eq!(
        board.move_create_free_three_secondary_pattern(&free_three_move),
        1
    );
}

#[test]
fn free_three_2_detected_diagonal_left() {
    let rules = RuleSet::default();
    let mut board = Board::default();

    board.set_move(
        &rules,
        &Move {
            player: Player::Black,
            coordinates: coord!(1, 1),
        },
    );
    let second_move = Move {
        player: Player::Black,
        coordinates: coord!(2, 2),
    };
    board.set_move(&rules, &second_move);
    assert_eq!(board.move_create_free_three_direct_pattern(&second_move), 0);
    assert_eq!(
        board.move_create_free_three_secondary_pattern(&second_move),
        0
    );

    let free_three_move = Move {
        player: Player::Black,
        coordinates: coord!(4, 4),
    };
    assert_eq!(
        board.move_create_free_three_direct_pattern(&free_three_move),
        0
    );
    assert_eq!(
        board.move_create_free_three_secondary_pattern(&free_three_move),
        1
    );
}

#[test]
fn free_three_2_detected_diagonal_right() {
    let rules = RuleSet::default();
    let mut board = Board::default();

    board.set_move(
        &rules,
        &Move {
            player: Player::Black,
            coordinates: coord!(BORDER - 1, BORDER - 1),
        },
    );
    let second_move = Move {
        player: Player::Black,
        coordinates: coord!(BORDER - 2, BORDER - 2),
    };
    board.set_move(&rules, &second_move);
    assert_eq!(board.move_create_free_three_direct_pattern(&second_move), 0);
    assert_eq!(
        board.move_create_free_three_secondary_pattern(&second_move),
        0
    );

    let free_three_move = Move {
        player: Player::Black,
        coordinates: coord!(BORDER - 4, BORDER - 4),
    };
    assert_eq!(
        board.move_create_free_three_direct_pattern(&free_three_move),
        0
    );
    assert_eq!(
        board.move_create_free_three_secondary_pattern(&free_three_move),
        1
    );
}

#[test]
fn captured_five_in_a_row_horizontal_1() {
    let rules = RuleSet::default();
    let mut board = Board::default();

    let blocked_rocks = vec![
        (1, 2),
        (2, 2),
        (3, 2),
        (4, 2),
        (5, 2),
        (1, 1), // Rock that allow capture
    ];
    let capture_rocks = vec![(1, 0)];

    for rock in blocked_rocks {
        board.set_move(
            &rules,
            &Move {
                player: Player::Black,
                coordinates: coord!(rock.0, rock.1),
            },
        );
    }

    for rock in capture_rocks {
        board.set_move(
            &rules,
            &Move {
                player: Player::White,
                coordinates: coord!(rock.0, rock.1),
            },
        );
    }

    assert!(board.has_five_in_a_row(Player::Black));
    assert!(!board.has_uncaptured_five_in_a_row(&RuleSet::default(), Player::Black));
    assert!(!board.is_winning(&rules, Player::Black));
}

#[test]
fn captured_five_in_a_row_horizontal_2() {
    let rules = RuleSet::default();
    let mut board = Board::default();

    let blocked_rocks = vec![
        (1, 2),
        (2, 2),
        (3, 2),
        (4, 2),
        (5, 2),
        (1, 1), // Rock that allow capture
    ];
    let capture_rocks = vec![(1, 3)];

    for rock in blocked_rocks {
        board.set_move(
            &rules,
            &Move {
                player: Player::Black,
                coordinates: coord!(rock.0, rock.1),
            },
        );
    }

    for rock in capture_rocks {
        board.set_move(
            &rules,
            &Move {
                player: Player::White,
                coordinates: coord!(rock.0, rock.1),
            },
        );
    }

    assert!(board.has_five_in_a_row(Player::Black));
    assert!(!board.has_uncaptured_five_in_a_row(&RuleSet::default(), Player::Black));
    assert!(!board.is_winning(&rules, Player::Black));
}

#[test]
fn captured_five_in_a_row_vertical_1() {
    let rules = RuleSet::default();
    let mut board = Board::default();

    let blocked_rocks = vec![
        (2, 1),
        (2, 2),
        (2, 3),
        (2, 4),
        (2, 5),
        (1, 1), // Rock that allow capture
    ];
    let capture_rocks = vec![(3, 1)];

    for rock in blocked_rocks {
        board.set_move(
            &rules,
            &Move {
                player: Player::Black,
                coordinates: coord!(rock.0, rock.1),
            },
        );
    }

    for rock in capture_rocks {
        board.set_move(
            &rules,
            &Move {
                player: Player::White,
                coordinates: coord!(rock.0, rock.1),
            },
        );
    }

    assert!(board.has_five_in_a_row(Player::Black));
    assert!(!board.has_uncaptured_five_in_a_row(&RuleSet::default(), Player::Black));
    assert!(!board.is_winning(&rules, Player::Black));
}

#[test]
fn captured_five_in_a_row_vertical_2() {
    let rules = RuleSet::default();
    let mut board = Board::default();

    let blocked_rocks = vec![
        (2, 1),
        (2, 2),
        (2, 3),
        (2, 4),
        (2, 5),
        (1, 1), // Rock that allow capture
    ];
    let capture_rocks = vec![(0, 1)];

    for rock in blocked_rocks {
        board.set_move(
            &rules,
            &Move {
                player: Player::Black,
                coordinates: coord!(rock.0, rock.1),
            },
        );
    }

    for rock in capture_rocks {
        board.set_move(
            &rules,
            &Move {
                player: Player::White,
                coordinates: coord!(rock.0, rock.1),
            },
        );
    }

    assert!(board.has_five_in_a_row(Player::Black));
    assert!(!board.has_uncaptured_five_in_a_row(&RuleSet::default(), Player::Black));
    assert!(!board.is_winning(&rules, Player::Black));
}

#[test]
fn captured_five_in_a_row_diagonal_1() {
    let rules = RuleSet::default();
    let mut board = Board::default();

    let blocked_rocks = vec![
        (2, 2),
        (3, 3),
        (4, 4),
        (5, 5),
        (6, 6),
        (1, 3), // Rock that allow capture
    ];
    let capture_rocks = vec![(3, 1)];

    for rock in blocked_rocks {
        board.set_move(
            &rules,
            &Move {
                player: Player::Black,
                coordinates: coord!(rock.0, rock.1),
            },
        );
    }

    for rock in capture_rocks {
        board.set_move(
            &rules,
            &Move {
                player: Player::White,
                coordinates: coord!(rock.0, rock.1),
            },
        );
    }

    assert!(board.has_five_in_a_row(Player::Black));
    assert!(!board.has_uncaptured_five_in_a_row(&RuleSet::default(), Player::Black));
    assert!(!board.is_winning(&rules, Player::Black));
}

#[test]
fn captured_five_in_a_row_diagonal_2() {
    let rules = RuleSet::default();
    let mut board = Board::default();

    let blocked_rocks = vec![
        (2, 2),
        (3, 3),
        (4, 4),
        (5, 5),
        (6, 6),
        (1, 3), // Rock that allow capture
    ];
    let capture_rocks = vec![(0, 4)];

    for rock in blocked_rocks {
        board.set_move(
            &rules,
            &Move {
                player: Player::Black,
                coordinates: coord!(rock.0, rock.1),
            },
        );
    }

    for rock in capture_rocks {
        board.set_move(
            &rules,
            &Move {
                player: Player::White,
                coordinates: coord!(rock.0, rock.1),
            },
        );
    }

    assert!(board.has_five_in_a_row(Player::Black));
    assert!(!board.has_uncaptured_five_in_a_row(&RuleSet::default(), Player::Black));
    assert!(!board.is_winning(&rules, Player::Black));
}

#[test]
fn captured_five_in_a_row_diagonal_3() {
    let rules = RuleSet::default();
    let mut board = Board::default();

    let blocked_rocks = vec![
        (8, 8),
        (7, 9),
        (6, 10),
        (5, 11),
        (4, 12),
        (7, 7), // Rock that allow capture
    ];
    let capture_rocks = vec![(6, 6)];

    for rock in blocked_rocks {
        board.set_move(
            &rules,
            &Move {
                player: Player::Black,
                coordinates: coord!(rock.0, rock.1),
            },
        );
    }

    for rock in capture_rocks {
        board.set_move(
            &rules,
            &Move {
                player: Player::White,
                coordinates: coord!(rock.0, rock.1),
            },
        );
    }

    assert!(board.has_five_in_a_row(Player::Black));
    assert!(!board.has_uncaptured_five_in_a_row(&RuleSet::default(), Player::Black));
    assert!(!board.is_winning(&rules, Player::Black));
}

#[test]
fn captured_five_in_a_row_diagonal_4() {
    let rules = RuleSet::default();
    let mut board = Board::default();

    let blocked_rocks = vec![
        (8, 8),
        (7, 9),
        (6, 10),
        (5, 11),
        (4, 12),
        (7, 7), // Rock that allow capture
    ];
    let capture_rocks = vec![(9, 9)];

    for rock in blocked_rocks {
        board.set_move(
            &rules,
            &Move {
                player: Player::Black,
                coordinates: coord!(rock.0, rock.1),
            },
        );
    }

    for rock in capture_rocks {
        board.set_move(
            &rules,
            &Move {
                player: Player::White,
                coordinates: coord!(rock.0, rock.1),
            },
        );
    }

    assert!(board.has_five_in_a_row(Player::Black));
    assert!(!board.has_uncaptured_five_in_a_row(&RuleSet::default(), Player::Black));
    assert!(!board.is_winning(&rules, Player::Black));
}

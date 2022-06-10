use std::collections::BTreeSet;

use gomoku::{
    board::{Board, Coordinates, Move},
    constants::{BOARD_SIZE, BOARD_SIZE_USIZE, DIRECTIONS},
    player::Player,
    rock::Rock,
    rules::RuleSet,
};

macro_rules! coord {
    ($x: expr, $y: expr) => {{
        use gomoku::board::Coordinates;
        Coordinates { x: $x, y: $y }
    }};
}

macro_rules! set_many {
    (mut $board: expr, $player: expr, $( $coordinate: expr ),*) => {{
        use gomoku::board::Move;
        $(
            #[allow(unused_assignments)]
            {
                $board.set_move(&RuleSet::default(), &Move {
                    player:  $player,
                    coordinates: coord!($coordinate.0, $coordinate.1)
                });
            }
        )*
    }};
}

const CENTER: Coordinates = coord!(BOARD_SIZE / 2, BOARD_SIZE / 2);
const BORDER: i16 = BOARD_SIZE - 1;

// * Set move

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
fn player_boards_set_move_black() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            coordinates: CENTER,
        },
    );
    assert_eq!(
        board.get_for_player(CENTER.x, CENTER.y, Player::Black),
        false
    );
    assert_eq!(
        board.get_for_player(CENTER.x, CENTER.y, Player::White),
        true
    );
}

#[test]
fn player_boards_set_move_white() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::White,
            coordinates: CENTER,
        },
    );
    assert_eq!(
        board.get_for_player(CENTER.x, CENTER.y, Player::White),
        false
    );
    assert_eq!(
        board.get_for_player(CENTER.x, CENTER.y, Player::Black),
        true
    );
}

#[test]
fn player_boards_set_move_both() {
    let mut board = Board::default();
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
            player: Player::White,
            coordinates: coord!(CENTER.x + 1, CENTER.y),
        },
    );
    assert_eq!(
        board.get_for_player(CENTER.x - 1, CENTER.y, Player::Black),
        false
    );
    assert_eq!(
        board.get_for_player(CENTER.x - 1, CENTER.y, Player::White),
        true
    );
    assert_eq!(
        board.get_for_player(CENTER.x, CENTER.y, Player::Black),
        true
    );
    assert_eq!(
        board.get_for_player(CENTER.x, CENTER.y, Player::White),
        true
    );
    assert_eq!(
        board.get_for_player(CENTER.x + 1, CENTER.y, Player::Black),
        true
    );
    assert_eq!(
        board.get_for_player(CENTER.x + 1, CENTER.y, Player::White),
        false
    );
}

// * Undo moves

#[test]
fn undo_single_move() {
    let mut board = Board::default();
    let movement = Move {
        player: Player::Black,
        coordinates: coord!(0, 0),
    };
    board.set_move(&RuleSet::default(), &movement);
    assert_eq!(board.get(0, 0), Rock::Black);
    assert_eq!(board.moves_restore, vec![vec![]]);
    assert_eq!(board.black.rocks, BTreeSet::from([coord!(0, 0)]));
    assert_eq!(board.all_rocks, BTreeSet::from([coord!(0, 0)]));

    board.undo_move(&RuleSet::default(), &movement);
    assert_eq!(board.get(0, 0), Rock::None);
    assert!(board.moves_restore.is_empty());
    assert_eq!(board.black.rocks, BTreeSet::new());
    assert_eq!(board.all_rocks, BTreeSet::new());
}

#[test]
fn undo_two_moves() {
    let mut board = Board::default();
    let movement_1 = Move {
        player: Player::Black,
        coordinates: coord!(0, 0),
    };
    board.set_move(&RuleSet::default(), &movement_1);
    assert_eq!(board.get(0, 0), Rock::Black);
    assert_eq!(board.get(1, 0), Rock::None);
    assert_eq!(board.moves_restore, vec![vec![]]);
    assert_eq!(board.black.rocks, BTreeSet::from([coord!(0, 0)]));
    assert_eq!(board.white.rocks, BTreeSet::new());
    assert_eq!(board.all_rocks, BTreeSet::from([coord!(0, 0)]));

    let movement_2 = Move {
        player: Player::White,
        coordinates: coord!(1, 0),
    };
    board.set_move(&RuleSet::default(), &movement_2);
    assert_eq!(board.get(0, 0), Rock::Black);
    assert_eq!(board.get(1, 0), Rock::White);
    assert_eq!(board.moves_restore, vec![vec![], vec![]]);
    assert_eq!(board.black.rocks, BTreeSet::from([coord!(0, 0)]));
    assert_eq!(board.white.rocks, BTreeSet::from([coord!(1, 0)]));
    assert_eq!(
        board.all_rocks,
        BTreeSet::from([coord!(0, 0), coord!(1, 0)])
    );

    board.undo_move(&RuleSet::default(), &movement_2);
    assert_eq!(board.get(0, 0), Rock::Black);
    assert_eq!(board.get(1, 0), Rock::None);
    assert_eq!(board.moves_restore, vec![vec![]]);
    assert_eq!(board.black.rocks, BTreeSet::from([coord!(0, 0)]));
    assert_eq!(board.white.rocks, BTreeSet::new());
    assert_eq!(board.all_rocks, BTreeSet::from([coord!(0, 0)]));

    board.undo_move(&RuleSet::default(), &movement_1);
    assert_eq!(board.get(0, 0), Rock::None);
    assert_eq!(board.get(1, 0), Rock::None);
    assert!(board.moves_restore.is_empty());
    assert_eq!(board.black.rocks, BTreeSet::new());
    assert_eq!(board.white.rocks, BTreeSet::new());
    assert_eq!(board.all_rocks, BTreeSet::new());
}

#[test]
fn undo_capture() {
    let mut board = Board::default();

    // Setup capture
    set_many!(mut board, Player::Black, (0, 0));
    set_many!(mut board, Player::White, (1, 0), (2, 0));
    assert_eq!(board.get(0, 0), Rock::Black);
    assert_eq!(board.get_for_player(0, 0, Player::Black), false);
    assert_eq!(board.get_for_player(0, 0, Player::White), true);
    assert_eq!(board.get(1, 0), Rock::White);
    assert_eq!(board.get_for_player(1, 0, Player::Black), true);
    assert_eq!(board.get_for_player(1, 0, Player::White), false);
    assert_eq!(board.get(2, 0), Rock::White);
    assert_eq!(board.get_for_player(2, 0, Player::Black), true);
    assert_eq!(board.get_for_player(2, 0, Player::White), false);
    assert_eq!(board.moves_restore, vec![vec![], vec![], vec![]]);
    assert_eq!(board.black.rocks, BTreeSet::from([coord!(0, 0)]));
    assert_eq!(board.black.captures, 0);
    assert_eq!(
        board.white.rocks,
        BTreeSet::from([coord!(1, 0), coord!(2, 0)])
    );
    assert_eq!(board.white.captures, 0);
    assert_eq!(
        board.all_rocks,
        BTreeSet::from([coord!(0, 0), coord!(1, 0), coord!(2, 0)])
    );

    // Make capture move
    let movement = Move {
        player: Player::Black,
        coordinates: coord!(3, 0),
    };
    board.set_move(&RuleSet::default(), &movement);
    assert_eq!(board.get(0, 0), Rock::Black);
    assert_eq!(board.get_for_player(0, 0, Player::Black), false);
    assert_eq!(board.get_for_player(0, 0, Player::White), true);
    assert_eq!(board.get(1, 0), Rock::None);
    assert_eq!(board.get_for_player(1, 0, Player::Black), true);
    assert_eq!(board.get_for_player(1, 0, Player::White), true);
    assert_eq!(board.get(2, 0), Rock::None);
    assert_eq!(board.get_for_player(2, 0, Player::Black), true);
    assert_eq!(board.get_for_player(2, 0, Player::White), true);
    assert_eq!(board.get(3, 0), Rock::Black);
    assert_eq!(board.get_for_player(0, 0, Player::Black), false);
    assert_eq!(board.get_for_player(0, 0, Player::White), true);
    assert_eq!(
        board.moves_restore,
        vec![vec![], vec![], vec![], vec![coord!(2, 0), coord!(1, 0)]]
    );
    assert_eq!(
        board.black.rocks,
        BTreeSet::from([coord!(0, 0), coord!(3, 0)])
    );
    assert_eq!(board.black.captures, 2);
    assert_eq!(board.white.rocks, BTreeSet::new());
    assert_eq!(board.white.captures, 0);
    assert_eq!(
        board.all_rocks,
        BTreeSet::from([coord!(0, 0), coord!(3, 0)])
    );

    // Undo capture
    board.undo_move(&RuleSet::default(), &movement);
    assert_eq!(board.get(0, 0), Rock::Black);
    assert_eq!(board.get_for_player(0, 0, Player::Black), false);
    assert_eq!(board.get_for_player(0, 0, Player::White), true);
    assert_eq!(board.get(1, 0), Rock::White);
    assert_eq!(board.get_for_player(1, 0, Player::Black), true);
    assert_eq!(board.get_for_player(1, 0, Player::White), false);
    assert_eq!(board.get(2, 0), Rock::White);
    assert_eq!(board.get_for_player(2, 0, Player::Black), true);
    assert_eq!(board.get_for_player(2, 0, Player::White), false);
    assert_eq!(board.moves_restore, vec![vec![], vec![], vec![]]);
    assert_eq!(board.black.rocks, BTreeSet::from([coord!(0, 0)]));
    assert_eq!(board.black.captures, 0);
    assert_eq!(
        board.white.rocks,
        BTreeSet::from([coord!(2, 0), coord!(1, 0)])
    );
    assert_eq!(board.white.captures, 0);
    assert_eq!(
        board.all_rocks,
        BTreeSet::from([coord!(0, 0), coord!(1, 0), coord!(2, 0)])
    );
}

// * Intersections

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

// * Player rocks

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
    assert_eq!(board.black.rocks, BTreeSet::from([CENTER]));
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
    assert_eq!(board.black.rocks, BTreeSet::from([CENTER, center_right]));
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
    assert_eq!(board.white.rocks, BTreeSet::from([CENTER]));
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
    assert_eq!(board.white.rocks, BTreeSet::from([CENTER, center_right]));
}

// * Five in a row detection

#[test]
fn five_in_a_row_center() {
    let mut board = Board::default();
    set_many!(
        mut board,
        Player::Black,
        (CENTER.x, CENTER.y),
        (CENTER.x + 1, CENTER.y),
        (CENTER.x + 2, CENTER.y),
        (CENTER.x + 3, CENTER.y),
        (CENTER.x + 4, CENTER.y)
    );
    assert!(board.has_five_in_a_row(Player::Black));
}

#[test]
fn five_in_a_row_top_left_horizontal() {
    let mut board = Board::default();
    set_many!(
        mut board,
        Player::Black,
        (0, 0),
        (1, 0),
        (2, 0),
        (3, 0),
        (4, 0)
    );
    assert!(board.has_five_in_a_row(Player::Black));
}

#[test]
fn five_in_a_row_top_left_diagonal() {
    let mut board = Board::default();
    set_many!(
        mut board,
        Player::Black,
        (0, 0),
        (1, 1),
        (2, 2),
        (3, 3),
        (4, 4)
    );
    assert!(board.has_five_in_a_row(Player::Black));
}

#[test]
fn five_in_a_row_top_left_vertical() {
    let mut board = Board::default();
    set_many!(
        mut board,
        Player::Black,
        (0, 0),
        (0, 1),
        (0, 2),
        (0, 3),
        (0, 4)
    );
    assert!(board.has_five_in_a_row(Player::Black));
}

#[test]
fn five_in_a_row_top_right_horizontal() {
    let mut board = Board::default();
    set_many!(
        mut board,
        Player::Black,
        (BORDER, 0),
        (BORDER - 1, 0),
        (BORDER - 2, 0),
        (BORDER - 3, 0),
        (BORDER - 4, 0)
    );
    assert!(board.has_five_in_a_row(Player::Black));
}

#[test]
fn five_in_a_row_top_right_diagonal() {
    let mut board = Board::default();
    set_many!(
        mut board,
        Player::Black,
        (BORDER, 0),
        (BORDER - 1, 1),
        (BORDER - 2, 2),
        (BORDER - 3, 3),
        (BORDER - 4, 4)
    );
    assert!(board.has_five_in_a_row(Player::Black));
}

#[test]
fn five_in_a_row_top_right_vertical() {
    let mut board = Board::default();
    set_many!(
        mut board,
        Player::Black,
        (BORDER, 0),
        (BORDER, 1),
        (BORDER, 2),
        (BORDER, 3),
        (BORDER, 4)
    );
    assert!(board.has_five_in_a_row(Player::Black));
}

#[test]
fn five_in_a_row_bottom_left_horizontal() {
    let mut board = Board::default();
    set_many!(
        mut board,
        Player::Black,
        (0, BORDER),
        (1, BORDER),
        (2, BORDER),
        (3, BORDER),
        (4, BORDER)
    );
    assert!(board.has_five_in_a_row(Player::Black));
}

#[test]
fn five_in_a_row_bottom_left_diagonal() {
    let mut board = Board::default();
    set_many!(
        mut board,
        Player::Black,
        (0, BORDER),
        (1, BORDER - 1),
        (2, BORDER - 2),
        (3, BORDER - 3),
        (4, BORDER - 4)
    );
    assert!(board.has_five_in_a_row(Player::Black));
}

#[test]
fn five_in_a_row_bottom_left_vertical() {
    let mut board = Board::default();
    set_many!(
        mut board,
        Player::Black,
        (0, BORDER),
        (0, BORDER - 1),
        (0, BORDER - 2),
        (0, BORDER - 3),
        (0, BORDER - 4)
    );
    assert!(board.has_five_in_a_row(Player::Black));
}

#[test]
fn five_in_a_row_bottom_right_horizontal() {
    let mut board = Board::default();
    set_many!(
        mut board,
        Player::Black,
        (BORDER, BORDER),
        (BORDER - 1, BORDER),
        (BORDER - 2, BORDER),
        (BORDER - 3, BORDER),
        (BORDER - 4, BORDER)
    );
    assert!(board.has_five_in_a_row(Player::Black));
}

#[test]
fn five_in_a_row_bottom_right_diagonal() {
    let mut board = Board::default();
    set_many!(
        mut board,
        Player::Black,
        (BORDER, BORDER),
        (BORDER - 1, BORDER - 1),
        (BORDER - 2, BORDER - 2),
        (BORDER - 3, BORDER - 3),
        (BORDER - 4, BORDER - 4)
    );
    assert!(board.has_five_in_a_row(Player::Black));
}

#[test]
fn five_in_a_row_bottom_right_vertical() {
    let mut board = Board::default();
    set_many!(
        mut board,
        Player::Black,
        (BORDER, BORDER),
        (BORDER, BORDER - 1),
        (BORDER, BORDER - 2),
        (BORDER, BORDER - 3),
        (BORDER, BORDER - 4)
    );
    assert!(board.has_five_in_a_row(Player::Black));
}

#[test]
fn four_in_a_row_no_match() {
    let mut board = Board::default();
    set_many!(
        mut board,
        Player::Black,
        (CENTER.x, CENTER.y),
        (CENTER.x - 1, CENTER.y),
        (CENTER.x - 2, CENTER.y),
        (CENTER.x + 1, CENTER.y)
    );
    assert!(!board.has_five_in_a_row(Player::Black));
}

// * Recursive captures detection

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

// * Single free three detection

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
    assert_eq!(
        board.move_create_free_three_direct_pattern(&second_move.coordinates, Player::Black),
        0
    );
    assert_eq!(
        board.move_create_free_three_secondary_pattern(&second_move.coordinates, Player::Black),
        0
    );

    let free_three_move = Move {
        player: Player::Black,
        coordinates: coord!(3, 0),
    };
    assert_eq!(
        board.move_create_free_three_direct_pattern(&free_three_move.coordinates, Player::Black),
        1
    );
    assert_eq!(
        board.move_create_free_three_secondary_pattern(&free_three_move.coordinates, Player::Black),
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
    assert_eq!(
        board.move_create_free_three_direct_pattern(&second_move.coordinates, Player::Black),
        0
    );
    assert_eq!(
        board.move_create_free_three_secondary_pattern(&second_move.coordinates, Player::Black),
        0
    );

    let free_three_move = Move {
        player: Player::Black,
        coordinates: coord!(0, 3),
    };
    assert_eq!(
        board.move_create_free_three_direct_pattern(&free_three_move.coordinates, Player::Black),
        1
    );
    assert_eq!(
        board.move_create_free_three_secondary_pattern(&free_three_move.coordinates, Player::Black),
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
    assert_eq!(
        board.move_create_free_three_direct_pattern(&second_move.coordinates, Player::Black),
        0
    );
    assert_eq!(
        board.move_create_free_three_secondary_pattern(&second_move.coordinates, Player::Black),
        0
    );

    let free_three_move = Move {
        player: Player::Black,
        coordinates: coord!(3, 3),
    };
    assert_eq!(
        board.move_create_free_three_direct_pattern(&free_three_move.coordinates, Player::Black),
        1
    );
    assert_eq!(
        board.move_create_free_three_secondary_pattern(&free_three_move.coordinates, Player::Black),
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
    assert_eq!(
        board.move_create_free_three_direct_pattern(&second_move.coordinates, Player::Black),
        0
    );
    assert_eq!(
        board.move_create_free_three_secondary_pattern(&second_move.coordinates, Player::Black),
        0
    );

    let free_three_move = Move {
        player: Player::Black,
        coordinates: coord!(BORDER - 3, BORDER - 3),
    };
    assert_eq!(
        board.move_create_free_three_direct_pattern(&free_three_move.coordinates, Player::Black),
        1
    );
    assert_eq!(
        board.move_create_free_three_secondary_pattern(&free_three_move.coordinates, Player::Black),
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
    assert_eq!(
        board.move_create_free_three_direct_pattern(&second_move.coordinates, Player::Black),
        0
    );
    assert_eq!(
        board.move_create_free_three_secondary_pattern(&second_move.coordinates, Player::Black),
        0
    );

    let free_three_move = Move {
        player: Player::Black,
        coordinates: coord!(4, 0),
    };
    assert_eq!(
        board.move_create_free_three_direct_pattern(&free_three_move.coordinates, Player::Black),
        0
    );
    assert_eq!(
        board.move_create_free_three_secondary_pattern(&free_three_move.coordinates, Player::Black),
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
    assert_eq!(
        board.move_create_free_three_direct_pattern(&second_move.coordinates, Player::Black),
        0
    );
    assert_eq!(
        board.move_create_free_three_secondary_pattern(&second_move.coordinates, Player::Black),
        0
    );

    let free_three_move = Move {
        player: Player::Black,
        coordinates: coord!(0, 4),
    };
    assert_eq!(
        board.move_create_free_three_direct_pattern(&free_three_move.coordinates, Player::Black),
        0
    );
    assert_eq!(
        board.move_create_free_three_secondary_pattern(&free_three_move.coordinates, Player::Black),
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
    assert_eq!(
        board.move_create_free_three_direct_pattern(&second_move.coordinates, Player::Black),
        0
    );
    assert_eq!(
        board.move_create_free_three_secondary_pattern(&second_move.coordinates, Player::Black),
        0
    );

    let free_three_move = Move {
        player: Player::Black,
        coordinates: coord!(4, 4),
    };
    assert_eq!(
        board.move_create_free_three_direct_pattern(&free_three_move.coordinates, Player::Black),
        0
    );
    assert_eq!(
        board.move_create_free_three_secondary_pattern(&free_three_move.coordinates, Player::Black),
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
    assert_eq!(
        board.move_create_free_three_direct_pattern(&second_move.coordinates, Player::Black),
        0
    );
    assert_eq!(
        board.move_create_free_three_secondary_pattern(&second_move.coordinates, Player::Black),
        0
    );

    let free_three_move = Move {
        player: Player::Black,
        coordinates: coord!(BORDER - 4, BORDER - 4),
    };
    assert_eq!(
        board.move_create_free_three_direct_pattern(&free_three_move.coordinates, Player::Black),
        0
    );
    assert_eq!(
        board.move_create_free_three_secondary_pattern(&free_three_move.coordinates, Player::Black),
        1
    );
}

// * Double free three

#[test]
fn create_double_free_three_1() {
    let mut board = Board::default();

    // [1, 1,>{0}<, 1, 1]
    set_many!(
        mut board,
        Player::Black,
        (CENTER.x, CENTER.y - 2),
        (CENTER.x, CENTER.y - 1),
        (CENTER.x + 1, CENTER.y),
        (CENTER.x + 2, CENTER.y)
    );
    let movement = Move {
        player: Player::Black,
        coordinates: coord!(CENTER.x, CENTER.y),
    };
    assert!(board.movement_create_double_free_three(&movement.coordinates, Player::Black));
}

#[test]
fn create_double_free_three_2() {
    let mut board = Board::default();

    // Subject example
    set_many!(
        mut board,
        Player::Black,
        (CENTER.x - 3, CENTER.y - 3),
        (CENTER.x - 2, CENTER.y - 2),
        (CENTER.x + 1, CENTER.y),
        (CENTER.x + 2, CENTER.y)
    );
    let movement = Move {
        player: Player::Black,
        coordinates: coord!(CENTER.x, CENTER.y),
    };
    assert!(board.movement_create_double_free_three(&movement.coordinates, Player::Black));
}

// * Recursive capture

#[test]
fn movement_create_recursive_capture_horizontal_1() {
    let mut board = Board::default();
    set_many!(
        mut board,
        Player::Black,
        (CENTER.x, CENTER.y),
        (CENTER.x + 3, CENTER.y)
    );
    set_many!(mut board, Player::White, (CENTER.x + 1, CENTER.y));
    assert!(board.movement_create_recursive_capture(&coord!(CENTER.x + 2, CENTER.y), Player::White));

    let mut board = Board::default();
    set_many!(
        mut board,
        Player::Black,
        (CENTER.x, CENTER.y),
        (CENTER.x + 3, CENTER.y)
    );
    set_many!(mut board, Player::White, (CENTER.x + 2, CENTER.y));
    assert!(board.movement_create_recursive_capture(&coord!(CENTER.x + 1, CENTER.y), Player::White));
}

#[test]
fn movement_create_recursive_capture_horizontal_2() {
    let mut board = Board::default();
    set_many!(
        mut board,
        Player::Black,
        (CENTER.x, CENTER.y),
        (CENTER.x - 3, CENTER.y)
    );
    set_many!(mut board, Player::White, (CENTER.x - 1, CENTER.y));
    assert!(board.movement_create_recursive_capture(&coord!(CENTER.x - 2, CENTER.y), Player::White));

    let mut board = Board::default();
    set_many!(
        mut board,
        Player::Black,
        (CENTER.x, CENTER.y),
        (CENTER.x - 3, CENTER.y)
    );
    set_many!(mut board, Player::White, (CENTER.x - 2, CENTER.y));
    assert!(board.movement_create_recursive_capture(&coord!(CENTER.x - 1, CENTER.y), Player::White));
}

#[test]
fn movement_create_recursive_capture_vertical_1() {
    let mut board = Board::default();
    set_many!(
        mut board,
        Player::Black,
        (CENTER.x, CENTER.y),
        (CENTER.x, CENTER.y + 3)
    );
    set_many!(mut board, Player::White, (CENTER.x, CENTER.y + 1));
    assert!(board.movement_create_recursive_capture(&coord!(CENTER.x, CENTER.y + 2), Player::White));

    let mut board = Board::default();
    set_many!(
        mut board,
        Player::Black,
        (CENTER.x, CENTER.y),
        (CENTER.x, CENTER.y + 3)
    );
    set_many!(mut board, Player::White, (CENTER.x, CENTER.y + 2));
    assert!(board.movement_create_recursive_capture(&coord!(CENTER.x, CENTER.y + 1), Player::White));
}

#[test]
fn movement_create_recursive_capture_vertical_2() {
    let mut board = Board::default();
    set_many!(
        mut board,
        Player::Black,
        (CENTER.x, CENTER.y),
        (CENTER.x, CENTER.y - 3)
    );
    set_many!(mut board, Player::White, (CENTER.x, CENTER.y - 1));
    assert!(board.movement_create_recursive_capture(&coord!(CENTER.x, CENTER.y - 2), Player::White));

    let mut board = Board::default();
    set_many!(
        mut board,
        Player::Black,
        (CENTER.x, CENTER.y),
        (CENTER.x, CENTER.y - 3)
    );
    set_many!(mut board, Player::White, (CENTER.x, CENTER.y - 2));
    assert!(board.movement_create_recursive_capture(&coord!(CENTER.x, CENTER.y - 1), Player::White));
}

#[test]
fn movement_create_recursive_capture_diagonal_left_1() {
    let mut board = Board::default();
    set_many!(
        mut board,
        Player::Black,
        (CENTER.x, CENTER.y),
        (CENTER.x - 3, CENTER.y - 3)
    );
    set_many!(mut board, Player::White, (CENTER.x - 1, CENTER.y - 1));
    assert!(
        board.movement_create_recursive_capture(&coord!(CENTER.x - 2, CENTER.y - 2), Player::White)
    );

    let mut board = Board::default();
    set_many!(
        mut board,
        Player::Black,
        (CENTER.x, CENTER.y),
        (CENTER.x - 3, CENTER.y - 3)
    );
    set_many!(mut board, Player::White, (CENTER.x - 2, CENTER.y - 2));
    assert!(
        board.movement_create_recursive_capture(&coord!(CENTER.x - 1, CENTER.y - 1), Player::White)
    );
}

#[test]
fn movement_create_recursive_capture_diagonal_left_2() {
    let mut board = Board::default();
    set_many!(
        mut board,
        Player::Black,
        (CENTER.x, CENTER.y),
        (CENTER.x + 3, CENTER.y + 3)
    );
    set_many!(mut board, Player::White, (CENTER.x + 1, CENTER.y + 1));
    assert!(
        board.movement_create_recursive_capture(&coord!(CENTER.x + 2, CENTER.y + 2), Player::White)
    );

    let mut board = Board::default();
    set_many!(
        mut board,
        Player::Black,
        (CENTER.x, CENTER.y),
        (CENTER.x + 3, CENTER.y + 3)
    );
    set_many!(mut board, Player::White, (CENTER.x + 2, CENTER.y + 2));
    assert!(
        board.movement_create_recursive_capture(&coord!(CENTER.x + 1, CENTER.y + 1), Player::White)
    );
}

#[test]
fn movement_create_recursive_capture_diagonal_right_1() {
    let mut board = Board::default();
    set_many!(
        mut board,
        Player::Black,
        (CENTER.x, CENTER.y),
        (CENTER.x + 3, CENTER.y - 3)
    );
    set_many!(mut board, Player::White, (CENTER.x + 1, CENTER.y - 1));
    assert!(
        board.movement_create_recursive_capture(&coord!(CENTER.x + 2, CENTER.y - 2), Player::White)
    );

    let mut board = Board::default();
    set_many!(
        mut board,
        Player::Black,
        (CENTER.x, CENTER.y),
        (CENTER.x + 3, CENTER.y - 3)
    );
    set_many!(mut board, Player::White, (CENTER.x + 2, CENTER.y - 2));
    assert!(
        board.movement_create_recursive_capture(&coord!(CENTER.x + 1, CENTER.y - 1), Player::White)
    );
}

#[test]
fn movement_create_recursive_capture_diagonal_right_2() {
    let mut board = Board::default();
    set_many!(
        mut board,
        Player::Black,
        (CENTER.x, CENTER.y),
        (CENTER.x - 3, CENTER.y + 3)
    );
    set_many!(mut board, Player::White, (CENTER.x - 1, CENTER.y + 1));
    assert!(
        board.movement_create_recursive_capture(&coord!(CENTER.x - 2, CENTER.y + 2), Player::White)
    );

    let mut board = Board::default();
    set_many!(
        mut board,
        Player::Black,
        (CENTER.x, CENTER.y),
        (CENTER.x - 3, CENTER.y + 3)
    );
    set_many!(mut board, Player::White, (CENTER.x - 2, CENTER.y + 2));
    assert!(
        board.movement_create_recursive_capture(&coord!(CENTER.x - 1, CENTER.y + 1), Player::White)
    );
}

// * Five in a row under capture

#[test]
fn captured_five_in_a_row_horizontal_1() {
    let rules = RuleSet::default();
    let mut board = Board::default();

    set_many!(
        mut board,
        Player::Black,
        (1, 2),
        (2, 2),
        (3, 2),
        (4, 2),
        (5, 2),
        (1, 1) // Rock that allow capture
    );
    set_many!(mut board, Player::White, (1, 0));

    assert!(board.has_five_in_a_row(Player::Black));
    assert!(!board.has_uncaptured_five_in_a_row(&RuleSet::default(), Player::Black));
    assert!(!board.is_winning(&rules, Player::Black));
}

#[test]
fn captured_five_in_a_row_horizontal_2() {
    let rules = RuleSet::default();
    let mut board = Board::default();

    set_many!(
        mut board,
        Player::Black,
        (1, 2),
        (2, 2),
        (3, 2),
        (4, 2),
        (5, 2),
        (1, 1) // Rock that allow capture
    );
    set_many!(mut board, Player::White, (1, 3));

    assert!(board.has_five_in_a_row(Player::Black));
    assert!(!board.has_uncaptured_five_in_a_row(&RuleSet::default(), Player::Black));
    assert!(!board.is_winning(&rules, Player::Black));
}

#[test]
fn captured_five_in_a_row_vertical_1() {
    let rules = RuleSet::default();
    let mut board = Board::default();

    set_many!(
        mut board,
        Player::Black,
        (2, 1),
        (2, 2),
        (2, 3),
        (2, 4),
        (2, 5),
        (1, 1) // Rock that allow capture
    );
    set_many!(mut board, Player::White, (3, 1));

    assert!(board.has_five_in_a_row(Player::Black));
    assert!(!board.has_uncaptured_five_in_a_row(&RuleSet::default(), Player::Black));
    assert!(!board.is_winning(&rules, Player::Black));
}

#[test]
fn captured_five_in_a_row_vertical_2() {
    let rules = RuleSet::default();
    let mut board = Board::default();

    set_many!(
        mut board,
        Player::Black,
        (2, 1),
        (2, 2),
        (2, 3),
        (2, 4),
        (2, 5),
        (1, 1) // Rock that allow capture
    );
    set_many!(mut board, Player::White, (0, 1));

    assert!(board.has_five_in_a_row(Player::Black));
    assert!(!board.has_uncaptured_five_in_a_row(&RuleSet::default(), Player::Black));
    assert!(!board.is_winning(&rules, Player::Black));
}

#[test]
fn captured_five_in_a_row_diagonal_1() {
    let rules = RuleSet::default();
    let mut board = Board::default();

    set_many!(
        mut board,
        Player::Black,
        (2, 2),
        (3, 3),
        (4, 4),
        (5, 5),
        (6, 6),
        (1, 3) // Rock that allow capture
    );
    set_many!(mut board, Player::White, (3, 1));

    assert!(board.has_five_in_a_row(Player::Black));
    assert!(!board.has_uncaptured_five_in_a_row(&RuleSet::default(), Player::Black));
    assert!(!board.is_winning(&rules, Player::Black));
}

#[test]
fn captured_five_in_a_row_diagonal_2() {
    let rules = RuleSet::default();
    let mut board = Board::default();

    set_many!(
        mut board,
        Player::Black,
        (2, 2),
        (3, 3),
        (4, 4),
        (5, 5),
        (6, 6),
        (1, 3) // Rock that allow capture
    );
    set_many!(mut board, Player::White, (0, 4));

    assert!(board.has_five_in_a_row(Player::Black));
    assert!(!board.has_uncaptured_five_in_a_row(&RuleSet::default(), Player::Black));
    assert!(!board.is_winning(&rules, Player::Black));
}

#[test]
fn captured_five_in_a_row_diagonal_3() {
    let rules = RuleSet::default();
    let mut board = Board::default();

    set_many!(
        mut board,
        Player::Black,
        (8, 8),
        (7, 9),
        (6, 10),
        (5, 11),
        (4, 12),
        (7, 7) // Rock that allow capture
    );
    set_many!(mut board, Player::White, (6, 6));

    assert!(board.has_five_in_a_row(Player::Black));
    assert!(!board.has_uncaptured_five_in_a_row(&RuleSet::default(), Player::Black));
    assert!(!board.is_winning(&rules, Player::Black));
}

#[test]
fn captured_five_in_a_row_diagonal_4() {
    let rules = RuleSet::default();
    let mut board = Board::default();

    set_many!(
        mut board,
        Player::Black,
        (8, 8),
        (7, 9),
        (6, 10),
        (5, 11),
        (4, 12),
        (7, 7) // Rock that allow capture
    );
    set_many!(mut board, Player::White, (9, 9));

    assert!(board.has_five_in_a_row(Player::Black));
    assert!(!board.has_uncaptured_five_in_a_row(&RuleSet::default(), Player::Black));
    assert!(!board.is_winning(&rules, Player::Black));
}

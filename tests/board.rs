use gomoku::{
    board::{Board, Move, Pawn, BOARD_PIECES, BOARD_SIZE},
    player::Player,
    rules::RuleSet,
};

#[test]
fn coordinates_to_index() {
    let mut i = 0;
    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            assert!(
                Board::coordinates_to_index(x, y) == i,
                "index was {:#?} instead of {} ({}x{})",
                Board::coordinates_to_index(x, y),
                i,
                x,
                y
            );
            i += 1;
        }
    }
}

#[test]
fn index_to_coordinates() {
    let mut x: usize = 0;
    let mut y: usize = 0;
    for i in 0..BOARD_PIECES {
        assert!(
            Board::index_to_coordinates(i) == (x, y),
            "coordinates were {:#?} instead of {}x{} ({})",
            Board::index_to_coordinates(i),
            x,
            y,
            i
        );
        x += 1;
        if x == BOARD_SIZE {
            x = 0;
            y += 1;
        }
    }
}

#[test]
fn board_set_move() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 180,
        },
    );
    let mut raw_board = [Pawn::None; BOARD_PIECES];
    raw_board[180] = Pawn::Black;
    assert_eq!(board.pieces, raw_board);
}

// #[test]
// fn board_set_move_fail() {
//     let mut board = Board::default();
//     assert!(board
//         .set_move(
//             &RuleSet::default(),
//             &Move {
//                 player: Player::Black,
//                 index: 180,
//             },
//         )
//         .is_ok());
//     let mut raw_board = [Pawn::None; BOARD_PIECES];
//     raw_board[180] = Pawn::Black;
//     assert_eq!(board.pieces, raw_board);
//     assert!(board
//         .set_move(
//             &RuleSet::default(),
//             &Move {
//                 player: Player::Black,
//                 index: 180,
//             },
//         )
//         .is_err());
// }

#[test]
fn open_intersections_empty_board() {
    let board = Board::default();
    assert_eq!(board.open_intersections(), vec![180]);
}

#[test]
fn open_intersections_one_pawn() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 180,
        },
    );
    assert_eq!(
        board.open_intersections(),
        vec![160, 179, 198, 161, 199, 162, 181, 200],
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
            index: 180,
        },
    );
    assert_eq!(board.black_rocks, vec![180]);
}

#[test]
fn board_save_black_moves_2() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 180,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 181,
        },
    );
    assert_eq!(board.black_rocks, vec![180, 181]);
}

#[test]
fn board_save_white_moves_1() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::White,
            index: 180,
        },
    );
    assert_eq!(board.white_rocks, vec![180]);
}

#[test]
fn board_save_white_moves_2() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::White,
            index: 180,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::White,
            index: 181,
        },
    );
    assert_eq!(board.white_rocks, vec![180, 181]);
}

#[test]
fn five_in_a_row_center() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 180,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 181,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 182,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 183,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 184,
        },
    );
    assert!(board.has_five_in_a_row(&Player::Black));
}

#[test]
fn five_in_a_row_top_left_horizontal() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 0,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 1,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 2,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 3,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 4,
        },
    );
    assert!(board.has_five_in_a_row(&Player::Black));
}

#[test]
fn five_in_a_row_top_left_diagonal() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 0,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 20,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 40,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 60,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 80,
        },
    );
    assert!(board.has_five_in_a_row(&Player::Black));
}

#[test]
fn five_in_a_row_top_left_vertical() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 0,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 19,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 38,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 57,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 76,
        },
    );
    assert!(board.has_five_in_a_row(&Player::Black));
}

#[test]
fn five_in_a_row_top_right_horizontal() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 14,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 15,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 16,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 17,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 18,
        },
    );
    assert!(board.has_five_in_a_row(&Player::Black));
}

#[test]
fn five_in_a_row_top_right_diagonal() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 18,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 36,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 54,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 72,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 90,
        },
    );
    assert!(board.has_five_in_a_row(&Player::Black));
}

#[test]
fn five_in_a_row_top_right_vertical() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 18,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 37,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 56,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 75,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 94,
        },
    );
    assert!(board.has_five_in_a_row(&Player::Black));
}

#[test]
fn five_in_a_row_bottom_left_horizontal() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 342,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 343,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 344,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 345,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 346,
        },
    );
    assert!(board.has_five_in_a_row(&Player::Black));
}

#[test]
fn five_in_a_row_bottom_left_diagonal() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 342,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 324,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 306,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 288,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 270,
        },
    );
    assert!(board.has_five_in_a_row(&Player::Black));
}

#[test]
fn five_in_a_row_bottom_left_vertical() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 342,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 323,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 304,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 285,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 266,
        },
    );
    assert!(board.has_five_in_a_row(&Player::Black));
}

#[test]
fn five_in_a_row_bottom_right_horizontal() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 360,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 359,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 358,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 357,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 356,
        },
    );
    assert!(board.has_five_in_a_row(&Player::Black));
}

#[test]
fn five_in_a_row_bottom_right_diagonal() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 360,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 340,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 320,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 300,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 280,
        },
    );
    assert!(board.has_five_in_a_row(&Player::Black));
}

#[test]
fn five_in_a_row_bottom_right_vertical() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 360,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 341,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 322,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 303,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 284,
        },
    );
    assert!(board.has_five_in_a_row(&Player::Black));
}

#[test]
fn four_in_a_row_no_match() {
    let mut board = Board::default();
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 180,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 181,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 182,
        },
    );
    board.set_move(
        &RuleSet::default(),
        &Move {
            player: Player::Black,
            index: 183,
        },
    );
    assert!(!board.has_five_in_a_row(&Player::Black));
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
    x_offset: usize,
    y_offset: usize,
) -> (Board, Vec<(usize, usize)>) {
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
                index: Board::coordinates_to_index(rock.0 + x_offset, rock.1 + y_offset),
            },
        )
    }

    for rock in blocked_rocks {
        board.set_move(
            &rules,
            &Move {
                player: Player::White,
                index: Board::coordinates_to_index(rock.0 + x_offset, rock.1 + y_offset),
            },
        )
    }

    (board, move_rocks)
}

fn assert_recursive_capture_are_illegal(board: Board, moves: Vec<(usize, usize)>) {
    let rules = RuleSet::default();
    for movement in moves {
        assert!(
            !board.is_move_legal(
                &rules,
                &Move {
                    player: Player::White,
                    index: Board::coordinates_to_index(movement.0, movement.1),
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
            index: 1,
            player: Player::Black,
        },
    );
    board.set_move(
        &rules,
        &Move {
            index: 2,
            player: Player::Black,
        },
    );
    assert!(!board.has_free_three(&Player::Black));
    assert!(!board.has_free_three(&Player::White));

    let free_three_move = Move {
        index: 3,
        player: Player::Black,
    };
    assert!(board.move_create_free_three(&free_three_move));

    board.set_move(&rules, &free_three_move);
    assert!(board.has_free_three(&Player::Black));
    assert!(!board.has_free_three(&Player::White));
}

#[test]
fn free_three_1_detected_vertical() {
    let rules = RuleSet::default();
    let mut board = Board::default();

    board.set_move(
        &rules,
        &Move {
            index: 20,
            player: Player::Black,
        },
    );
    board.set_move(
        &rules,
        &Move {
            index: 39,
            player: Player::Black,
        },
    );
    assert!(!board.has_free_three(&Player::Black));
    assert!(!board.has_free_three(&Player::White));

    let free_three_move = Move {
        index: 58,
        player: Player::Black,
    };
    assert!(board.move_create_free_three(&free_three_move));

    board.set_move(&rules, &free_three_move);
    assert!(board.has_free_three(&Player::Black));
    assert!(!board.has_free_three(&Player::White));
}

#[test]
fn free_three_1_detected_diagonal_left() {
    let rules = RuleSet::default();
    let mut board = Board::default();

    board.set_move(
        &rules,
        &Move {
            index: 22,
            player: Player::Black,
        },
    );
    board.set_move(
        &rules,
        &Move {
            index: 40,
            player: Player::Black,
        },
    );
    assert!(!board.has_free_three(&Player::Black));
    assert!(!board.has_free_three(&Player::White));

    let free_three_move = Move {
        index: 58,
        player: Player::Black,
    };
    assert!(board.move_create_free_three(&free_three_move));

    board.set_move(&rules, &free_three_move);
    assert!(board.has_free_three(&Player::Black));
    assert!(!board.has_free_three(&Player::White));
}

#[test]
fn free_three_1_detected_diagonal_right() {
    let rules = RuleSet::default();
    let mut board = Board::default();

    board.set_move(
        &rules,
        &Move {
            index: 20,
            player: Player::Black,
        },
    );
    board.set_move(
        &rules,
        &Move {
            index: 40,
            player: Player::Black,
        },
    );
    assert!(!board.has_free_three(&Player::Black));
    assert!(!board.has_free_three(&Player::White));

    let free_three_move = Move {
        index: 60,
        player: Player::Black,
    };
    assert!(board.move_create_free_three(&free_three_move));

    board.set_move(&rules, &free_three_move);
    assert!(board.has_free_three(&Player::Black));
    assert!(!board.has_free_three(&Player::White));
}

#[test]
fn free_three_2_detected_horizontal() {
    let rules = RuleSet::default();
    let mut board = Board::default();

    board.set_move(
        &rules,
        &Move {
            index: 1,
            player: Player::Black,
        },
    );
    board.set_move(
        &rules,
        &Move {
            index: 2,
            player: Player::Black,
        },
    );
    assert!(!board.has_free_three(&Player::Black));
    assert!(!board.has_free_three(&Player::White));

    let free_three_move = Move {
        index: 4,
        player: Player::Black,
    };
    assert!(board.move_create_free_three(&free_three_move));

    board.set_move(&rules, &free_three_move);
    assert!(board.has_free_three(&Player::Black));
    assert!(!board.has_free_three(&Player::White));
}

#[test]
fn free_three_2_detected_vertical() {
    let rules = RuleSet::default();
    let mut board = Board::default();

    board.set_move(
        &rules,
        &Move {
            index: 20,
            player: Player::Black,
        },
    );
    board.set_move(
        &rules,
        &Move {
            index: 39,
            player: Player::Black,
        },
    );
    assert!(!board.has_free_three(&Player::Black));
    assert!(!board.has_free_three(&Player::White));

    let free_three_move = Move {
        index: 77,
        player: Player::Black,
    };
    assert!(board.move_create_free_three(&free_three_move));

    board.set_move(&rules, &free_three_move);
    assert!(board.has_free_three(&Player::Black));
    assert!(!board.has_free_three(&Player::White));
}

#[test]
fn free_three_2_detected_diagonal_left() {
    let rules = RuleSet::default();
    let mut board = Board::default();

    board.set_move(
        &rules,
        &Move {
            index: 23,
            player: Player::Black,
        },
    );
    board.set_move(
        &rules,
        &Move {
            index: 41,
            player: Player::Black,
        },
    );
    assert!(!board.has_free_three(&Player::Black));
    assert!(!board.has_free_three(&Player::White));

    let free_three_move = Move {
        index: 77,
        player: Player::Black,
    };
    assert!(board.move_create_free_three(&free_three_move));

    board.set_move(&rules, &free_three_move);
    assert!(board.has_free_three(&Player::Black));
    assert!(!board.has_free_three(&Player::White));
}

#[test]
fn free_three_2_detected_diagonal_right() {
    let rules = RuleSet::default();
    let mut board = Board::default();

    board.set_move(
        &rules,
        &Move {
            index: 20,
            player: Player::Black,
        },
    );
    board.set_move(
        &rules,
        &Move {
            index: 40,
            player: Player::Black,
        },
    );
    assert!(!board.has_free_three(&Player::Black));
    assert!(!board.has_free_three(&Player::White));

    let free_three_move = Move {
        index: 80,
        player: Player::Black,
    };
    assert!(board.move_create_free_three(&free_three_move));

    board.set_move(&rules, &free_three_move);
    assert!(board.has_free_three(&Player::Black));
    assert!(!board.has_free_three(&Player::White));
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
                index: Board::coordinates_to_index(rock.0, rock.1),
            },
        )
    }

    for rock in capture_rocks {
        board.set_move(
            &rules,
            &Move {
                player: Player::White,
                index: Board::coordinates_to_index(rock.0, rock.1),
            },
        )
    }

    assert!(board.has_five_in_a_row(&Player::Black));
    assert!(!board.has_uncaptured_five_in_a_row(&Player::Black));
    assert!(!board.is_winning(&rules, &Player::Black));
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
                index: Board::coordinates_to_index(rock.0, rock.1),
            },
        )
    }

    for rock in capture_rocks {
        board.set_move(
            &rules,
            &Move {
                player: Player::White,
                index: Board::coordinates_to_index(rock.0, rock.1),
            },
        )
    }

    assert!(board.has_five_in_a_row(&Player::Black));
    assert!(!board.has_uncaptured_five_in_a_row(&Player::Black));
    assert!(!board.is_winning(&rules, &Player::Black));
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
                index: Board::coordinates_to_index(rock.0, rock.1),
            },
        )
    }

    for rock in capture_rocks {
        board.set_move(
            &rules,
            &Move {
                player: Player::White,
                index: Board::coordinates_to_index(rock.0, rock.1),
            },
        )
    }

    assert!(board.has_five_in_a_row(&Player::Black));
    assert!(!board.has_uncaptured_five_in_a_row(&Player::Black));
    assert!(!board.is_winning(&rules, &Player::Black));
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
                index: Board::coordinates_to_index(rock.0, rock.1),
            },
        )
    }

    for rock in capture_rocks {
        board.set_move(
            &rules,
            &Move {
                player: Player::White,
                index: Board::coordinates_to_index(rock.0, rock.1),
            },
        )
    }

    assert!(board.has_five_in_a_row(&Player::Black));
    assert!(!board.has_uncaptured_five_in_a_row(&Player::Black));
    assert!(!board.is_winning(&rules, &Player::Black));
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
                index: Board::coordinates_to_index(rock.0, rock.1),
            },
        )
    }

    for rock in capture_rocks {
        board.set_move(
            &rules,
            &Move {
                player: Player::White,
                index: Board::coordinates_to_index(rock.0, rock.1),
            },
        )
    }

    assert!(board.has_five_in_a_row(&Player::Black));
    assert!(!board.has_uncaptured_five_in_a_row(&Player::Black));
    assert!(!board.is_winning(&rules, &Player::Black));
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
                index: Board::coordinates_to_index(rock.0, rock.1),
            },
        )
    }

    for rock in capture_rocks {
        board.set_move(
            &rules,
            &Move {
                player: Player::White,
                index: Board::coordinates_to_index(rock.0, rock.1),
            },
        )
    }

    assert!(board.has_five_in_a_row(&Player::Black));
    assert!(!board.has_uncaptured_five_in_a_row(&Player::Black));
    assert!(!board.is_winning(&rules, &Player::Black));
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
                index: Board::coordinates_to_index(rock.0, rock.1),
            },
        )
    }

    for rock in capture_rocks {
        board.set_move(
            &rules,
            &Move {
                player: Player::White,
                index: Board::coordinates_to_index(rock.0, rock.1),
            },
        )
    }

    assert!(board.has_five_in_a_row(&Player::Black));
    assert!(!board.has_uncaptured_five_in_a_row(&Player::Black));
    assert!(!board.is_winning(&rules, &Player::Black));
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
                index: Board::coordinates_to_index(rock.0, rock.1),
            },
        )
    }

    for rock in capture_rocks {
        board.set_move(
            &rules,
            &Move {
                player: Player::White,
                index: Board::coordinates_to_index(rock.0, rock.1),
            },
        )
    }

    assert!(board.has_five_in_a_row(&Player::Black));
    assert!(!board.has_uncaptured_five_in_a_row(&Player::Black));
    assert!(!board.is_winning(&rules, &Player::Black));
}

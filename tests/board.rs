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

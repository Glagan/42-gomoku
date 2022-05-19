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

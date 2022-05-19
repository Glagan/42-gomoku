use gomoku::{
    board::{Board, Move},
    computer::Computer,
    player::Player,
    rules::RuleSet,
};

#[test]
fn find_pattern_live_two_horizontal() {
    let mut board = Board::default();
    assert!(board
        .set_move(
            &RuleSet::default(),
            &Move {
                player: Player::Black,
                index: 180,
            },
        )
        .is_ok());
    assert!(board
        .set_move(
            &RuleSet::default(),
            &Move {
                player: Player::Black,
                index: 182,
            },
        )
        .is_ok());
    let computer = Computer::new(&RuleSet::default(), &Player::Black);
    assert_eq!(computer.evaluate_board(&board, &Player::Black), 200);
}

#[test]
fn find_pattern_live_three_horizontal() {
    let mut board = Board::default();
    assert!(board
        .set_move(
            &RuleSet::default(),
            &Move {
                player: Player::Black,
                index: 180,
            },
        )
        .is_ok());
    assert!(board
        .set_move(
            &RuleSet::default(),
            &Move {
                player: Player::Black,
                index: 181,
            },
        )
        .is_ok());
    assert!(board
        .set_move(
            &RuleSet::default(),
            &Move {
                player: Player::Black,
                index: 182,
            },
        )
        .is_ok());
    let computer = Computer::new(&RuleSet::default(), &Player::Black);
    assert_eq!(computer.evaluate_board(&board, &Player::Black), 5000);
}

#[test]
fn find_pattern_live_two_vertical() {
    let mut board = Board::default();
    assert!(board
        .set_move(
            &RuleSet::default(),
            &Move {
                player: Player::Black,
                index: 180,
            },
        )
        .is_ok());
    assert!(board
        .set_move(
            &RuleSet::default(),
            &Move {
                player: Player::Black,
                index: 218,
            },
        )
        .is_ok());
    let computer = Computer::new(&RuleSet::default(), &Player::Black);
    assert_eq!(computer.evaluate_board(&board, &Player::Black), 200);
}

#[test]
fn find_pattern_live_three_vertical() {
    let mut board = Board::default();
    assert!(board
        .set_move(
            &RuleSet::default(),
            &Move {
                player: Player::Black,
                index: 180,
            },
        )
        .is_ok());
    assert!(board
        .set_move(
            &RuleSet::default(),
            &Move {
                player: Player::Black,
                index: 199,
            },
        )
        .is_ok());
    assert!(board
        .set_move(
            &RuleSet::default(),
            &Move {
                player: Player::Black,
                index: 218,
            },
        )
        .is_ok());
    let computer = Computer::new(&RuleSet::default(), &Player::Black);
    assert_eq!(computer.evaluate_board(&board, &Player::Black), 5000);
}

#[test]
fn find_pattern_live_three_vertical_border() {
    let mut board = Board::default();
    assert!(board
        .set_move(
            &RuleSet::default(),
            &Move {
                player: Player::Black,
                index: 322,
            },
        )
        .is_ok());
    assert!(board
        .set_move(
            &RuleSet::default(),
            &Move {
                player: Player::Black,
                index: 341,
            },
        )
        .is_ok());
    assert!(board
        .set_move(
            &RuleSet::default(),
            &Move {
                player: Player::Black,
                index: 360,
            },
        )
        .is_ok());
    let computer = Computer::new(&RuleSet::default(), &Player::Black);
    assert_eq!(computer.evaluate_board(&board, &Player::Black), 5000);
}

#[test]
fn find_pattern_live_two_diagonal_left_down() {
    let mut board = Board::default();
    assert!(board
        .set_move(
            &RuleSet::default(),
            &Move {
                player: Player::Black,
                index: 180,
            },
        )
        .is_ok());
    assert!(board
        .set_move(
            &RuleSet::default(),
            &Move {
                player: Player::Black,
                index: 216,
            },
        )
        .is_ok());
    let computer = Computer::new(&RuleSet::default(), &Player::Black);
    assert_eq!(computer.evaluate_board(&board, &Player::Black), 200);
}

#[test]
fn find_pattern_live_two_diagonal_right_down() {
    let mut board = Board::default();
    assert!(board
        .set_move(
            &RuleSet::default(),
            &Move {
                player: Player::Black,
                index: 180,
            },
        )
        .is_ok());
    assert!(board
        .set_move(
            &RuleSet::default(),
            &Move {
                player: Player::Black,
                index: 220,
            },
        )
        .is_ok());
    let computer = Computer::new(&RuleSet::default(), &Player::Black);
    assert_eq!(computer.evaluate_board(&board, &Player::Black), 200);
}

#[test]
fn find_pattern_live_two_diagonal_left_up() {
    let mut board = Board::default();
    assert!(board
        .set_move(
            &RuleSet::default(),
            &Move {
                player: Player::Black,
                index: 180,
            },
        )
        .is_ok());
    assert!(board
        .set_move(
            &RuleSet::default(),
            &Move {
                player: Player::Black,
                index: 140,
            },
        )
        .is_ok());
    let computer = Computer::new(&RuleSet::default(), &Player::Black);
    assert_eq!(computer.evaluate_board(&board, &Player::Black), 200);
}

#[test]
fn find_pattern_live_two_diagonal_right_up() {
    let mut board = Board::default();
    assert!(board
        .set_move(
            &RuleSet::default(),
            &Move {
                player: Player::Black,
                index: 180,
            },
        )
        .is_ok());
    assert!(board
        .set_move(
            &RuleSet::default(),
            &Move {
                player: Player::Black,
                index: 144,
            },
        )
        .is_ok());
    let computer = Computer::new(&RuleSet::default(), &Player::Black);
    assert_eq!(computer.evaluate_board(&board, &Player::Black), 200);
}

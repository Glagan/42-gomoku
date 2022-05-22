use gomoku::{
    board::{Board, Move},
    pattern::PATTERN_FINDER,
    player::Player,
    rules::RuleSet,
};

#[test]
// 0 0 0 0 0 0 0 0 0 1 0 1 0 0 0 0 0 0 0
fn find_pattern_live_two_horizontal() {
    let mut board = Board::default();
    let rules = &RuleSet::default();
    board.set_move(
        rules,
        &Move {
            player: Player::Black,
            index: 180,
        },
    );
    board.set_move(
        rules,
        &Move {
            player: Player::Black,
            index: 182,
        },
    );
    assert_eq!(PATTERN_FINDER.player_score(&board, &Player::Black), 200);
}

#[test]
// 0 0 0 0 0 0 0 0 0 1 1 1 0 0 0 0 0 0 0
fn find_pattern_live_three_horizontal() {
    let mut board = Board::default();
    let rules = &RuleSet::default();
    board.set_move(
        rules,
        &Move {
            player: Player::Black,
            index: 180,
        },
    );
    board.set_move(
        rules,
        &Move {
            player: Player::Black,
            index: 181,
        },
    );
    board.set_move(
        rules,
        &Move {
            player: Player::Black,
            index: 182,
        },
    );
    assert_eq!(PATTERN_FINDER.player_score(&board, &Player::Black), 15000);
}

#[test]
// 0 0 0 0 0 0 0 0 0 1 0 1 0 0 0 0 0 0 0
fn find_pattern_live_two_vertical() {
    let mut board = Board::default();
    let rules = &RuleSet::default();
    board.set_move(
        rules,
        &Move {
            player: Player::Black,
            index: 180,
        },
    );
    board.set_move(
        rules,
        &Move {
            player: Player::Black,
            index: 218,
        },
    );
    assert_eq!(PATTERN_FINDER.player_score(&board, &Player::Black), 200);
}

#[test]
// 0 0 0 0 0 0 0 0 0 1 1 1 0 0 0 0 0 0 0
fn find_pattern_live_three_vertical() {
    let mut board = Board::default();
    let rules = &RuleSet::default();
    board.set_move(
        rules,
        &Move {
            player: Player::Black,
            index: 180,
        },
    );
    board.set_move(
        rules,
        &Move {
            player: Player::Black,
            index: 199,
        },
    );
    board.set_move(
        rules,
        &Move {
            player: Player::Black,
            index: 218,
        },
    );
    assert_eq!(PATTERN_FINDER.player_score(&board, &Player::Black), 15000);
}

#[test]
// 0 0 0 0 0 0 0 0 0 1 1 1 0 0 0 0 0 0 0
fn find_pattern_live_three_vertical_border() {
    let mut board = Board::default();
    let rules = &RuleSet::default();
    board.set_move(
        rules,
        &Move {
            player: Player::Black,
            index: 322,
        },
    );
    board.set_move(
        rules,
        &Move {
            player: Player::Black,
            index: 341,
        },
    );
    board.set_move(
        rules,
        &Move {
            player: Player::Black,
            index: 360,
        },
    );
    assert_eq!(PATTERN_FINDER.player_score(&board, &Player::Black), 15000);
}

#[test]
// 0 0 0 0 0 0 0 0 0 1 0 1 0 0 0 0 0 0 0
fn find_pattern_live_two_diagonal_left_down() {
    let mut board = Board::default();
    let rules = &RuleSet::default();
    board.set_move(
        rules,
        &Move {
            player: Player::Black,
            index: 180,
        },
    );
    board.set_move(
        rules,
        &Move {
            player: Player::Black,
            index: 216,
        },
    );
    assert_eq!(PATTERN_FINDER.player_score(&board, &Player::Black), 200);
}

#[test]
// 0 0 0 0 0 0 0 0 0 1 0 1 0 0 0 0 0 0 0
fn find_pattern_live_two_diagonal_right_down() {
    let mut board = Board::default();
    let rules = &RuleSet::default();
    board.set_move(
        rules,
        &Move {
            player: Player::Black,
            index: 180,
        },
    );
    board.set_move(
        rules,
        &Move {
            player: Player::Black,
            index: 220,
        },
    );
    assert_eq!(PATTERN_FINDER.player_score(&board, &Player::Black), 200);
}

#[test]
// 0 0 0 0 0 0 0 0 0 1 0 1 0 0 0 0 0 0 0
fn find_pattern_live_two_diagonal_left_up() {
    let mut board = Board::default();
    let rules = &RuleSet::default();
    board.set_move(
        rules,
        &Move {
            player: Player::Black,
            index: 180,
        },
    );
    board.set_move(
        rules,
        &Move {
            player: Player::Black,
            index: 140,
        },
    );
    assert_eq!(PATTERN_FINDER.player_score(&board, &Player::Black), 200);
}

#[test]
// 0 0 0 0 0 0 0 0 0 1 0 1 0 0 0 0 0 0 0
fn find_pattern_live_two_diagonal_right_up() {
    let mut board = Board::default();
    let rules = &RuleSet::default();
    board.set_move(
        rules,
        &Move {
            player: Player::Black,
            index: 180,
        },
    );
    board.set_move(
        rules,
        &Move {
            player: Player::Black,
            index: 144,
        },
    );
    assert_eq!(PATTERN_FINDER.player_score(&board, &Player::Black), 200);
}
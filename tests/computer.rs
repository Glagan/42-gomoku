use gomoku::{
    board::{Board, Move},
    computer::SortedMove,
    pattern::{PatternCategory, PATTERN_FINDER},
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
    assert_eq!(
        PATTERN_FINDER
            .count_patterns(&board, &Player::Black)
            .live_three,
        1
    );
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
    assert_eq!(
        PATTERN_FINDER
            .count_patterns(&board, &Player::Black)
            .live_three,
        1
    );
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
    assert_eq!(
        PATTERN_FINDER
            .count_patterns(&board, &Player::Black)
            .live_three,
        1
    );
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
    assert_eq!(
        PATTERN_FINDER
            .count_patterns(&board, &Player::Black)
            .live_three,
        1
    );
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
    assert_eq!(
        PATTERN_FINDER
            .count_patterns(&board, &Player::Black)
            .live_three,
        1
    );
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
    assert_eq!(
        PATTERN_FINDER
            .count_patterns(&board, &Player::Black)
            .live_two,
        1
    );
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
    assert_eq!(
        PATTERN_FINDER
            .count_patterns(&board, &Player::Black)
            .live_two,
        1
    );
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
    assert_eq!(
        PATTERN_FINDER
            .count_patterns(&board, &Player::Black)
            .live_two,
        1
    );
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
    assert_eq!(
        PATTERN_FINDER
            .count_patterns(&board, &Player::Black)
            .live_two,
        1
    );
}

#[test]
fn option_pattern_category_eq() {
    let sorted_move_1 = SortedMove {
        movement: Move {
            player: Player::Black,
            index: 0,
        },
        pattern: Some(PatternCategory::FiveInRow),
    };
    let sorted_move_2 = SortedMove {
        movement: Move {
            player: Player::Black,
            index: 0,
        },
        pattern: Some(PatternCategory::FiveInRow),
    };
    assert_eq!(sorted_move_1, sorted_move_2);

    let sorted_move_2 = SortedMove {
        movement: Move {
            player: Player::Black,
            index: 0,
        },
        pattern: Some(PatternCategory::LiveFour),
    };
    assert_ne!(sorted_move_1, sorted_move_2);

    let sorted_move_2 = SortedMove {
        movement: Move {
            player: Player::Black,
            index: 0,
        },
        pattern: None,
    };
    assert_ne!(sorted_move_1, sorted_move_2);

    let sorted_move_2 = SortedMove {
        movement: Move {
            player: Player::Black,
            index: 0,
        },
        pattern: None,
    };
    assert_eq!(sorted_move_2, sorted_move_2);
}

#[test]
fn option_pattern_category_cmp_1() {
    let sorted_move_1 = SortedMove {
        movement: Move {
            player: Player::Black,
            index: 0,
        },
        pattern: Some(PatternCategory::FiveInRow),
    };
    let sorted_move_2 = SortedMove {
        movement: Move {
            player: Player::Black,
            index: 0,
        },
        pattern: Some(PatternCategory::LiveFour),
    };
    assert!(sorted_move_1 > sorted_move_2);
    assert!(sorted_move_1 >= sorted_move_2);
    assert!(sorted_move_1 >= sorted_move_1);
    assert!(sorted_move_2 < sorted_move_1);
    assert!(sorted_move_2 <= sorted_move_1);
    assert!(sorted_move_2 <= sorted_move_2);

    let sorted_move_2 = SortedMove {
        movement: Move {
            player: Player::Black,
            index: 0,
        },
        pattern: None,
    };
    assert!(sorted_move_1 > sorted_move_2);
    assert!(sorted_move_1 >= sorted_move_2);
    assert!(sorted_move_1 >= sorted_move_1);
    assert!(sorted_move_2 < sorted_move_1);
    assert!(sorted_move_2 <= sorted_move_1);
    assert!(sorted_move_2 <= sorted_move_2);
}

#[test]
fn option_pattern_category_cmp_2() {
    let sorted_move_1 = SortedMove {
        movement: Move {
            player: Player::Black,
            index: 0,
        },
        pattern: Some(PatternCategory::LiveThree),
    };
    let sorted_move_2 = SortedMove {
        movement: Move {
            player: Player::Black,
            index: 0,
        },
        pattern: Some(PatternCategory::DeadTwo),
    };
    assert!(sorted_move_1 > sorted_move_2);
    assert!(sorted_move_1 >= sorted_move_2);
    assert!(sorted_move_1 >= sorted_move_1);
    assert!(sorted_move_2 < sorted_move_1);
    assert!(sorted_move_2 <= sorted_move_1);
    assert!(sorted_move_2 <= sorted_move_2);

    let sorted_move_2 = SortedMove {
        movement: Move {
            player: Player::Black,
            index: 0,
        },
        pattern: None,
    };
    assert!(sorted_move_1 > sorted_move_2);
    assert!(sorted_move_1 >= sorted_move_2);
    assert!(sorted_move_1 >= sorted_move_1);
    assert!(sorted_move_2 < sorted_move_1);
    assert!(sorted_move_2 <= sorted_move_1);
    assert!(sorted_move_2 <= sorted_move_2);
}

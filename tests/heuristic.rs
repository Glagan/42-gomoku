use gomoku::{
    board::{Board, Coordinates, Move},
    constants::{BOARD_SIZE, DIRECTIONS, OPPOSITE_DIRECTIONS},
    heuristic::HEURISTIC,
    patterns::{Category, PatternCount},
    player::Player,
    rock::{PlayerRock, Rock},
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

// * Simple patterns

#[test]
// 0 0 0 0 0 0 0 0 0 1 0 1 0 0 0 0 0 0 0
fn find_pattern_open_two_horizontal() {
    let mut board = Board::default();

    let moves = vec![(CENTER.x - 1, CENTER.y), (CENTER.x + 1, CENTER.y)];
    set_many!(mut board, Player::Black, moves[0], moves[1]);

    assert!(moves.iter().all(|coordinates| {
        HEURISTIC
            .count_movement_patterns(
                &RuleSet::default(),
                &board,
                &Move {
                    player: Player::Black,
                    coordinates: coord!(coordinates.0, coordinates.1),
                },
                0,
            )
            .open_two
            == 1
    }));
}

#[test]
// 0 0 0 0 0 0 0 0 0 1 0 1 0 0 0 0 0 0 0
fn find_pattern_open_two_vertical() {
    let mut board = Board::default();

    let moves = vec![(CENTER.x, CENTER.y), (CENTER.x, CENTER.y - 2)];
    set_many!(mut board, Player::Black, moves[0], moves[1]);

    assert!(moves.iter().all(|coordinates| {
        HEURISTIC
            .count_movement_patterns(
                &RuleSet::default(),
                &board,
                &Move {
                    player: Player::Black,
                    coordinates: coord!(coordinates.0, coordinates.1),
                },
                0,
            )
            .open_two
            == 1
    }));
}

#[test]
// 0 0 0 0 0 0 0 0 0 1 0 1 0 0 0 0 0 0 0
fn find_pattern_open_two_diagonal_left_up() {
    let mut board = Board::default();

    let moves = vec![(CENTER.x, CENTER.y), (CENTER.x - 2, CENTER.y - 2)];
    set_many!(mut board, Player::Black, moves[0], moves[1]);

    assert!(moves.iter().all(|coordinates| {
        HEURISTIC
            .count_movement_patterns(
                &RuleSet::default(),
                &board,
                &Move {
                    player: Player::Black,
                    coordinates: coord!(coordinates.0, coordinates.1),
                },
                0,
            )
            .open_two
            == 1
    }));
}

#[test]
// 0 0 0 0 0 0 0 0 0 1 0 1 0 0 0 0 0 0 0
fn find_pattern_open_two_diagonal_left_down() {
    let mut board = Board::default();

    let moves = vec![(CENTER.x, CENTER.y), (CENTER.x + 2, CENTER.y + 2)];
    set_many!(mut board, Player::Black, moves[0], moves[1]);

    assert!(moves.iter().all(|coordinates| {
        HEURISTIC
            .count_movement_patterns(
                &RuleSet::default(),
                &board,
                &Move {
                    player: Player::Black,
                    coordinates: coord!(coordinates.0, coordinates.1),
                },
                0,
            )
            .open_two
            == 1
    }));
}

#[test]
// 0 0 0 0 0 0 0 0 0 1 0 1 0 0 0 0 0 0 0
fn find_pattern_open_two_diagonal_right_up() {
    let mut board = Board::default();

    let moves = vec![(CENTER.x, CENTER.y), (CENTER.x + 2, CENTER.y - 2)];
    set_many!(mut board, Player::Black, moves[0], moves[1]);

    assert!(moves.iter().all(|coordinates| {
        HEURISTIC
            .count_movement_patterns(
                &RuleSet::default(),
                &board,
                &Move {
                    player: Player::Black,
                    coordinates: coord!(coordinates.0, coordinates.1),
                },
                0,
            )
            .open_two
            == 1
    }));
}

#[test]
// 0 0 0 0 0 0 0 0 0 1 0 1 0 0 0 0 0 0 0
fn find_pattern_open_two_diagonal_right_down() {
    let mut board = Board::default();

    let moves = vec![(CENTER.x, CENTER.y), (CENTER.x - 2, CENTER.y + 2)];
    set_many!(mut board, Player::Black, moves[0], moves[1]);

    assert!(moves.iter().all(|coordinates| {
        HEURISTIC
            .count_movement_patterns(
                &RuleSet::default(),
                &board,
                &Move {
                    player: Player::Black,
                    coordinates: coord!(coordinates.0, coordinates.1),
                },
                0,
            )
            .open_two
            == 1
    }));
}

#[test]
// 0 0 0 0 0 0 0 0 0 1 1 1 0 0 0 0 0 0 0
fn find_pattern_open_three_horizontal() {
    let mut board = Board::default();

    let moves = vec![
        (CENTER.x - 1, CENTER.y),
        (CENTER.x, CENTER.y),
        (CENTER.x + 1, CENTER.y),
    ];
    set_many!(mut board, Player::Black, moves[0], moves[1], moves[2]);

    assert!(moves.iter().all(|coordinates| {
        HEURISTIC
            .count_movement_patterns(
                &RuleSet::default(),
                &board,
                &Move {
                    player: Player::Black,
                    coordinates: coord!(coordinates.0, coordinates.1),
                },
                0,
            )
            .open_three
            >= 1
    }));
}

#[test]
// 0 0 0 0 0 0 0 0 0 1 1 1 0 0 0 0 0 0 0
fn find_pattern_open_three_vertical() {
    let mut board = Board::default();

    let moves = vec![
        (CENTER.x, CENTER.y - 1),
        (CENTER.x, CENTER.y),
        (CENTER.x, CENTER.y + 1),
    ];
    set_many!(mut board, Player::Black, moves[0], moves[1], moves[2]);

    assert!(moves.iter().all(|coordinates| {
        HEURISTIC
            .count_movement_patterns(
                &RuleSet::default(),
                &board,
                &Move {
                    player: Player::Black,
                    coordinates: coord!(coordinates.0, coordinates.1),
                },
                0,
            )
            .open_three
            >= 1
    }));
}

#[test]
// 0 0 0 0 0 0 0 0 0 1 1 1 0 0 0 0 0 0 0
fn find_pattern_open_three_vertical_border() {
    let mut board = Board::default();

    let moves = vec![
        (BORDER, BORDER - 3),
        (BORDER, BORDER - 2),
        (BORDER, BORDER - 1),
    ];
    set_many!(mut board, Player::Black, moves[0], moves[1], moves[2]);

    println!("{}", board);

    assert!(moves.iter().all(|coordinates| {
        HEURISTIC
            .count_movement_patterns(
                &RuleSet::default(),
                &board,
                &Move {
                    player: Player::Black,
                    coordinates: coord!(coordinates.0, coordinates.1),
                },
                0,
            )
            .open_three
            >= 1
    }));
}

// * Check that all patterns increase their category in PatternCount when found

fn construct_board_from_pattern(direction: &(i16, i16), pattern: &Vec<(i16, PlayerRock)>) -> Board {
    let mut board = Board::default();

    for (mov, rock) in pattern {
        if rock != &PlayerRock::None {
            board.set_on_boards(
                &coord!(CENTER.x + direction.0 * mov, CENTER.y + direction.1 * mov),
                match rock {
                    PlayerRock::Opponent => Player::White,
                    _ => Player::Black,
                },
            );
        }
    }
    board.set_on_boards(&CENTER, Player::Black);
    board
}

fn construct_all_boards_from_pattern(pattern: &Vec<(i16, PlayerRock)>) -> Vec<Board> {
    DIRECTIONS
        .iter()
        .map(|direction| construct_board_from_pattern(direction, pattern))
        .collect()
}

fn pattern_category_increase_count(category: Category) -> bool {
    HEURISTIC
        .patterns
        .iter()
        .filter(|pattern| pattern.1 == category)
        .all(|(pattern, _)| {
            construct_all_boards_from_pattern(pattern)
                .iter()
                .all(|board| {
                    let patterns = HEURISTIC.count_movement_patterns(
                        &RuleSet::default(),
                        board,
                        &Move {
                            player: Player::Black,
                            coordinates: coord!(CENTER.x, CENTER.y),
                        },
                        0,
                    );
                    if (match category {
                        Category::FiveInRow => patterns.five_in_row,
                        Category::KillFour => patterns.kill_four,
                        Category::OpenFour => patterns.open_four,
                        Category::ReduceThree => patterns.reduce_three,
                        Category::CloseFour => patterns.close_four,
                        Category::OpenThree => patterns.open_three,
                        Category::KillThree => patterns.kill_three,
                        Category::BlockedCapture => patterns.blocked_capture,
                        Category::CapturedFiveInRow => patterns.captured_five_in_row,
                        Category::CloseThree => patterns.close_three,
                        Category::OpenTwo => patterns.open_two,
                        Category::ReduceTwo => patterns.reduce_two,
                        Category::CreateCapture => patterns.created_captures,
                        Category::RevealCapture => patterns.reveal_capture,
                        // Category::CloseTwo => patterns.close_two,
                    }) >= 1
                    {
                        true
                    } else {
                        println!("{:#?}\n{}", patterns, board);
                        false
                    }
                })
        })
}

#[test]
fn find_all_five_in_row() {
    assert!(pattern_category_increase_count(Category::FiveInRow));
}

#[test]
fn find_all_kill_four() {
    assert!(pattern_category_increase_count(Category::KillFour));
}

#[test]
fn find_all_open_four() {
    assert!(pattern_category_increase_count(Category::OpenFour));
}

#[test]
fn find_all_reduce_three() {
    assert!(pattern_category_increase_count(Category::ReduceThree));
}

#[test]
fn find_all_close_four() {
    // ! The last 2 OpenFour patterns (variations) are under capture
    HEURISTIC
        .patterns
        .iter()
        .filter(|pattern| pattern.1 == Category::CloseFour)
        .all(|(pattern, _)| {
            construct_all_boards_from_pattern(pattern)
                .iter()
                .all(|board| {
                    let patterns = HEURISTIC.count_movement_patterns(
                        &RuleSet::default(),
                        board,
                        &Move {
                            player: Player::Black,
                            coordinates: coord!(CENTER.x, CENTER.y),
                        },
                        0,
                    );
                    patterns.close_four >= 1 /*  || patterns.close_three >= 1 */
                })
        });
}

#[test]
fn find_all_open_three() {
    assert!(pattern_category_increase_count(Category::OpenThree));
}

#[test]
fn find_all_kill_three() {
    assert!(pattern_category_increase_count(Category::KillThree));
}

#[test]
fn find_all_blocked_capture() {
    assert!(pattern_category_increase_count(Category::BlockedCapture));
}

#[test]
fn find_all_captured_five_in_row() {
    assert!(pattern_category_increase_count(Category::CapturedFiveInRow));
}

#[test]
fn find_all_close_three() {
    assert!(pattern_category_increase_count(Category::CloseThree));
}

#[test]
fn find_all_open_two() {
    assert!(pattern_category_increase_count(Category::OpenTwo));
}

#[test]
fn find_all_reduce_two() {
    assert!(pattern_category_increase_count(Category::ReduceTwo));
}

// #[test]
// fn find_all_close_two() {
//     assert!(pattern_category_increase_count(Category::CloseTwo));
// }

#[test]
fn find_create_capture() {
    assert!(pattern_category_increase_count(Category::CreateCapture));
}

#[test]
fn find_reveal_capture() {
    assert!(pattern_category_increase_count(Category::RevealCapture));
}

// * Check that all patterns under capture are downgraded

fn under_capture_patterns_for_coordinates(
    board: &Board,
    opposite_directions: ((i16, i16), (i16, i16)),
    coordinates: Coordinates,
) -> Vec<PatternCount> {
    let mut all_patterns = vec![];
    let (left, right) = opposite_directions;
    // Left
    {
        // Add the player rock and opponent rock
        let mut board = board.clone();
        board.set_move(
            &RuleSet::default(),
            &Move {
                player: Player::Black,
                coordinates: coord!(coordinates.x + left.0, coordinates.y + left.1),
            },
        );
        board.set_move(
            &RuleSet::default(),
            &Move {
                player: Player::White,
                coordinates: coord!(coordinates.x + left.0 * 2, coordinates.y + left.1 * 2),
            },
        );
        // Check that the pattern is under capture
        all_patterns.push(HEURISTIC.count_movement_patterns(
            &RuleSet::default(),
            &board,
            &Move {
                player: Player::Black,
                coordinates: coord!(CENTER.x, CENTER.y),
            },
            0,
        ));
    }
    // Right
    {
        // Add the player rock and opponent rock
        let mut board = board.clone();
        board.set_move(
            &RuleSet::default(),
            &Move {
                player: Player::Black,
                coordinates: coord!(coordinates.x + right.0, coordinates.y + right.1),
            },
        );
        board.set_move(
            &RuleSet::default(),
            &Move {
                player: Player::White,
                coordinates: coord!(coordinates.x + right.0 * 2, coordinates.y + right.1 * 2),
            },
        );
        // Check that the pattern is under capture
        all_patterns.push(HEURISTIC.count_movement_patterns(
            &RuleSet::default(),
            &board,
            &Move {
                player: Player::Black,
                coordinates: coord!(CENTER.x, CENTER.y),
            },
            0,
        ));
    }
    all_patterns
}

#[test]
fn downgrade_capture_five_in_a_row() {
    let pattern_and_boards = HEURISTIC
        .patterns
        .iter()
        .filter(|pattern| pattern.1 == Category::FiveInRow)
        .map(|(pattern, _)| (pattern, construct_all_boards_from_pattern(pattern)))
        .collect::<Vec<(&Vec<(i16, PlayerRock)>, Vec<Board>)>>();

    // For each boards associated with each patterns
    // -- generate all block capture in all positions
    for (pattern, boards) in pattern_and_boards {
        for opposite_directions in OPPOSITE_DIRECTIONS {
            for (index, direction) in DIRECTIONS.iter().enumerate() {
                let board = boards.get(index).unwrap();
                // Check the center rock ...
                // Ignore directions that goes to the pattern itself
                if board.get(
                    CENTER.x + opposite_directions.0 .0,
                    CENTER.y + opposite_directions.0 .1,
                ) == Rock::None
                    && board.get(
                        CENTER.x + opposite_directions.1 .0,
                        CENTER.y + opposite_directions.1 .1,
                    ) == Rock::None
                {
                    assert!(under_capture_patterns_for_coordinates(
                        board,
                        opposite_directions,
                        CENTER
                    )
                    .iter()
                    .all(|patterns| {
                        patterns.five_in_row == 0 && patterns.captured_five_in_row >= 1
                    }));
                }
                // ... and all other rocks in the pattern
                for (mov, _) in pattern.iter() {
                    let coordinates =
                        coord!(CENTER.x + mov * direction.0, CENTER.y + mov * direction.1);
                    // Ignore directions that goes to the pattern itself
                    if board.get(
                        coordinates.x + opposite_directions.0 .0,
                        coordinates.y + opposite_directions.0 .1,
                    ) == Rock::None
                        && board.get(
                            coordinates.x + opposite_directions.1 .0,
                            coordinates.y + opposite_directions.1 .1,
                        ) == Rock::None
                    {
                        assert!(under_capture_patterns_for_coordinates(
                            board,
                            opposite_directions,
                            CENTER
                        )
                        .iter()
                        .all(|patterns| {
                            patterns.five_in_row == 0 && patterns.captured_five_in_row >= 1
                        }));
                    }
                }
            }
        }
    }
}

#[test]
fn downgrade_open_four() {
    // Only the direct four in a row pattern need to be checked
    let open_four_patterns = vec![
        (
            vec![
                (-1, PlayerRock::None),
                (1, PlayerRock::Player),
                (2, PlayerRock::Player),
                (3, PlayerRock::Player),
                (4, PlayerRock::None),
            ],
            Category::OpenFour,
        ),
        (
            vec![
                (-2, PlayerRock::None),
                (-1, PlayerRock::Player),
                (1, PlayerRock::Player),
                (2, PlayerRock::Player),
                (3, PlayerRock::None),
            ],
            Category::OpenFour,
        ),
    ];
    let pattern_and_boards = open_four_patterns
        .iter()
        .map(|(pattern, _)| (pattern, construct_all_boards_from_pattern(pattern)))
        .collect::<Vec<(&Vec<(i16, PlayerRock)>, Vec<Board>)>>();

    // For each boards associated with each patterns
    // -- generate all block capture in all positions
    for (pattern, boards) in pattern_and_boards {
        for opposite_directions in OPPOSITE_DIRECTIONS {
            for (index, direction) in DIRECTIONS.iter().enumerate() {
                let board = boards.get(index).unwrap();
                // Check the center rock ...
                // Ignore directions that goes to the pattern itself
                if board.get(
                    CENTER.x + opposite_directions.0 .0,
                    CENTER.y + opposite_directions.0 .1,
                ) == Rock::None
                    && board.get(
                        CENTER.x + opposite_directions.1 .0,
                        CENTER.y + opposite_directions.1 .1,
                    ) == Rock::None
                {
                    assert!(under_capture_patterns_for_coordinates(
                        board,
                        opposite_directions,
                        CENTER
                    )
                    .iter()
                    .all(|patterns| { patterns.open_four == 0 && patterns.close_three >= 1 }));
                }
                // ... and all other rocks in the pattern
                for (mov, _) in pattern.iter() {
                    let coordinates =
                        coord!(CENTER.x + mov * direction.0, CENTER.y + mov * direction.1);
                    // Ignore directions that goes to the pattern itself
                    if board.get(
                        coordinates.x + opposite_directions.0 .0,
                        coordinates.y + opposite_directions.0 .1,
                    ) == Rock::None
                        && board.get(
                            coordinates.x + opposite_directions.1 .0,
                            coordinates.y + opposite_directions.1 .1,
                        ) == Rock::None
                    {
                        assert!(under_capture_patterns_for_coordinates(
                            board,
                            opposite_directions,
                            CENTER
                        )
                        .iter()
                        .all(|patterns| { patterns.open_four == 0 && patterns.close_three >= 1 }));
                    }
                }
            }
        }
    }
}

use gomoku::{
    board::{Board, Coordinates, Move},
    constants::BOARD_SIZE,
    heuristic::HEURISTIC,
    player::Player,
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

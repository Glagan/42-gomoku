use board::Board;

use crate::board::{Move, Pawn, Player};

mod board;
mod computer;
mod game;
mod interface;

fn main() {
    let mut board = Board::default();
    println!("{}", board);
    board.set_move(&Move {
        index: 0,
        player: Player::Black,
    });
    board.set_move(&Move {
        index: 19,
        player: Player::White,
    });
    board.set_move(&Move {
        index: 20,
        player: Player::Black,
    });
    board.set_move(&Move {
        index: 360,
        player: Player::White,
    });
    board.set_move(&Move {
        index: 361,
        player: Player::Black,
    });
    println!("---");
    println!("{}", board);
}

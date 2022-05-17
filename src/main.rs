use board::Board;

use crate::{
    board::{Move, Pawn},
    computer::Computer,
    player::Player,
};

mod board;
mod computer;
mod game;
mod interface;
mod player;

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
    println!("---");
    let player = Player::Black;
    let computer = Computer::new(&player);
    let play_result = computer.play(&board, 4);
    println!("play: {:#?}", play_result);
}

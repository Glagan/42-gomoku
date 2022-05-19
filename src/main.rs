use std::time::Instant;

use board::Board;

use crate::{
    board::{Move, Pawn},
    computer::Computer,
    player::Player,
    rules::RuleSet,
};

mod board;
mod computer;
mod game;
mod interface;
mod player;
mod rules;

fn main() {
    let rules = RuleSet::default();
    let mut board = Board::default();
    // println!("{}", board);
    // board.set_move(
    //     &rules,
    //     &Move {
    //         index: 0,
    //         player: Player::Black,
    //     },
    // );
    // board.set_move(
    //     &rules,
    //     &Move {
    //         index: 19,
    //         player: Player::White,
    //     },
    // );
    // board.set_move(
    //     &rules,
    //     &Move {
    //         index: 20,
    //         player: Player::Black,
    //     },
    // );
    // board.set_move(
    //     &rules,
    //     &Move {
    //         index: 360,
    //         player: Player::White,
    //     },
    // );
    // board.set_move(
    //     &rules,
    //     &Move {
    //         index: 361,
    //         player: Player::Black,
    //     },
    // );
    // println!("---");
    println!("{}", board);
    println!("---");
    let player1 = Player::Black;
    let player2 = Player::White;
    let computer1 = Computer::new(&rules, &player1);
    let computer2 = Computer::new(&rules, &player2);
    for i in 0..20 {
        let now = Instant::now();
        if i % 2 == 0 {
            let play_result = computer1.play(&board, 4);
            println!("play: {:#?}", play_result);
            if let Ok(play) = play_result {
                if let Some(movement) = play.movement {
                    board.set_move(&rules, &movement);
                }
            }
        } else {
            let play_result = computer2.play(&board, 4);
            println!("play: {:#?}", play_result);
            if let Ok(play) = play_result {
                if let Some(movement) = play.movement {
                    board.set_move(&rules, &movement);
                }
            }
        }
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
        println!("{}", board);
        println!("---");
    }
}

use board::Board;

mod board;

fn main() {
    let board = Board::default();
    println!("{}", board);
}

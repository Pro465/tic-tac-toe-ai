//==driver code==

mod board;
use board::Board;

fn main() {
  let mut b = Board::new();
  println!("Welcome to my Tic Tac Toe In Rust😀");
  
  b.print_board();
  
  b.start();
}

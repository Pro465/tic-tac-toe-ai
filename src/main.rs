//==driver code==

use tic_tac_toe_ai::Board;

fn main() {
  let mut b = Board::new();
  println!("Welcome to my Tic Tac Toe In Rust😀");
  
  b.print_board();
  
  b.start();
}

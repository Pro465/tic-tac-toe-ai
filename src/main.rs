//rust 1.30.0 
mod board;
use board::Board;

fn main() {
  let mut b = Board::new();
  println!("Welcome to my Tic Tac Toe In Rust😀");
  
  println!("This is my rust+OOP version of my JS+Functional Programming attempt in making a Tic Tac Toe AI");
  
  b.print_board();
  
  b.start();
}

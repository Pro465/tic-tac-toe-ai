//==The class where the game's important methods are stored==
  
pub struct Board{
  human_first: bool,
  curr: bool,
  board: [u8;9],
}

//constructor impl
impl Board{
//=========
  
  //constructor
  pub fn new() -> Board
  {
    Board{
      human_first: true,
      curr: true,
      board: [2;9]
    }
  }
}

//all the important methods are here
impl Board{
//=========
    
  //to start the game
  pub fn start(&mut self)
  {
    self.play();
  }
  
  //to print the board
  fn print_board(&self)
  {
    for i in 0..3
    {
      println!("{} | {} | {}",
      if self.board[i*3] != 2
      {
        if self.board[i*3] == 0 {
          "O"
        }
        else{
          "X"
        }
      }
      else {
        " "
      },
      if self.board[i*3+1] != 2
      {
        if self.board[i*3+1] == 0 {
          "O"
        }
        else {
          "X"
        }
      }
      else {
        " "
      },
      if self.board[i*3+2] != 2
      {
        if self.board[i*3+2] == 0 {
          "O"
        }
        else {
          "X"
        }
      }
      else {
        " "
      });
    }
  }
  
  //to announce who chose which move
  fn announce(&mut self, pos: u8)
  {
    println!("{} chose position {}",
    if self.curr {"you"} else {"ai"} ,if self.curr {pos} else {pos+1});
    self.print_board();
    self.curr = !self.curr;
    self.play();
  }
  
  //checks the board & returns true if there's any winner
  fn check(board : [u8;9]) -> bool
  {
    ((board[0]==0 && board[1]==0 
    && board[2]==0)
    || 
    (board[3]==0 && board[4]==0 
    && board[5]==0)
    ||
    (board[6]==0 && board[7]==0 
    && board[8]==0) 
    ||
    (board[0]==0 && board[3]==0 
    && board[6]==0)
    || 
    (board[1]==0 && board[4]==0 
    && board[7]==0)
    ||
    (board[2]==0 && board[5]==0 
    && board[8]==0)
    ||
    (board[0]==0 && board[4]==0 
    && board[8]==0)
    || 
    (board[2]==0 && board[4]==0 
    && board[6]==0))
    ||
    ((board[0]==1 && board[1]==1 
    && board[2]==1)
    || 
    (board[3]==1 && board[4]==1 
    && board[5]==1)
    ||
    (board[6]==1 && board[7]==1
    && board[8]==1) 
    ||
    (board[0]==1 && board[3]==1 
    && board[6]==1)
    || 
    (board[1]==1 && board[4]==1 
    && board[7]==1)
    ||
    (board[2]==1 && board[5]==1 
    && board[8]==1)
    ||
    (board[0]==1 && board[4]==1 
    && board[8]==1)
    || 
    (board[2]==1 && board[4]==1 
    && board[6]==1))
  }
  
  //to reset the game state
  fn reset(&mut self)
  {
    self.board = [2;9];
    self.human_first = !self.human_first;
    self.curr = self.human_first;
  }
  
  //to let the current player (human or ai) play
  fn play(&mut self) 
  {
    let mut draw = true;
    for i in self.board.iter()
    {
      if *i==2u8
      {
        draw = false;
        break;
      }
    }
    if Board::check(self.board) || draw
    {
      if draw
      {
        println!("draw!");
      }
      else
      {
        if self.curr
        {
          println!("ai won!");
        }
        else
        {
          println!("you won!");
        }
      }
      self.reset();
      self.play();
      return;
    }
    if self.curr
    {
      let p: u8 = input(Some("please choose your position: "))
      .parse()
      .expect("Error");
      if self.board[(p-1) as usize]==2
      {
        self.board[(p-1) as usize] = if self.human_first {1} else {0};
        self.announce(p);
      }
      else
      {
        self.play();
      }
    }
    else
    {
      let p: u8 = self.minimax(self.board,true,-2,2)[0] as u8;
      self.board[p as usize] = if !self.human_first {1} else {0};
      self.announce(p);
    }
  }
  
  //to decide the best move for the ai
  fn minimax(&self,board: [u8;9],is_ai: bool,mut alpha: i8,mut beta: i8) -> [i8;2]
  {
    let mut best : [i8;2] = if is_ai {[-1,-2]} else {[-1,2]};
    
    let min_or_max = |x: [i8;2],y: [i8;2]| {
        if is_ai && y[1] < x[1] {
          x
        }
        else if !is_ai && y[1] > x[1] {
          x
        }
        else {
          y
        }
    };
    
    for i in 0..9
    {
      if board[i]!=2
      {
        continue;
      }
      let mut bcp = board;
      bcp[i] = if is_ai {if self.human_first {0} else {1}} else {if self.human_first {1} else {0}};
      if Board::check(bcp)
      {
        let b = min_or_max([i as i8,if is_ai {1} else {-1}],best);
        best = b;
      }
      else
      {
        let score = self.minimax(bcp,!is_ai,alpha,beta)[1];
        
        let b = min_or_max([i as i8,score],best);
        
        best = b;
        
        if is_ai
        {
          alpha = if score > alpha {score} else {alpha};
        }
        else
        {
          beta = if score < beta {score} else {beta};
        }
        
        if alpha >= beta
        {
          break;
        }
      }
    }
    
    if best[0]==-1 {
      [0,0]
    } 
    else {
      best
    }
 }
}
//==end of Board impl==

//function to get input
fn input(string_to_be_printed: Option<&str>) -> String {
  use std::io::Write;
  
  if let Some(x) = string_to_be_printed{
    print!("{}",x);
    std::io::stdout()
    .flush()
    .unwrap ();
  }
  
  let mut s = String::new();
  
  std::io::stdin()
     .read_line(&mut s)
     .expect("Error");
  
  String::from(&s[..s.len()-1])
}

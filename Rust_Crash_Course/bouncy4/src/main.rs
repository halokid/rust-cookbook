
mod mutx;
mod les4_1;
mod les5_1;
mod les5_1_1;
mod les5_1_2;
mod bibao;
mod movex;

use std::fmt::{Display, Formatter};
use std::slice::from_raw_parts;

enum VerDir {
  Up,
  Down,
}

enum HorizDir {
  Left,
  Right,
}

struct Ball {
  x:           u32,
  y:           u32,
  vert_dir:    VerDir,
  horiz_dir:   HorizDir,
}

struct Frame {
  width:    u32,
  height:   u32,
}

struct Game {
  frame:    Frame,
  ball:     Ball,
}


impl Game {
  fn new() -> Game {
    let frame = Frame {
      width:    60,
      height:   30,
    };

    let ball = Ball {
      x:  2,
      y:  4,
      vert_dir:   VerDir::Up,
      horiz_dir:  HorizDir::Left,
    };

    Game { frame, ball }
  }

  fn step(&mut self) {
    self.ball.bounce(&self.frame);
    self.ball.mv();
  }

}

impl Ball {
  fn bounce(&mut self, frame: &Frame) {
    if self.x == 0 {
      self.horiz_dir = HorizDir::Right
    } else if self.x == frame.width - 1 {
      self.horiz_dir = HorizDir::Left;
    }

    if self.y == 0 {
      self.vert_dir = VerDir::Down;
    } else if self.y == frame.height - 1 {
      self.vert_dir = VerDir::Up;
    }
  }

  fn mv(&mut self) {
    match self.horiz_dir {
      HorizDir::Left => self.x -= 1,
      HorizDir::Right => self.x += 1,
    }

    match self.vert_dir {
      VerDir::Up => self.y -= 1,
      VerDir::Down => self.y += 1,
    }
  }
}

impl Display for Game {

  fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {

    // todo: lambda 的语法
    let top_bottom = | fmt: &mut Formatter | {
      write!(fmt, "+");
      for _ in 0..self.frame.width {
        write!(fmt, "-");
      }
      write!(fmt, "+\n")
    };      // todo: 定义一个闭包函数变量

    top_bottom(fmt)?;

    for row in 0..self.frame.height {
      write!(fmt, "|");

      for column in 0..self.frame.width {
        let c = if row == self.ball.y && column == self.ball.x {
          'o'
        } else {
          ' '
        };
        write!(fmt, "{}", c);
      }

      write!(fmt, "|\n");
    }

    top_bottom(fmt)
  }
}


fn main() {
  /*
  let mut game = Game::new();
  let sleep_duration = std::time::Duration::from_millis(33);
  loop {
    println!("{}", game);
    game.step();
    std::thread::sleep(sleep_duration);
  }
   */

  // mutx::comm();

  // les4_1::comm();

  // les5_1::comm();

  // les5_1_1::comm();

  // les5_1_2::comm();

  // bibao::comm();
  // bibao::comm2();
  // bibao::comm4();
  // bibao::comm5();

  // movex::comm1();
  // movex::comm2();
  movex::comm3();
}












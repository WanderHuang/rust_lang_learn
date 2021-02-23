/// 学习
/// trait
/// struct
/// etc


/// 定义移动
pub trait Move {
  fn step(&mut self, x: i32, y: i32) -> bool;
  fn step3(&mut self, x: i32, y: i32, z: i32) -> bool;
}

pub trait Cry<T> {
  fn cry(self, to: T) -> String;
}

/// 定义人
pub struct Man {
  pub name: &'static str,
  pub x: i32,
  pub y: i32,
  pub z: i32
}

impl Move for Man {
    fn step(&mut self, x: i32, y: i32) -> bool {
      self.x += x;
      self.y += y;

      println!("{} moves to [{}, {}]", self.name, self.x, self.y);
      
      true
    }
    fn step3(&mut self, x: i32, y: i32, z: i32) -> bool {
      self.x += x;
      self.y += y;
      self.z += z;
      
      println!("{} moves to [{}, {}, {}]", self.name, self.x, self.y, self.z);
      true
    }
}

impl Cry<&str> for Man {
    fn cry(self, to: &str) -> String {
        let res = format!("{} {} {}", self.name, "cry to", to);
        println!("{}", res);

        res

    }
}


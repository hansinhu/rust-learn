mod shapes;
extern crate rand;

use shapes::Rectangle;
use std::collections::HashMap;
use rand::Rng;

fn main() {
  println!("Hello, world!");

  print_value(188, 166);

  let value: i32 = 2;
  increase_by_one(value);
  println!("After increase_by_one: {}", increase_by_one(value));


  let x: &str = "5";
 
  if x == "5" {
    println!("X is the char 5!");
  } else if x == "6" {
    println!("X is the char 6!");
  } else {
    println!("I dont know what X is.");
  }

  let mut y = 0;
 
  while y != 5 {
    println!("y: {}", y);
    y += 1; // this is equal to x = x + 1;
  }

  let mut z = 0;
  loop {
    if z == 5 {
      break;
    }
    println!("z: {}", z);
    z += 1;
  }

  for w in 0..5 {
    println!("w: {}", w);
  }


  let h = vec![0, 1, 2, 3, 4];
  for element in &h { //&h 表示 for 循环“借用了”矢量，该概念将在下一节中介绍
    println!("element: {}", element);
  }

  // 所有权问题
  let mut q = vec![1, 2, 3];
  push_five(&mut q);
  let q = print_vector(&mut q);
  println!("{:?}", q);

  // 对象1
  let c_1 = Circle {
    center: (0, 0),
    radius: 1,
  };

  let c_2: Circle = Circle {
    center: (-1, 1),
    radius: 2,
  };

  println!("c_1’s radius is {} & c2's center is {:?}", c_1.radius, c_2.center);

  // Circle 构造函数
  let mut c = Circle::new((0, 0), 1);
  println!("The circle's area is {}", c.area());
  println!("The circle location is {:?}", c.center);

  c.move_to((-1, 1));

  println!("The circle location is {:?}", c.center);

  // 枚举
  let color = Color::Red;
  match color {
    Color::Red => println!("Got a red color!"),
    Color::Blue => println!("Got a blue color!"),
  }

  // Match
  let m = 5;
  match m {
    1 => {
      println!("Matched to 1!")
    },
    2 => println!("Matched to 2!"),
    3 => println!("Matched to 3!"),
    4 => println!("Matched to 4!"),
    5 => {
      print!("Matched===>");
      println!("Matched to 5!");
    },
    6 => println!("Matched to 6!"),
    _ => println!("Matched to some other number!"),
  };

  // mod
  let r = Rectangle {
    width: 100,
    height: 50,
  };
  println!("The Rectangle width is {}", r.width);

  // mod-2 std
  let mut counter = HashMap::new(); // HashMap<char, i32>
  counter.insert('a', 1);

  // 第三方包 rand
  let mut rng = rand::thread_rng();
  let rx: f64 = rng.gen();

  println!("Rng Num is: {}", rx)
}

// ==============================
fn print_value (value: i32, value2: i32) {
  println!("The Values are :{} and {}", value, value2);
}

fn increase_by_one(value: i32) -> i32 {
  let mut value1 = value;
  value1 += 1;
  return value1
}

fn push_five(v: &mut Vec<i32>) {
  v.push(5);
}

fn print_vector(v: &Vec<i32>) {
  println!("I borrowed this data: {:?}", v);
}

struct Circle {
  center: (i32, i32),
  radius: u32,
}

impl Circle {
  fn new(center: (i32, i32), radius: u32) ->Circle {
    Circle {
      center: center,
      radius: radius,
    }
  }

  fn area(&self) -> f64 {
    // convertion self.radius, a u32, to an f64
    let f_radius = self.radius as f64;
    f_radius * f_radius * 3.14159
  }

  fn move_to(&mut self, new_center: (i32, i32)) {
    self.center = new_center;
  }

}

// 枚举
enum Color {
  Red,
  Blue,
}

use std::cmp::PartialOrd;

struct Point<T, U> {
  x: T,
  y: U,
}

impl<T, U> Point<T, U> {
  fn x(&self) -> &T {
    &self.x
  }

  // fn distace_from_origin(&self) -> f32 {
  //   (self.x.powi(2) + self.y.powi(2)).sqrt()
  // }

  fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
    Point {
      x: self.x,
      y: other.y,
    }
  }
}

enum Option<T> {
  Some(T),
  None,
}

enum Result<T, E> {
  Ok(T),
  Err(E),
}

fn main() {

  let integer = Point { x: 5, y: 10 };
  let float = Point { x: 1.0, y: 4.0 };

  println!("float.x is: {}", float.x());

  let p1 = Point { x: 5, y: 10.4 };
  let p2 = Point { x: "Hello", y: 'c' };

  let p3 = p1.mixup(p2);

  println!("p3.x = {}, p3.y = {}", p3.x, p3.y);


  let number_list = vec![34, 50, 25, 100, 65];

  // let result = largest_i32(&number_list);
  let result = largest(&number_list);
  println!("The largest number is {}", result);

  let char_list = vec!['y', 'm', 'a', 'q'];

  // let result = largest_char(&char_list);
  let result = largest(&char_list);
  println!("The largest char is {}", result);
}

fn largest_i32(list: &[i32]) -> i32 {
  let mut largest = list[0];
  for &item in list {
    if item > largest {
      largest = item;
    }
  }
  largest
}

fn largest_char(list: &[char]) -> char {
  let mut largest = list[0];
  for &item in list {
    if item > largest {
      largest = item;
    }
  }
  largest
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
  let mut largest = list[0];

  for &item in list {
    if item > largest {
      largest = item;
    }
  }

  largest
}


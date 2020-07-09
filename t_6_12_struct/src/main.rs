fn main() {
  let mut user1 = build_user(String::from("someone@example.com"), String::from("someone123"));

  user1.username = String::from("someone456");

  println!("user1's name: {:?}", user1.username);

  // 语法..指定未明确设置的其余字段应与给定实例中的字段具有相同的值
  let user2 = User {
      email: String::from("another@example.com"),
      username: String::from("anotherusername567"),
      ..user1
  };

  println!("user2's value: {} {} {}", user2.email, user2.sign_in_count, user2.active);

  // 元组结构
  struct Color(i32, i32, i32);
  struct Point(i32, i32, i32);

  let black = Color(3, 3, 3);
  let origin = Point(1, 2, 2);
  println!("{} {}", black.0, origin.0);


  // 计算矩形面积
  rectangles()
}

struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

fn build_user(email: String, username: String) -> User {
  User {
    email,
    username,
    active: true,
    sign_in_count: 1,
  }
}

fn rectangles () {

  // 直白方式
  let w = 30;
  let h = 50;

  println!( "area：{}", area(w, h));

  // 元组方式
  let rect1 = (30, 50);
  println!( "area1：{}", area1(rect1));

  // 对象形式
  
  let rect2 = Rectangle {
    width: 30,
    height: 50,
  };
  println!( "area2：{}", area2(&rect2));
  println!( "rect2 is: {:#?}", rect2);

  // 定义方法 fn
  impl Rectangle {
      fn area(&self) -> u32 {
          self.width * self.height
      }
      fn can_hold(&self, other: &Rectangle) -> bool {
          self.width > other.width && self.height > other.height
      }

      // 关联函数 associated functions
      // 定义范围内的功能impl块是不带self作为参数。这些被称为关联函数，因为它们与结构关联。
      // 它们仍然是函数，而不是方法，因为它们没有可使用的结构实例。您已经使用了String::from关联的功能。
      // 要调用此关联函数，我们使用::带有结构名称的语法； 如 let sq = Rectangle::square(3);
      // fn square(size: u32) -> Rectangle {
      //     Rectangle {
      //         width: size,
      //         height: size,
      //     }
      // }
  }

  let rect3 = Rectangle {
    width: 10,
    height: 20,
  };
  println!( "impl area3：{}", rect3.area());
  println!( "impl area2：{}", rect2.area());

  println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));

  // 允许多个 associated functions
  impl Rectangle {
      // 关联函数 associated functions
      fn square(size: u32) -> Rectangle {
          Rectangle {
              width: size,
              height: size,
          }
      }
  }
  // 关联函数调用
  let sq = Rectangle::square(30);
  println!("sq is {:#?}", sq);
}

fn area(width: u32, height: u32) -> u32 {
  width * height
}

fn area1(rect:(u32, u32)) -> u32 {
  rect.0 * rect.1
}

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

fn area2(rect: &Rectangle) -> u32 {
  rect.width * rect.height
}


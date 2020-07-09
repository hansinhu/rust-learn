mod front_of_house;

// use在作用域中添加和路径类似于在文件系统中创建符号链接
pub use crate::front_of_house::hosting;

// 起始相对路径 super
fn serve_order() {}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}

    // 公开结构和枚举
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

// 使用as关键字提供新名称
// use std::io::Result as IoResult;

// 重新导出
// pub use crate::front_of_house::hosting;

// use std::cmp::Ordering;
// use std::io;
// 等价于
// use std::{cmp::Ordering, io};

// 引用路径
pub fn eat_at_restaurant() {
  // 相对路径
  // front_of_house::hosting::add_to_waitlist();

  // 绝对路径
  // 倾向于指定绝对路径，方便彼此独立地移动代码定义和项目调用。
  // crate::front_of_house::hosting::add_to_waitlist();
  hosting::add_to_waitlist();
  hosting::add_to_waitlist();
  hosting::add_to_waitlist();

  // Order a breakfast in the summer with Rye toast
  let mut meal = back_of_house::Breakfast::summer("Rye");
  // Change our mind about what bread we'd like
  meal.toast = String::from("Wheat");
  println!("I'd like {} toast please", meal.toast);

  // The next line won't compile if we uncomment it; we're not allowed
  // to see or modify the seasonal fruit that comes with the meal
  // meal.seasonal_fruit = String::from("blueberries");

  // pub enum
  let order1 = back_of_house::Appetizer::Soup;
  let order2 = back_of_house::Appetizer::Salad;
}

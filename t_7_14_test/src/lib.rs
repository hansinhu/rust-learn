
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

pub fn add_two(a: i32) -> i32 {
  a + 2
}



#[cfg(test)]
mod tests {
    // 将外部模块中的测试代码带入内部模块的范围。我们在这里使用glob，因此我们在外部模块中定义的任何内容都可用于该 tests模块
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
      let larger = Rectangle {
        width: 8,
        height: 7,
      };

      let smaller = Rectangle {
        width: 5,
        height: 3,
      };

      assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn it_adds_two() {
      assert_eq!(add_two(2), 4);
    }

    #[test]
    fn it_works_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}



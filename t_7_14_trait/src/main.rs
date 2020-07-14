
fn main() {
  pub trait Summary {
    fn summarize(&self) -> String;
  }

  pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
  }
  // impl块中定义的函数 可以是独立的，调用方式 Foo::bar()。如果函数采用self，&self或&mut self作为它的第一个参数，也可以使用方法调用的语法，像foo.bar()。
  impl Summary for NewsArticle {
    fn summarize(&self) -> String {
      format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
  }

  pub struct Tweet {
    pub username: String,
    pub content: String,
    pub replay: bool,
    pub retweet: bool,
  }

  impl Summary for Tweet {
    fn summarize(&self) -> String {
      format!("{}: {}", self.username, self.content)
    }
  }

  let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    replay: false,
    retweet: false,
  };

  println!("1 new tweet: {}", tweet.summarize());




  // pub fn notify<T: Summary>(item: &T) {
  //     println!("Breaking news! {}", item.summarize());
  // }

  // fn returns_summarizable(switch: bool) -> impl Summary {
  //     if switch {
  //         NewsArticle {
  //             headline: String::from(
  //                 "Penguins win the Stanley Cup Championship!",
  //             ),
  //             location: String::from("Pittsburgh, PA, USA"),
  //             author: String::from("Iceburgh"),
  //             content: String::from(
  //                 "The Pittsburgh Penguins once again are the best \
  //                 hockey team in the NHL.",
  //             ),
  //         }
  //     } else {
  //         Tweet {
  //             username: String::from("horse_ebooks"),
  //             content: String::from(
  //                 "of course, as you probably already know, people",
  //             ),
  //             reply: false,
  //             retweet: false,
  //         }
  //     }
  // }
  
}

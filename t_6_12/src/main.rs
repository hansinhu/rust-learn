fn main() {
  let s1 = gives_ownership();         // gives_ownership moves its return, value into s1

  let s2 = String::from("hahaha");     // s2 comes into scope

  let s3 = takes_and_gives_back(s2);  // s2 is moved into takes_and_gives_back, which also moves its return value into s3
  println!("values is {} {}", s1, s3);

  // 如果我们想要再次使用它，则还需要将返回的信息传递回去。可以使用元组返回多个值
  let (s4, len) = calculate_length(s1);

  println!("The length of '{}' is {}.", s4, len);

  // 借用
  let len2 = calculate_length_2(&s3);
  println!("The length_2 is '{}' is {}.", s3, len2);

} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
// moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                           // return value into the function
                                           // that calls it

  let some_string = String::from("hello"); // some_string comes into scope

  some_string                              // some_string is returned and
                                           // moves out to the calling
                                           // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                    // scope
  a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
  let length = s.len(); // len() returns the length of a String

  (s, length)
}

fn calculate_length_2(s: &String) -> usize {
  s.len() // len() returns the length of a String
}
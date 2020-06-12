fn main() {
  let s = String::from("Hello World");
  let slice_1 = &s[0..5];
  let slice_2 = &s[6..11];
  let slice_3 = &s[..5];
  let slice_4 = &s[6..];
  println!("SLICE: {} --- {} {} {} {}", s, slice_1, slice_2, slice_3, slice_4);

  let first_word = get_first_word(&s);
  println!("第一个单词是： {}", first_word);

  // 数组切片
  let a = [1, 2, 3, 4, 5];
  let slice_a = &a[1..3];
  println!("数组切片是： {:?}", slice_a);
}

fn get_first_word(s: &String) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
      println!("i, item {} {}", i, item);
      if item == b' ' {
          return &s[0..i];
      }
  }

  &s[..]
}

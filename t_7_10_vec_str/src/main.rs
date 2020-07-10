
fn main() {
  let mut v = Vec::new();

  v.push(5);
  v.push(6);
  v.push(7);
  v.push(8);

  println!("{:#?}", v);


  let v2 = vec![1, 2, 3, 4, 5];

  let third: &i32 = &v2[2];
  println!("The third element is {}", third);

  match v2.get(3) {
      Some(fourth) => println!("The fourth element is {}", fourth),
      None => println!("There is no third element."),
  }

  // 遍历
  for i in &v2 {
      println!("{}", i);
  }

  // 遍历并更改元素，用解引用运算符（*）来获取值，i然后才能使用该 +=运算
  let mut v3 = vec![100, 32, 57];
  for i in &mut v3 {
      *i += 50;
  }
  println!("{:?}", v3);

  // 枚举储存不同值
  enum SpreadsheetCell {
      Int(i32),
      Float(f64),
      Text(String),
  }

  let row = vec![
      SpreadsheetCell::Int(3),
      SpreadsheetCell::Text(String::from("blue")),
      SpreadsheetCell::Float(10.12),
  ];
  
  // match row.get(1) {
  //     Some(cell) => println!("The cell is {}", cell),
  //     None => println!("There is no third element."),
  // }

  /*****************String********************/
  // A String is a wrapper over a Vec<u8>.
  // String
  let mut s1 = String::from("foo");
  let s2 = "bar";
  s1.push_str(s2);
  println!("s1 is {}", s1);

  // +
  let s3 = String::from("Hello, ");
  let s4 = String::from("world!");
  let s5 = s3 + &s4; // note s1 has been moved here and can no longer be used
  println!("s5 is {}", s5);

  // format! 宏
  let s6 = String::from("tic");
  let s7 = String::from("tac");
  let s8 = String::from("toe");

  let s678 = format!("{}-{}-{}", s6, s7, s8);
  println!("s678 is {}", s678);

  // 字符切片
  let hello = "Здравствуйте";

  let hello_slice = &hello[0..4];
  println!("hello_slice is {}", hello_slice);

  // 字符串遍历
  for c in "नमस्ते".chars() {
    println!("{}", c);
  }

  // 字符串遍历原始字节
  for b in "नमस्ते".bytes() {
    println!("{}", b);
  }
}

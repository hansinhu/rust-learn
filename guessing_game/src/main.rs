use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is: {}", secret_number);

    // 允许循环输入
    loop {
      println!("Please input your guess.");

      let mut guess = String::new();

      // 读取输入赋值给 guess
      io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

      // 字符串转数字
      let guess: u32 = match guess
        .trim()
        .parse() {
          Ok(num) => num,
          Err(_) => {
            println!("Please type a number!");
            continue;
          },
        };
        // .expect("Please type a number!");

      println!("You guessed: {}", guess);

      match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
          println!("You win!");
          break;
        },
      }
    }
}

fn main() {
  #[derive(Debug)]
  enum UsState {
    Alabama,
    Alaska,
  }

  enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
  }


  fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
  }

  value_in_cents(Coin::Quarter(UsState::Alaska));

  
  // option
  fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
  }

  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);
  println!("plus_one {:?},{:?}", six, none);

  // if let
  let mut count = 0;
  let coin = Coin::Quarter(UsState::Alabama);
  if let Coin::Quarter(state) = coin {
      println!("State quarter from {:?}!", state);
  } else {
      count += 1;
  }
}

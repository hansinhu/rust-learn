fn main() {
    println!("Hello, world!");

    let x = 5;

    let y = {
        let mut x = 3;
        x += 1;
        x - 2
    };

    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);


    // loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
          break counter * 2;
        }
    };

    println!("The result is {}", result);

    // while
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("[while] the value is: {}", a[index]);
        index += 1;
    }

    // for
    for element in a.iter() {
      println!("[for] the value is: {}", element);
    }

    for number in 1..4 {
      println!("{}!", number);
    }
}

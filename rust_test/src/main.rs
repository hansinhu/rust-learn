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
}

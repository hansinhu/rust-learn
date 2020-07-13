use std::fs::File;
use std::io;
use std::io::Read;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let _f = match f {
      Ok(file) => file,
      Err(error) => match error.kind() {
        ErrorKind::NotFound => match File::create("hello.txt") {
          Ok(fc) => fc,
          Err(e) => panic!("Problem creating the file: {:?}", e)
        },
        other_error => {
          panic!("Problem opening the file: {:?}", other_error)
        }
      },
    };

    // 闭包实现
    // let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });


    // unwrap or expect
    // let _f2 = File::open("hello_2.txt").expect("Failed to open hello.txt");

    // 传播错误
    // fn read_username_from_file() -> Result<String, io::Error> {
    //     let f = File::open("hello.txt");
    
    //     let mut f = match f {
    //         Ok(file) => file,
    //         Err(e) => return Err(e),
    //     };
    
    //     let mut s = String::new();
    
    //     match f.read_to_string(&mut s) {
    //         Ok(_) => Ok(s),
    //         Err(e) => Err(e),
    //     }
    // }

    // 传播错误的捷径：?运算符
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

    let s = read_username_from_file();
    println!("my name is: {:?}", s);

}

// use std::error::Error;
// use std::fs::File;

// fn main() -> Result<(), Box<dyn Error>> {
//     let f = File::open("hello.txt")?;

//     Ok(())
// }

use std::fs::File;
use std::io::{self, Read};
// use std::io::{ErrorKind};

fn main() {
    // let f = File::open("example.txt");

    // alwats use unwrap or expect instead of match statement

    // let f = match f {
    //     Ok(file) => file,
    //     Err(e) => match e.kind() {
    //         ErrorKind::NotFound => match File::create("example.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     }
    // };

    // let f = File::open("example.txr").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("example.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });

    // let _f = File::open("example.txt").unwrap(); // panics with default error message
    // let f = File::open("example.txt").expect("This is a custom message"); // allows for custom error message
    // println!("{:#?}", f);
    println!("{}", read_username_from_file().unwrap());
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("example.txt")?
        .read_to_string(&mut s)?;

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    
    Ok(s)
}
use std::{fs::File, fs, io::{ErrorKind, Read, Error}};

fn read_username_from_file() -> Result<String, Error> {
    // If we want to propogate the error `?` can be used as a shortcut
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s);


    // OR
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s);


    // OR
    fs::read_to_string("hello.txt")

}

fn main() {
    // let v = vec![1, 2, 3];
    // v[99];
    let f = File::open("hello.txt");
    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e),
            },
        other_error => {
            panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };


    // Another way to write the above
    let _f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
        panic!("Problem creating the file {:?}", error);
        }
    });

    // unwrap either returns Ok or calls panic! for us
    let _f = File::open("hello.txt").unwrap();

    // expect is like unwrap but allows for a string
    let _f = File::open("hello.txt").expect("This is part of the panic message");

}

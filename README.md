# Rust book
From https://doc.rust-lang.org/book/

## Notes

### Crates
- **https://crates.io/**: community crate registry

### Cargo
- `cargo help`: in case forget any of the below
- `cargo new`: create a new package in current directory (e.g. `cargo new my_package`)
- `cargo fix`: fix rust linting errors
- `cargo clean`: delete prior **target/** directories
- `cargo update`: update dependencies as defined in lock file
- `cargo build`: build project & dependencies (&/or install from crates.io)
- `cargo run`: run the current package (build if necessary, will not rebuild if not necessary)
- `cargo check`: check compilation without building executable
- `cargo doc --open`: build documentation for all dependencies locally and open in browser
- More here: https://doc.rust-lang.org/cargo/commands/

### Rust
* Error handling:
  * swapping from `expect` call to `match` statement is common to handle an err
    * FROM
      ```rust 
          let guess: u32 = guess.trim().parse().expect("Please type a number!");
      ```
    * TO
       ```rust 
            let guess: u32 = match guess.trim().parse() {
              Ok(num) => num,
              Err(_) => {println!("Enter a valid number between 1 and 100");continue},
          };
      ```


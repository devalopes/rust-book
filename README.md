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

* Variables:
  * Immutable by default (so are refs `&` hence `&mut`)
  * `const`s never change (and type must be annotated)
  * Tuples:
    * Fixed length
    * Can contain multiple types
    * Access tuple elements through `.` notation (`x.1, x.2, x.3`)
    * Support destructuring:
      * let tuple = (100, 200, 300);
      * let (x, y  z) = tup;
  * Arrays:
    * Every element of array must be same type
    * **STACK** Allocated
    * Have fixed length, like tuples
    * `let a: [i32; 5] = [1, 2, 3, 4, 5];`
* Functions:
  * Must declare type of each parameter and return type if any
  * Expressions do not end in semicolons
    * if it does, it's a statement.
* Loops:
  * loop (continuous until break)
  * while (conditional loop)
  * for
* Stack vs Heap
  * Stack = LIFO (last in, first out -- like a stack of plates)
    * push onto stack, pop off of stack
    * All data stored on stack must have a known fixed size - else gets put on heap
    * Stack is faster than heap since *allocation* is not needed
      * Memory always knows where to put stack data -- on top!
  * Heap
    * Unknown size at compile time or size that might change
    * Request a certain heap size, memory allocator finds a spot for it, marks it in use and returns a **pointer**
    * **pointer** is the memory address on the heap of the data
    * The pointer can be stored on the stack(since it has a known, fixed size), but when you want the actual data have to follow pointer to the heap address
    * Heap allocation is slower since memory allocator needs to find a spot for it and reserve it
    * Accessing data on heap is slower also since you have to follow the pointer
  * When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack.
  * Managing stack and _heap_ is why rust **ownership** exists
  * **Ownership**
    * *Rules*:
      * Each value in Rust has a variable that’s called its owner.
      * There can only be one owner at a time.
      * When the owner goes out of scope, the value will be dropped.
    *   
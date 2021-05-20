fn main() {

    // mutability
    let mut x: u8 = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    // constants
    const MAX_POINTS: u32 = 100_000;
    println!("{0}", MAX_POINTS);

    // shadowing
    let y = 5;

    let y = y + 1;

    let y = y * 2;

    println!("The value of y is: {}", y);

    // Shadowing can let you change the variables type
    // mut will not work here, since you can never mutate an objects type
    let spaces = "   ";
    let spaces = spaces.len();
}
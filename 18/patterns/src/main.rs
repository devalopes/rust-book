fn print_coordinates(&(x, y): &(i32, i32)) {
    // Functions can pattern match (as in the signature)
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    // If let is a great alternative to `match` when you only have a few cases you care about
    // Or have cases which do not relate to each other
    // though the compiler does not ensure exhaustiveness in `if let` as it does in match.
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    // While let similar to if let
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];

    // for loops can destructure tuples for pattern matching
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // `let` itself is a pattern matcher
    // let PATTERN = EXPRESSION;
    let (x, y, z) = (1, 2, 3);

    let point = (3, 5);
    print_coordinates(&point);


    // Refutable vs Irrefutable patterns
    // `irrefutable`: Patterns that will match for any value passed
    // e.g. `x` in the statement let x = 5; since this can never fail
    // `refutable`: patterns that can fail to match
    // e.g. if let Some(x) = a_value (fails if a_value is None!)
}

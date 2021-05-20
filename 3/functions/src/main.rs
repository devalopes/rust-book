fn five() -> i32{
    5
}

fn main() {
    println!("Hello, world!");

    let x = five();

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    another_function(x);
}

fn another_function(x: i32) {
    println!("Another function, {}", x);
}
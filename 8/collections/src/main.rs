fn main() {
    // vector, storing type <T>
    let v: Vec<i32> = Vec::new();

    // vector created using macro, holding whatever value types you give.
    // Vectors can only store values of the same type
    let mut v = vec![1,2,3];

    // add to a vector
    v.push(4);
    println!("{:?}", v);

    // Vectors still implement drop
    {
        let v = vec![1, 2, 3, 4];
        // do stuff with v
    } // <- v goes out of scope and is freed here


    // Reading vector elements
    // Will cause panic if referencing non-existing item
    let third: &i32 = &v[2];
    println!("the third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }


    // Iterating over vector elements
    let v = vec!["a", "b", "c"];
    for i in &v{
        println!("{}", i);
    }
    let mut v = vec![100, 200, 300];
    for i in &mut v{
        //To change the value that the mutable reference refers to, we have to use the dereference
        //operator (*) to get to the value in i before we can use the += operator
        *i += 50;
    }
    println!("{:?}", v);


    // Use an enum to store vector of multiple types!
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    };

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("hello")),
        SpreadsheetCell::Float(10.23),
    ];

}

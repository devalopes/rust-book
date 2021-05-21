fn main() {

    let mut s = String::from("hello");

    s.push_str(" alex"); // String is mutable, &str is not

    println!("{}", s);

    
    {   // Resource is allocated
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    };                                // this scope is now over, and s is no
                                       // longer valid
                                       // s is "dropped"

    let s1 = String::from("hello"); 
    let s2 = s1; 

    // println!("{}, world!", s1); // Value literally moved, to prevent double free errors. Will error out.

    // cloning is copying the entire heap data
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);


    let s = String::from("hi");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward
    
     // Here, x goes out of scope, then s. But because s's value was moved, nothing
      // special happens.
    
    fn takes_ownership(some_string: String) { // some_string comes into scope
        println!("{}", some_string);
    } // Here, some_string goes out of scope and `drop` is called. The backing
      // memory is freed.
    
    fn makes_copy(some_integer: i32) { // some_integer comes into scope
        println!("{}", some_integer);
    } // Here, some_integer goes out of scope. Nothing special happens.


    let s1 = String::from("hello");

    // Used a reference so we don't have to worry about ownership change!!
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);


    fn calculate_length(s: &String) -> usize {
        s.len()
    }

}

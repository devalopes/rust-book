fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // The function signature now tells Rust that for some lifetime 'a, the function takes two parameters,
    // both of which are string slices that live at least as long as lifetime 'a.
    // The function signature also tells Rust that the string slice returned from the function will live at least as long as lifetime 'a.
    // In practice, it means that the lifetime of the reference returned by the longest function is the same as the smaller
    // of the lifetimes of the references passed in.
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // Doesnt work because the subject of reference doesnt live as long as the ref
    // {
        // let r;                // ---------+-- 'a
        //                       //          |
        // {                     //          |
        //     let x = 5;        // -+-- 'b  |
        //     r = &x;           //  |       |
        // }                     // -+       |
        //                       //          |
        // println!("r: {}", r); //          |
    // }
    //
    // Lifetimes ('a and 'b) -- works because reference will always be valid
    {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);



    // Lifetime ellisons
    // lifetime rules inserted automatically by the compiler
    /*
    * The first rule is that each parameter that is a reference gets its own lifetime parameter. In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32); a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.
    * The second rule is if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.
    * The third rule is if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary.
    */


    // The static lifetime
    // Special rule 'static meaning reference can live forever (string literals)

    // The text of this string is stored directly in the program’s binary, which is always available. Therefore, the lifetime of all string literals is 'static.
    let s: &'static str = "I have a static lifetime.";
}

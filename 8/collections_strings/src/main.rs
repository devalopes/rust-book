fn main() {
    // &str is a string __slice__, reference to utf8 encoded stored string
    // String literals are stored in the programs binary

    let mut s = String::new();
    let mut s = "initial contents".to_string();

    s.push_str(" test");
    println!("{}", s);


    // concatenating
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // format! macro helps when add(&str) becomes unwieldy...
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);


    // IMPORTANT:   Cannot index strings in rust.
    // This is due to utf-8 characters taking 2 bytes
    // and String is really just a wrapper around vec<T>
    // no way to always return a single value
    // even if it did, it would return the byte value.
    //
    // This is also partially for multi language support.
    // And so that string indexing is always O(1)

}

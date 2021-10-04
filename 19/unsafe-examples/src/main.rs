extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    // Unsafe can allow you to:
    // Dereference a raw pointer
    // Call an unsafe function or method
    // Access or modify a mutable static variable
    // Implement an unsafe trait
    // Access fields of unions

    let mut num = 5;
    // Immutable and mutable raw pointers from refs
    // Not unsafe yet... we can create these but cannot deref them safely
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // Pointer whose validity we can't be certain of
    let address = 0x012345usize;
    let r = address as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

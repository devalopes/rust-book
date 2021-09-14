enum List {
    // Box<> used to store data on heap
    // can be treated like a reference since implements `Deref` (*) trait
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    // Custom drop trait
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

enum ListRc {
    Cons(i32, Rc<ListRc>),
    Nil,
}

use std::rc::Rc;

fn main() {
    // The most straightforward smart pointer is a box, whose type is written Box<T>.
    // Boxes allow you to store data on the heap rather than the stack.
    // What remains on the stack is the pointer to the heap data.
    // Boxes don’t have performance overhead, other than storing their data on the heap instead of on the stack.
    // But they don’t have many extra capabilities either. You’ll use them most often in these situations:
    // * When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
    // * When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
    // * When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type


    // Box helpful for recursive functionality
    let list: List = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = &x;

    // * fetches the literal underlying value, & would just return a ref
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Same as above -- but using Box<T> instead of a ref.
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Implementing our own Box<T> to show how we can use Deref trait.
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);


    // Deref coercion
    // Converts a type into a ref to another type
    // e.g. &String to &str
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // hello(&(*m)[..]);
    // Because rust can convert the type here, we do not need this code above ^^
    //
    // `Drop` is another trait commonly added to smart pointers
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");



    // Reference counted smart pointer Rc<T>
    // Allows for multiple owners, counting the refs, drops when no more refs (not in use)
    // Common with any graph or node data structure
    // Can only be used in single threaded programs
    let a = Rc::new(ListRc::Cons(5, Rc::new(ListRc::Cons(10, Rc::new(ListRc::Nil)))));
    let a = Rc::new(ListRc::Cons(5, Rc::new(ListRc::Cons(10, Rc::new(ListRc::Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = ListRc::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = ListRc::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    //With references and Box<T>, the borrowing rules’ invariants are enforced at compile time.
    //With RefCell<T>, these invariants are enforced at runtime. With references, if you break these rules, you’ll get a compiler error.
    //With RefCell<T>, if you break these rules, your program will panic and exit.
    //The RefCell<T> type is useful when you’re sure your code follows the borrowing rules but the compiler is unable to understand and guarantee that.
}

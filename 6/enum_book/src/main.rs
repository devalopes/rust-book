 #[derive(Debug)]
enum IpAddrKind{
    V4,
    V6
}

enum IpAddr{
    V4(u8, u8, u8, u8),
    V6(String)
}

fn main() {
    // Enum is a reserved word - so `enum_book` it is :)

    let ipv4 = IpAddrKind::V4;
    let ipv6 = IpAddrKind::V6;

    println!("{:?}", ipv4);

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));


    // Example enum & comparable struct
    /*
    Quit has no data associated with it at all.
    Move includes an anonymous struct inside it.
    Write includes a single String.
    ChangeColor includes three i32 values.
    */
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    /*
    But if we used the different structs, which each have their own type, we couldn’t as easily define a function to take any of these kinds of messages as we could with the Message enum which is a single type.
    */
    struct QuitMessage; // unit struct
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // tuple struct
    struct ChangeColorMessage(i32, i32, i32); // tuple struct

    // like structs -- can also define methods on enums!
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

}

fn route(ip_kind: IpAddrKind) {}

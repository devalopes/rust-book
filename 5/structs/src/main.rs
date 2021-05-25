struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn main() {
    let user1 = User {
        email: String::from("test@gmail.com"),
        username: String::from("test"),
        active: true,
        sign_in_count: 1,
    };

    // Tuple struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = (0, 0, 0);
    let origin = (0, 0, 0);


}

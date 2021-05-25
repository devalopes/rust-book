use std::cmp::Ordering;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }

    // Associated function (like String::from) -- does not take &self param
    // so it's a function - not a method!
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }

    fn can_hold(&self, query_rect: &Rectangle) -> bool {
        let self_rect_area = self.width * self.height;
        let query_rect_area = query_rect.width * query_rect.height;

        match self_rect_area.cmp(&query_rect_area) {
            Ordering::Less => false,
            Ordering::Greater => true,
            Ordering::Equal => true
        }
        
    }
}

fn main() {
    let rect = Rectangle{
        width: 30,
        height: 30,
    };

    println!(
        "The area of the rectangle is {} square pixels", 
        rect.area()   
    );

    println!("{:#?}", rect);

    
    let rect2 = Rectangle {
        width: 91,
        height: 10
    };

    println!("{:?}", rect.can_hold(&rect2));

    let square = Rectangle::square(50);
    println!("{:?}", square.area());
}

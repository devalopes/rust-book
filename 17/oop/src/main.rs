
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    // Represents encapsulation
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}


use oop::Draw;
use oop::{Button, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    // draw trait
    fn draw(&self) {
        // code to actually draw a select box
    }
}
fn main() {
    // Rust does not by default have inheritance
    // Though `traits` serve a similar feature
    // and can be thought of in below example (and related lib.rs)
    let screen = Screen {
        // User can add their own implementations
        // as long as it implements `draw` trait
        // so we don't have to know about all possible elements up front
        // very similar to dynamic dispatch (when class method is unknown at compile time)
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

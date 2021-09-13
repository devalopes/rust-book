use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
// fn generate_workout(intensity: u32, random_number: u32) {
//    // Original
//     if intensity < 25 {
//         println!(
//             "Today, do {} pushups!",
//             simulated_expensive_calculation(intensity)
//         );
//         println!(
//             "Next, do {} situps!",
//             simulated_expensive_calculation(intensity)
//         );
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Remember to stay hydrated!");
//         } else {
//              // Call expensive function again :(
//             println!(
//                 "Today, run for {} minutes!",
//                 simulated_expensive_calculation(intensity)
//             );
//         }
//     }
// }
//
// fn generate_workout(intensity: u32, random_number: u32) {
//     // Refactored to be called only once, but still a case where it shouldn't be called at all!
//     let expensive_result = simulated_expensive_calculation(intensity);

//     if intensity < 25 {
//         println!("Today, do {} pushups!", expensive_result);
//         println!("Next, do {} situps!", expensive_result);
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Remember to stay hydrated!");
//         } else {
//             println!("Today, run for {} minutes!", expensive_result);
//         }
//     }
// }

// Ex. closure cached result
// Cache struct
struct Cacher<T>
where
    T: Fn(u32) -> u32, // Fn trait indicates closure type
{
    calculation: T,
    value: Option<u32>,
}

// Cache implementation
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

// fn generate_workout(intensity: u32, random_number: u32) {
//     // Refactored with closure
//     // `let` means the closure contains the definition of a function, **not** the resulting value
//     // Still... introduces problems similar to original (called multiple times) -- and why no type hints?
//     /*
//      * Type hints are not needed since the types are assumed after first use by compiler
//      * (though you can add types).
//      * Closures are this way since use is meant to be small and specific
//      *
//      * As for calling multiple times -- we can refactor the closure to cache the result if it has
//      * been called already (Memoization)
//      */
//     let expensive_closure = |num| {
//         println!("calculating slowly...");
//         thread::sleep(Duration::from_secs(2));
//         num
//     };

//     if intensity < 25 {
//         println!("Today, do {} pushups!", expensive_closure(intensity));
//         println!("Next, do {} situps!", expensive_closure(intensity));
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Remember to stay hydrated!");
//         } else {
//             println!("Today, run for {} minutes!", expensive_closure(intensity));
//         }
//     }
// }

fn generate_workout(intensity: u32, random_number: u32) {
    // Final implementation using Cacher
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);


    /*
    * Closures can do things functions can't (Like capture the environment)
    *
    */
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));


    // Can force closure to take ownership of the value using `move` keyword
    // Useful for concurrency when you want to move the data so it's owned by new thread

}

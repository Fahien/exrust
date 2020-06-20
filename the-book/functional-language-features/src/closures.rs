use std::collections::HashMap;
use std::thread;
use std::time::Duration;

// We would like to simulate the expensive calculation only once when we need its result.
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    values: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        if self.values.contains_key(&arg) {
            self.values[&arg]
        } else {
            let value = (self.calculation)(arg);
            self.values.insert(arg, value);
            value
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // Closures do not need type annotations of parameters or return value
    // The compiler is able to infer them similar to how it is able to infer types of variables.
    let mut expensive_result = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    // Low intensity workout
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} pushups!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

pub fn run() {
    let simulated_user_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_value, simulated_random_number);
}

#[test]
// Cacher should work with different inputs
fn call_with_defferent_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
}

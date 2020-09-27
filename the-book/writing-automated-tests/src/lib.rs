#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

// This annotation tells rust to compile and run this code only when you run `cargo test`
// cfg stands for configuration and test is its option
#[cfg(test)]
mod tests {
    // Run tests with
    //   `cargo test`
    // Run tests in sequence with
    //   `cargo test -- --test-threads=1`
    // To see the output of successful tests run
    //   `cargo test -- --show-output`
    // Pass the name of the test to run only that test
    //   `cargo test can_add_two`
    // We can filter tests by passing only part of their name
    //   `cargo test two`
    // To run ignored tests
    //   `cargo test -- --ignored
    use super::*;

    // This is a test function because it is annotated with test
    #[test]
    fn can_add_two() {
        // Assert equals and not equals
        assert_eq!(add_two(2), 4);
        assert_ne!(add_two(3), 4);
        // A test fails when something panics
        // panic!("help");
    }

    #[test]
    // A test can return a Result
    // This is useful because enables you to use operator ?
    fn two_plus_two() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("Two plus two is not four"))
        }
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let a = Rect{ width: 10, height: 9 };
        let b = Rect { width: 5, height: 4 };
        assert!(!b.can_hold(&a));
    }

    #[test]
    fn larger_can_hold_smaller() {
        let a = Rect{ width: 10, height: 9 };
        let b = Rect { width: 5, height: 4 };
        assert!(a.can_hold(&b));
    }

    fn greets(name: &str) -> String {
        format!("Hello {}", name)
    }

    #[test]
    // We can ignore time consuming tests
    #[ignore]
    fn greeting_contains_name() {
        let name = "Slartibartfast";
        let result = greets(name);
        assert!(
            result.contains(name),
            "Greeting does not contain {}: result is {}", name, result
        );
    }

    #[test]
    // This test should panic, and the expected panic message
    // should contain the expected string
    #[should_panic(expected = "Guess value must be between 1 and 100")]
    fn greather_than_100() {
        Guess::new(101);
    }
}

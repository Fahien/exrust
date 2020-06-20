pub fn run() {
    let v1 = vec![1, 2, 3];

    // Iterators are lazy, nothing special is done here
    let v1_iter = v1.iter();

    // We can use iterator with fors
    // Fors take ownership of iterators making them mutable
    for val in v1_iter {
        println!("Got: {}", val);
    }
}

// Iterators implement a trait named Iterator
pub trait IteratorExample {
    // Type associated with this trait
    type Item;

    // The associated type is returned by this function
    // This function needs to be defined by implementors
    // When the iterator is over, it returns None
    fn next(&mut self) -> Option<Self::Item>;
}

#[test]
fn call_iterator_next() {
    let v1 = vec![1, 2, 3];

    // Even though it is mutable, it iterates over immutable references
    let mut v1_iter = v1.iter();

    // We can call next directly
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

// This function takes ownership of a vector of shoes
// Returning a vector of shoes of a specified size
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // Into iter takes ownership of the collection
    // Filter is another iterator adaptors. Here it uses a closure that captures shoes size.
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

struct CounterTo5 {
    count: u32,
}

impl CounterTo5 {
    fn new() -> CounterTo5 {
        CounterTo5 { count: 0 }
    }
}

// Implement iterator trait for CounterTo5
impl Iterator for CounterTo5 {
    // Do not care about associated type yet
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            // Once count is 5, stop
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn call_iterator_sum() {
        let v1 = vec![1, 2, 3];

        // Iterators have other methods like sum, which calls
        // next repeatedly adding each item to a running total
        let v1_iter = v1.iter();
        // Sum takes ownership of the iterator
        // Methods like sum are called consuming adaptors
        let total: i32 = v1_iter.sum();
        assert_eq!(total, 6);
    }

    #[test]
    fn call_iterator_map() {
        let v1 = vec![1, 2, 3];

        // Iterator methods like map are called iterator adaptors
        // because they change them into different kinds of iterators
        let v1_map = v1.iter().map(|x| x + 1);

        // Nothing has happened yet. We need to call a consuming method

        // We can call collect, which consumes the iterator
        // collecting the resulting values into a collection
        let v2: Vec<_> = v1_map.collect();

        assert_eq!(v2, vec![2, 3, 4]);
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ]
        );
    }

    #[test]
    fn custom_counter() {
        let mut counter = CounterTo5::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn methods_with_custom_counter() {
        // Create a new counter
        let sum: u32 = CounterTo5::new()
            // Zip this counter with another counter which skips the first element
            // (1,2),(2,3),(3,4),(4,5)
            .zip(CounterTo5::new().skip(1))
            // 2,6,12,20
            .map(|(a, b)| a * b)
            // 6,12
            .filter(|x| x % 3 == 0)
            // 18
            .sum();

        assert_eq!(sum, 18);
    }
}

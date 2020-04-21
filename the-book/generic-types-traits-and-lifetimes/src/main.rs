// Returns the largest item T in the list
// T is a generic and can be of any type
// But this type has to have partial ordering and copy traits implemented
// Where PartialOrd makes > to work
// And Copy ensures the type is stored on the stack
fn largest<T: PartialOrd + Copy>(list: &[T]) -> Option<T> {
    if list.is_empty() {
        return None;
    }

    let mut largest = list[0];

    for &elem in list {
        if elem > largest {
            largest = elem;
        }
    }

    Some(largest)
}

// Coordinates of any type
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// We can define generic methods
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// We can define method for specific instances
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn generic_data_types() {
    // Find the largest of two different list with different element types
    // using a generic function
    let numbers = vec![1, 6, 3, 9, 23, 523, 1, 2, 3, 0];
    println!("Largest number is {}", largest(&numbers).unwrap());
    let characters = vec!['a', 'f', '6', 'g', 'z'];
    println!("Largest char is {}", largest(&characters).unwrap());

    // Two coordinates, one integer, one float
    let i_point = Point { x: 1, y: 2 };
    println!("Integer point {:?}, method x {}", i_point, i_point.x());
    let f_point = Point { x: 0.1, y: 0.9 };
    println!(
        "Floating point {:?}, distance from origin {}",
        f_point,
        f_point.distance_from_origin()
    );
}

// We can define traits. They are similar to interfaces
trait Summary {
    // Useful method
    fn author(&self) -> String;

    // This needs implementation
    fn summarize(&self) -> String;

    // This can be used as default
    fn default_summary(&self) -> String {
        format!("Read more from {}...", self.author())
    }
}

// Then we have a normal struct
struct Article {
    headline: String,
    author: String,
    content: String,
}

// We can implement Summary trait for Article
impl Summary for Article {
    fn author(&self) -> String {
        format!("{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{} [{}] {}", self.headline, self.author(), self.content)
    }
}

// Then we have another normal struct
struct Tweet {
    username: String,
    content: String,
}

// Another implementation of Summary for Tweet
impl Summary for Tweet {
    fn author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.author(), self.content)
    }
}

fn print_thing(thing: &impl Summary) {
    println!("> {}", thing.summarize());
    println!(">> {}", thing.default_summary());
}

fn traits() {
    let a = Article {
        headline: String::from("This is it"),
        author: String::from("myself"),
        content: String::from("This is the content of the article"),
    };

    let t = Tweet {
        username: String::from("mike"),
        content: String::from("Hello from me"),
    };

    print_thing(&a);
    print_thing(&t);
}

// We specify that all references must have the same lifetime.
// The concrete lifetime that is substituted for 'a
// is the smaller of the lifetimes of x and y.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Structs need to specify lifetimes to hold references
struct Excerpt<'a> {
    part: &'a str,
}

// We need to use lifetimes in impl as well
impl<'a> Excerpt<'a> {
    // This method does not need lifetimes annotations
    fn level(&self) -> i32 {
        3
    }

    // Return lifetime will be that of self
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Listen: {}", announcement);
        self.part
    }
}

fn lifetimes() {
    let string1 = String::from("abdc");
    // All string literals have static lifetime
    // That means they will live for the entire duration of the program
    let string2: &'static str = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("Longest is {}", result);

    let exc = Excerpt {
        part: &string1[..2],
    };
    println!("Excerpt is {} - level {}", exc.part, exc.level());
    exc.announce_and_return_part("Get part");
}

fn main() {
    generic_data_types();
    traits();
    lifetimes();
}

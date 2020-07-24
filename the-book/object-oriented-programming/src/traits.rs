// In Rust we use Traits instead of inheritance
pub trait Draw {
    fn draw(&self);

    // A trait is object-safe if:
    // - methods do not return Self
    // - there are no generic type parameters
    // The std Clone trait is not object-safe cause clone returns Self
}

pub struct Screen {
    // This can hold any kind of drawable types
    // When we use trait objects, Rust must use dynamic dispatch.
    // Compiler does not know all the types that might be used.
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// This will implement the Draw trait
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // Code to draw the Button
    }
}

// Another struct implementing Draw
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // Code to draw a SelectBox
    }
}

pub fn run() {
    // A screen can contain both a Button and a SelectBox
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("No"),
                    String::from("Do not know"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    // We are concerned only with the messages a value responds to
    // rather than the value's concrete type. This is like duck typing:
    // If it walks like a duck and quacks like a duck, then it must be a duck!
    // Screen::run() does not need to know the concrete type of each component.
}

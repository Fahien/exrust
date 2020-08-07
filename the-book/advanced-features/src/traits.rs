use std::fmt;
// We can overload operations using trait listed in std::ops
use std::ops::Add;

pub trait Iterator {
	type Item; // The type will be specified by the implementor

	fn next(&mut self) -> Option<Self::Item>;
}

#[derive(Debug, PartialEq)]
struct Point {
	x: i32,
	y: i32,
}

impl Add for Point {
	type Output = Point;

	fn add(self, other: Point) -> Point {
		Point {
			x: self.x + other.x,
			y: self.y + other.y,
		}
	}
}

pub fn operator_overload() {
	assert_eq!(
		Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
		Point { x: 3, y: 3 }
	);
}

struct Millimeters(u32);
struct Meters(u32);

// We can specify a different type for RHS
impl Add<Meters> for Millimeters {
	type Output = Millimeters;

	fn add(self, other: Meters) -> Millimeters {
		Millimeters(self.0 + other.0 * 1000)
	}
}

trait Pilot {
	fn fly(&self);
}

trait Wizard {
	fn fly(&self);
}

struct Human;

// We implement both traits having a method with the same name
impl Pilot for Human {
	fn fly(&self) {
		println!("This is your captain speaking");
	}
}

impl Wizard for Human {
	fn fly(&self) {
		println!("Levitate");
	}
}

// There can also be a class method with the same name
impl Human {
	fn fly(&self) {
		println!("Waving arms");
	}
}

// Some traits can have associated functions
trait Animal {
	fn baby_name() -> String;
}

struct Dog;

impl Dog {
	fn baby_name() -> String {
		String::from("Spot")
	}
}

impl Animal for Dog {
	fn baby_name() -> String {
		String::from("Puppy")
	}
}

pub fn disambiguation() {
	// The compiler defaults to calling the method on the type
	let person = Human;
	person.fly(); // Waving arms

	// To call the other methods we need explicit syntax
	Pilot::fly(&person);
	Wizard::fly(&person);

	println!("A baby dog is called {}", Dog::baby_name());
	// If we wanted to call the function part of the Animal rait
	// Animal::baby_name is not enough, Rust can't figure out which
	// implementation we want. So we need to use qualified syntax.
	// <Type as Trait>::function(..)
	println!(
		"A baby dog is generally called {}",
		<Dog as Animal>::baby_name()
	);
}

// With supertraits we can specify that a certain trait will work only for types that
// also implement another trait which is used to implement the new trait's functionality
trait UnderlinePrint: fmt::Display {
	fn underline_print(&self) {
		let output = self.to_string();
		println!("{}", output);

		let len = output.len();
		println!("{}", "-".repeat(len));
	}
}

// Let's implement display for Point
impl fmt::Display for Point {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}

impl UnderlinePrint for Point {}

pub fn supertraits() {
	let p = Point { x: 1, y: 2 };
	p.underline_print()
}

// We can't implement a trait on a type that is not local to our crate.
// But there is a way to circumvent this restriction by using a new type.
// A tuple struct is going to be elided at compile time.
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "[{}]", self.0.join(","))
	}
}

impl UnderlinePrint for Wrapper {}

pub fn newtype() {
	let wrapper = Wrapper(vec![String::from("Hello"), String::from("world")]);
	wrapper.underline_print();
}

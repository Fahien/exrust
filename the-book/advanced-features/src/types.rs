// Similar to the newtype pattern, a type alias can be used to give an exysting type another name.
type Kilometers = i32;

pub fn aliases() {
	// Type aliases are not new types, they can be used as their underlying type
	let x: i32 = 5;
	let y: Kilometers = 3;
	println!("x (={}) (i32) + y (={}) (Kilometers) = {}", x, y, x + y);
}

// Type aliases are useful to avoid typing long names multiple times.
// For example with Result<T,E>
type Result<T> = std::result::Result<T, std::io::Error>;

pub fn use_result() -> Result<&'static str> {
	Result::Ok("Result")
}

// Rust has a special type named !
// This is called "never type", used for functions that never return
// These kinds of functions are said to be "diverging"
fn _bar() -> ! {
	panic!("Fly you fools!")
}

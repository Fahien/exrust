use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

// Macros compare a value to patterns, where the value is literal Rust source code.

// This annotation makes the macro available when this crate is used
#[macro_export]
// We start a macro with a macro_rules! followed by the name of the macro
macro_rules! pack {
	// Similar to a match expression we have an arm with an associated block

	// $x matches expressions, and * means that it matches zero or more times
	( $( $x:expr ),* ) => {
		{
			let mut temp_vec = Vec::new();
			// The code within these parentheses is generated for each part that matches x
			$(
				temp_vec.push($x);
			)*
			temp_vec
		}
	};
}

pub fn declarative() {
	let p = pack![1, 2, 3];
	println!("{:?}", p);
}

#[derive(HelloMacro)]
struct Pancakes;

pub fn procedural() {
	Pancakes::hello_macro();
}

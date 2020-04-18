mod front_of_house;

// Modules are like C++ namespaces, but children are private by default
mod back_of_house {
	// Structs can be public
	pub struct Breakfast {
		// Fields can be public as well
		pub toast: String,
		pub seasonal_fruit: String,
	}

	impl Breakfast {
		pub fn summer(toast: &str) -> Breakfast {
			Breakfast {
				toast: String::from(toast),
				seasonal_fruit: String::from("raspberry"),
			}
		}
	}

	// Public enums just need one pub
	#[derive(Debug)]
	pub enum Appetizer {
		Soup,
		Salad,
	}
}

// Function exposed with pub
pub fn eat_at_restaurant() {
	// Absolute path; crate is like using root /
	crate::front_of_house::hosting::add_to_waitlist();

	// Relative path
	front_of_house::hosting::add_to_waitlist();

	// Order breakfast in summer with rye toast
	let meal = back_of_house::Breakfast::summer("rye");
	println!("I'd like {} toast please", meal.toast);

	let mut order = back_of_house::Appetizer::Salad;
	println!("I'd like {:?} please", order);
	order = back_of_house::Appetizer::Soup;
	println!("Sorry, I changed my mind, I would like {:?} instead", order);
}

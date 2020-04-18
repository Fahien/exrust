// A module in its own file

pub mod hosting {
	pub fn add_to_waitlist() {
		// Wait for some time?
		seat_at_table();
	}

	fn seat_at_table() {
		// Super is like get parent ../
		super::serving::take_order();
	}
}

mod serving {
	pub fn take_order() {
		// Once seated, I guess :D
		// Then wait for order to be prepaired, and
		serve_order();
		take_payment();
	}

	fn serve_order() {}

	fn take_payment() {}
}

// Each file in the tests directory is compiled as a separate crate,
// therefore we need to import our library crate if we want to test it
use writing_automated_tests;

// Common module for integration tests created by us
mod common;

// To run all the tests in a particular integratin test file
// use `cargo test --test <integration test file name>`
#[test]
fn can_add_two() {
	common::setup();
	assert_eq!(4, writing_automated_tests::add_two(2));
}

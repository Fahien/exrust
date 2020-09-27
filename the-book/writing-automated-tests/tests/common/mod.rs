// Files in subdirectories on tests will not get compiled as separate crates
pub fn setup() {
	println!("This can be called by other integration test crates")
}

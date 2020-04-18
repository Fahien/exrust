// No need to mention src/main.rs in Cargo.toml as by convention that is the crate root of binary crate.
// If that was src/lib.rs, then the package would be a library crate.
// It it has both, the package contains wither a binary and a library crate.
// If you put files in src/bin, each of them will be a separate binary crate.

// Well, that's how I named the directory
use managing_growing_projects_with_packages_crates_and_modules::eat_at_restaurant;

fn main() {
    eat_at_restaurant();
}

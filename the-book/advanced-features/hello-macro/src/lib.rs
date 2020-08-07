pub trait HelloMacro {
    // This function should say hello with the name of the type implementing the trait but
    // Rust does have reflection capabilities, so we need to generate code at compile time.
    fn hello_macro();
}


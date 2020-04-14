fn main() {
    // This string can be mutated because it is stored on the heap
    let mut s1 = String::from("Hello ");
    s1.push_str("world!");
    println!("{}", s1);

    // This means move s1 into s2
    let s2 = s1;
    // If we use s1 here, rust gives an error

    // This does not happen for "Copy" types stored on the stack
    let x = 5;
    let y = x;
    println!("x {}, y {}", x, y);

    // If we actually want to copy data on the heap, clone is a common method for that
    let s3 = s2.clone();
    println!("{} = {}", s3, s2);

    // Move s2 into this function
    move_into_function(s2);
    // Here s2 is invalid

    let mut s3 = move_and_get_back(s3);
    borrow_to(&s3);

    // This creates a mutable reference to pass to this function
    change_value(&mut s3);
    println!("Value changed for s3: {}", s3);
    // Note: there cannot be more than one mutable reference in the same scope

    let word_end = get_first_word_end(&s3);
    let word_slice = &s3[..word_end];
    println!("Manually calculated slice: {}", word_slice);

    let word_slice = get_first_word(&s3);
    println!("Returned slice: {}", word_slice);

    // Array slices are also possible
    let a = [2,3,4,5];
    let array_slice = &a[..2];
    println!("Array slice: {}", array_slice[1]);
}

fn move_into_function(s: String) {
    println!("Got ownership of a string: {}", s);
    // s gets destroyed
}

fn move_and_get_back(s: String) -> String {
    println!("Got ownership of a sring: {} -> returning it", s);
    s
}

fn borrow_to(s: &String) {
    println!("Borrowed a string: {}", s);
}

fn change_value(s: &mut String) {
    s.push_str(" Bye!");
}

fn get_first_word_end(s: &str) -> usize {
    // array of bytes
    let bytes = s.as_bytes();

    // iter() lets iterate through the items
    // but here enumerate is better as it gives the index of the item as well
    // then the pattern (a,b) lets us destructure the tuple
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// Returns a string slice
fn get_first_word(s: &str) -> &str {
    &s[..get_first_word_end(s)]
}

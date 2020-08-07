pub fn unsafe_examples() {
    // We can create immutable and mutable raw pointers from references
    let mut num = 5;

    // Rust would not allow us to do the same using references
    let rp1 = &num as *const i32;
    let rp2 = &mut num as *mut i32;

    // We can create raw pointers to arbitrary locations in memory
    let address = 0x012345usize;
    let _rp = address as *const i32;

    // We can dereference raw pointers within unsafe blocks
    unsafe {
        println!("rp1 points to {}", *rp1);
        println!("rp2 points to {}", *rp2);
    }
}

use std::slice;

/// This function takes one slice and makes it two by splitting the slice at the given index
fn split_at_mut(r: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = r.len();
    assert!(mid <= len);

    // Rust can not borrow two mutable references at the same time
    // (&mut r[..mid], &mut r[mid..])
    // But we are borrowing two different parts of the slice, which should be ok

    // We get the raw pointer of the slice
    let ptr = r.as_mut_ptr();
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

pub fn unsafe_functions() {
    // We can mark functions as unsafe
    unsafe fn dangerous_call() {
        // No need to add unsafe blocks in the body of an unsafe function
    }

    // We can call it only within an unsafe block
    unsafe {
        dangerous_call();
    }

    // Wrapping unsafe code in a safe function is a common abstraction
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let mut r = &mut v[..];

    let (a, b) = split_at_mut(&mut r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

// We can define external functions from another language
extern "C" {
    // abs comes from the C standard library and it works out of the box
    fn abs(input: i32) -> i32;
}

// We can expose function to make them accessible from C code
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Rust function called from C");
}

pub fn extern_functions() {
    // External functions are unsafe as other languages don't enforce Rust's rules
    unsafe {
        println!("Abs(-3) = {}", abs(-3));
    }
}

// We can define global variables with static
// By convention screaming snake case with annotated type
// They can only store references with static lifetime
static GLOBAL_VARIABLE: &str = "Global message";

// We can declare mutable static variables
static mut GLOBAL_COUNTER: u32 = 0;

pub fn global_variables() {
    println!("Global variable: {}", GLOBAL_VARIABLE);

    // Accessing and modiying mutable static variables is unsafe
    unsafe {
        GLOBAL_COUNTER += 1;
        println!("Global counter: {}", GLOBAL_COUNTER);
    }
}

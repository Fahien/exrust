// The type of functions is fn, not to be confused with the Fn closure trait.
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

pub fn functions() {
    let answer = do_twice(add_one, 5);
    println!("The answer is {}", answer);

    let numbers = vec![1, 2, 3];
    // We could name a function as the argument to map instead of a closure
    let strings: Vec<String> = numbers.iter().map(ToString::to_string).collect();
    println!("{:?}", strings);
}

// You can't return closures directly as they are represented by traits
fn return_closure() -> Box<dyn Fn(i32) -> i32> {
	Box::new(|x| x + 1)
}

pub fn closures() {
	assert_eq!(return_closure()(2), 3);
}

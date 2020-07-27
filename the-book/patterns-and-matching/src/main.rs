fn play_with_if() {
    // We can use it let instead of match
    // and do some tricks with else branches
    let favourite_color: Option<&str> = Some("green");
    let is_tuesday = true;
    let age: Result<u8, _> = "30".parse();

    if let Some(color) = favourite_color {
        println!("Your favourite color is {}", color);
    } else if is_tuesday {
        println!("Tuesday color is red");
    }
    // We can do shadowing here
    else if let Ok(age) = age {
        if age > 30 {
            println!("Your color is yellow");
        } else {
            println!("Your color is white");
        }
    } else {
        println!("Wow, no color");
    }
}

fn play_with_while() {
    let mut stack = Vec::new();
    stack.push(4);
    stack.push(32);
    stack.push(42);

    // Vec::pop is a method which return an Option with the last element of the
    // vector removing it from the vector, or None of there are no more values
    while let Some(element) = stack.pop() {
        println!("Got {}", element);
    }
}

fn play_with_for() {
    let v = vec!['h', 'e', 'y'];

    // Vec::iter returns an iterator with does not modify the vector.
    // Calling enumerate on the iterator, will adapt it to return
    // a value together with its index
    for (index, value) in v.iter().enumerate() {
        println!("[{}] = {}", index, value);
    }

    assert_eq!(v.len(), 3);
}

fn play_with_let() {
    // Destructure a tuple with let by matching a tuple against a pattern
    let (x, y, z) = (1, 2, 3);
    println!("x[{}] y [{}] z[{}]", x, y, z);
}

fn play_with_fn() {
    // We can match a tuple in a function's arguments to the pattern
    fn print_coords(&(x, y): &(f32, f32)) {
        println!("Coords [{}, {}]", x, y);
    }

    let point = (2.0, 5.0);
    print_coords(&point);
}

fn match_literals() {
    let x = 1;

    // We can match patterns against literals directly
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anythong"),
    }
}

fn match_named_variables() {
    let x = Some(5);
    let y = 10;

    match x {
        // Match against a concrete value in Some
        Some(50) => println!("Got 50"),
        // Match against any value in Some
        // Y here is shadowing the y in the outer scope
        Some(y) => println!("Matched y = {:?}", y),
        // Should print x and y from the outer scope
        _ => println!("Default, x = {:?}, y = {:?}", x, y),
    }

    println!("End: x = {:?}, y = {:?}", x, y);
}

fn match_multiple_patterns() {
    // We can match multiple patterns using |, which means OR.
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        // We can match an inclusive range of values
        // But are only allowed with numbers or char values
        4..=8 => println!("four through eight"),
        _ => println!("anything"),
    }
}

fn destructure_struct() {
    // We can use patters to destructure structs, enums, tuples, and references
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    // Create a and b matching the values of x and y fields of Point
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // Shorthand pattern
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}

fn destructure_enum() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    // The pattern to destructure an enum, should correspond to its definition
    match msg {
        Message::Quit => println!("Quitting"),
        Message::Move { x, y } => println!("x[{}], y[{}]", x, y),
        Message::Write(text) => println!("Msg: {}", text),
        Message::ChangeColor(r, g, b) => println!("Color [R: {}, G: {}, B: {}]", r, g, b),
    }
}

fn play_with_match_guards() {
    let num = Some(42);

    match num {
        // Additional if here, useful to express more complex ideas
        Some(x) if x == 42 => println!("The answer {} is right", x),
        Some(x) => println!("The answer {} is wrong", x),
        None => println!("Still no answer"),
    }
}

fn main() {
    play_with_if();
    play_with_while();
    play_with_for();
    play_with_let();
    play_with_fn();

    match_literals();
    match_named_variables();
    match_multiple_patterns();

    destructure_struct();
    destructure_enum();

    play_with_match_guards();
}

fn vectors() {
    // new function
    let v: Vec<i32> = Vec::new();
    println!("Empty vector {:?}", v);

    // Convenience macro
    let mut v = vec![1, 2, 3];
    println!("Macro vector {:?}", v);

    // Add with push
    v.push(4);

    // Access elements
    // Reference
    let third: &i32 = &v[2];
    println!("Third element {}", third);

    // Get an Option
    match v.get(2) {
        Some(&value) => println!("Got value {}", value),
        None => println!("No value at that index"),
    }

    // Iterate through it
    for i in &v {
        println!("{}", i);
    }

    // Change elements
    for i in &mut v {
        *i += 50;
    }
    println!("Mutated vector {:?}", v);

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f32),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(2),
        SpreadsheetCell::Text(String::from("red")),
        SpreadsheetCell::Float(3.14),
    ];
    println!("Different types with enum values {:?}", row);
}

fn strings() {
    // Empty string with new
    let mut s = String::new();
    println!("Empty {}", s);

    // We can call to_string() on any type that implements Display trait
    // Or we can use String::from("...")
    s = "string 🐧".to_string();
    println!("Initial {}", s);

    // We can append
    s.push(' ');
    s.push_str("with an emoji");
    s += "! And ";
    s = format!("{} {}", s, "nothing else.");
    println!("{}", s);

    // Do not use indexing in strings as they do not implement that operation
    // You can still create string slices with [0..4] for example, but you need to
    // make sure to use valid indices which do not break characters, or crash happens

    // We can iterate through chars
    for c in "Tux🐧".chars() {
        println!("{}", c);
    }

    // We can iterate through bytes
    for b in "Tux🐧".bytes() {
        println!("{}", b);
    }
}

use std::collections::HashMap;

fn hashmaps() {
    // Always new, let it infer the type from the context
    let mut scores = HashMap::new();

    // Owned values like strings are moved here
    scores.insert(String::from("Green"), 42);
    scores.insert(String::from("Purple"), 18);
    println!("Scores {:?}", scores);

    let teams = vec![String::from("White"), String::from("Yellow")];
    let team_scores = vec![12, 57];

    // Create e vector of tuples with zip and create a hashmap out of the vector of tuples
    // We need to specify type HashMap because it is possible to collect for various collections
    // Also, using underscores means just infer the types from the vector of tuples
    let other_scores: HashMap<_,_> = teams.iter().zip(team_scores.iter()).collect();
    println!("Scores {:?}", other_scores);

    // Get values (Option) with get
    let white = String::from("White");
    if let Some(wsc) = scores.get(&white) {
        println!("White score is {}", wsc);
    }

    // We can iterate (arbitrary order)
    for (key, value) in &scores {
        println!("{} {}", key, value);
    }

    // Overwriting a value with insert
    scores.insert(String::from("Purple"), 89);
    println!("Scores {:?}", scores);

    // Inserting if key has no value
    scores.entry(String::from("Purple")).or_insert(192837);
    scores.entry(String::from("Red")).or_insert(7);
    println!("Scores {:?}", scores);

    // Update based on old value
    let red_score = scores.entry(String::from("Red")).or_insert(0);
    *red_score += 50;
    println!("Scores {:?}", scores);
}

mod exvec;

fn main() {
    vectors();
    strings();
    hashmaps();
    exvec::run();
}

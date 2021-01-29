use std::collections::HashMap;
use std::collections::BTreeMap;

fn hashmap() {
    let mut capitals = HashMap::new();

    capitals.insert(String::from("Italy"), String::from("Rome"));
    capitals.insert(String::from("France"), String::from("Paris"));
    capitals.insert(String::from("UK"), String::from("London"));
    capitals.insert(String::from("Spain"), String::from("Madrid"));

    println!("Countries: {{");
    for country in capitals.keys() {
        println!("\t{},", country);
    }
    println!("}}");

    println!("Capitals: {{");
    for capital in capitals.values_mut() {
        *capital = format!("{}?", capital);
        println!("\t{},", capital);
    }
    println!("}}");

    println!("{:?}", capitals);
    for (country, capital) in capitals.iter() {
        println!("{} -> {}", country, capital);
    }
    println!("Capital of Italy is {}", capitals["Italy"]);

    capitals.remove("Spain");
    if let Some(capital) = capitals.get("Spain") {
        println!("Capital of Spain is {}", capital);
    }
}

fn json() {
    let capitals = serde_json::json!({"Italy": "Rome", "France": "Paris", "UK": "London"});
    println!("{}", capitals);
    println!(
        "Capital of France is {}",
        capitals["France"].as_str().expect("Not a string")
    );
}

fn btreemap() {
    let mut populations = BTreeMap::new();

    populations.insert(2_873_000, "Rome");
    populations.insert(3_107_000, "Naples");
    populations.insert(1_352_000, "Milan");

    println!("Cities with a population greater than 2 millions");
    for (pop, city) in populations.range(2_000_000..) {
        println!("{}: {}", city, pop);
    }
}

fn main() {
    hashmap();
    json();
    btreemap();
}

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
struct City {
    name: String,
    population: usize,
    latitute: f64,
    longitude: f64,
}

fn serialization() {
    // We can serialize and deserialize with serde and bincode
    let rome = City {
        name: String::from("Rome"),
        population: 2387000,
        latitute: 41.9,
        longitude: 12.5
    };
    
    let rome_json = serde_json::to_string(&rome).unwrap();
    println!("json: {}", rome_json);

    let rome_cbor = serde_cbor::to_vec(&rome).unwrap();
    println!("cbor: {:?}", rome_cbor);
    println!("cbor (UTF-8) {:?}", String::from_utf8_lossy(&rome_cbor));

    let rome_bincode = bincode::serialize(&rome).unwrap();
    println!("bincode: {:?}", rome_bincode);
    println!("bincode (UTF-8) {:?}", String::from_utf8_lossy(&rome_bincode));
}

fn paths() {
    // Rust provides a class to work with path separators in a cross-platform way
    let mut hello_path = std::path::PathBuf::from("/tmp/hello/hello.txt");

    println!("Extension of {:?} = {:?}", hello_path, hello_path.extension().unwrap());
    print!("Directory of {:?} = ", hello_path);
    hello_path.pop(); // removes "hello.txt"
    println!("{:?}", hello_path);
}

fn main() {
    serialization();
    paths();
}

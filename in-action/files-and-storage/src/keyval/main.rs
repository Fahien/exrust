use keyval::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let usage = format!(
        "Usage:
    {0} <file> get <key>
    {0} <file> delete <key>
    {0} <file> insert <key> <val>
    {0} <file> update <key> <val>",
        args[0]
    );

    // Get arguments
    let file_path = args.get(1).expect(&usage);
    let action = args.get(2).expect(&usage);
    let key = args.get(3).expect(&usage).as_bytes();
    let maybe_value = args.get(4);

    // Open the store file
    let file_path = std::path::Path::new(&file_path);
    let mut store = Store::open(file_path).expect("Failed to open store file");
    store.load().expect("Failed to load data");

    match action.as_ref() {
        "get" => match store.get(key).expect("Failed to get value") {
            Some(value) => println!("{:?}", value),
            None => eprintln!("Key {:?} not found", key),
        },
        "delete" => store.delete(key).expect("No entry to delete"),
        "insert" => {
            let value = maybe_value.expect(&usage).as_ref();
            store.insert(key, value).expect("Failed to insert");
        }
        "update" => {
            let value = maybe_value.expect(&usage).as_ref();
            store.update(key, value).expect("Failed to update");
        }
        _ => eprintln!("{}", usage),
    }
}

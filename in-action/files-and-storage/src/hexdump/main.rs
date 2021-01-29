use std::io::prelude::*;

fn main() {
    // Open a file passed as cli argument
    let program = std::env::args().nth(0).unwrap();
    let arg = std::env::args().nth(1);
    let file_path = arg.expect(&format!("usage: {} <file>", program));
    let mut file = std::fs::File::open(&file_path).expect("Failed to open file");

    // Read content of file into a buffer of bytes
    let mut buffer = vec![];
    file.read_to_end(&mut buffer).unwrap();

    let bytes_per_line = 16;
    let mut position = 0;
    // Print its bytes in hexadecimal format
    for line in buffer.chunks(bytes_per_line) {
        print!("[0x{:08x}] ", position);
        for byte in line {
            print!("{:02x} ", byte);
        }
        println!();
        position += bytes_per_line;
    }
}

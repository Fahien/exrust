extern crate rand;

use rand::Rng;
use std::fmt;

#[derive(PartialEq)]
enum FileState {
    Open,
    Closed,
}

struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl File {
    /// Create a new empty file, and show example of markdown in doc comments.
    ///
    /// # Example
    /// This is **markdown** and it even works with *IDE plugins*.
    /// Also, example of code:
    /// ```
    /// let file = File::new("file.txt");
    /// ```
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    fn new_with_data(name: &str, data: Vec<u8>) -> Self {
        let mut f = Self::new(name);
        f.data = data;
        f
    }

    /// Returns the length of the file in bytes
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns the file's name
    pub fn name(&self) -> String {
        self.name.clone()
    }

    fn open(&mut self) -> Result<(), String> {
        let one_in_ten = rand::thread_rng().gen_weighted_bool(10);
        if one_in_ten {
            return Err(format!("Failed to open file {}", self.name))
        }

        self.state = FileState::Open;
        Ok(())
    }

    fn close(&mut self) {
        self.state = FileState::Closed;
    }
}

/// Traits allow for code reuse as multiple types could implement it.
trait Read {
    /// Reads its own content in bytes and saves it into the `save_to` vector.
    /// Returns Ok with the amount of bytes written into the vector.
    /// Err containing a message otherwise.
    fn read(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

impl Read for File {
    fn read(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading"));
        }

        let mut tmp = self.data.clone();
        let data_len = tmp.len();

        save_to.reserve(data_len);
        save_to.append(&mut tmp);

        Ok(data_len)
    }
}

// While it is possible to rely on the Debug trait, if we want some
// custom formatting for a type we need to implement the Display trait.
impl fmt::Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

fn main() {
    let file = File::new("file.txt");

    let data = vec![114, 117, 115, 116, 33];
    let mut file = File::new_with_data(&file.name, data);

    let mut buffer = vec![];

    file.open().unwrap();
    file.read(&mut buffer).unwrap();
    file.close();
    let text = String::from_utf8_lossy(&buffer);

    println!("{} is {} bytes length", file.name(), file.len());
    println!("{}", text);
}

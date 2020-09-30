use std::fmt;

/// Status of a satellite
#[derive(Debug, PartialEq, Copy, Clone)]
enum Status {
    Ok,
}

/// A message is just a string with a sender and a recipient
struct Message {
    /// Satellite ID
    to: u64,
    content: String,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[to: {}] {}", self.to, self.content)
    }
}

/// A mailbox is just a vector of messages
struct Mailbox {
    messages: Vec<Message>,
}

impl Mailbox {
    /// This function adds a message to the mailbox
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    /// This function returns a message directed to a specific recipient
    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }

        None
    }
}

// Type model for a satellite
// Implementing copy and clone is trivial, as all its members already implement those
#[derive(Copy, Clone, Debug)]
struct CubeSat {
    id: u64,
}

/// This function accepts a reference to a satellite.
/// It does not need to acquire ownership of it.
fn check(sat: &CubeSat) -> Status {
    Status::Ok
}

impl CubeSat {
    /// This satellite receives a message by taking it from the mailbox.
    fn recv(&mut self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

/// A ground station should be able to communicate with a cube sat
struct GroundStation;

impl GroundStation {
    /// This ground station sends a message to a Satellite through
    /// the Mailbox. It does not need ownership of both, but
    /// it should be able to modify the mailbox by pushing a new message
    /// therefore with use a mutable reference.
    fn send(&self, mailbox: &mut Mailbox, to: &CubeSat, msg: String) {
        let msg = Message {
            to: to.id,
            content: msg,
        };
        mailbox.post(msg);
    }

    /// This method creates a connection with a satellite from its id.
    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat { id: sat_id }
    }
}

fn main() {
    let station = GroundStation {};

    // These are satellites
    let mut a = station.connect(0);
    let b = station.connect(1);
    // Calling clone, as opposite as copy, is explicit.
    // This is good, as it warns that it could be expensive.
    let c = a.clone();

    // Make sure everything is ok
    assert_eq!(check(&a), Status::Ok);
    assert_eq!(check(&b), Status::Ok);
    assert_eq!(check(&c), Status::Ok);

    // This is used to communicate with the satellites
    let mut mailbox = Mailbox{ messages: vec![]};

    // A station sends a message to the satellite a
    station.send(&mut mailbox, & a, String::from("How is it going up there?"));

    // The satellites a receives the message and prints it
    let msg = a.recv(&mut mailbox).unwrap();
    println!("Msg: {}", msg);

    // Make sure everything is still ok
    assert_eq!(check(&a), Status::Ok);
    assert_eq!(check(&b), Status::Ok);
    assert_eq!(check(&c), Status::Ok);
}

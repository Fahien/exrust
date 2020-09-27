use std::ops::Deref;

fn dereference() {
    let x = 42;
    // Y is a reference to an i32
    let y = &x;

    // In order to access the value pointed to by y
    // we need to dereference y with *
    println!("y = {}", *y);
}

// Tuple struct with one element of type T
struct CustomBox<T>(T);

impl<T> CustomBox<T> {
    fn new(x: T) -> Self {
        CustomBox(x)
    }
}

// Enable dereferecing for CustomBox with Deref trait
impl<T> Deref for CustomBox<T> {
    // Associated type for the deref trait to use
    // Similar to generic parameters
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

// When CustomBox instances go out of scope,
// Rust will call the drop function of the Drop trait.
// Rust does not allow to call the drop method manually to avoid double free errors,
// but we can use std::mem::drop() instead.
impl<T> Drop for CustomBox<T> {
    fn drop(&mut self) {
        println!("Dropping custom box");
    }
}

// Demonstrate deref coercion
fn hello(name: &str) {
    println!("Hi {}", name);
}

fn custom_box() {
    let x = 42;
    let boxed_x = CustomBox::new(x);

    println!("Boxed x = {}", *boxed_x);
    drop(boxed_x);

    let boxed_name = CustomBox(String::from("Slartibartfast"));
    hello(&boxed_name);
}

use std::cell::RefCell;
use std::rc::{Rc, Weak};

// This uses Rc, reference count, which enables shared ownership of its data
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn reference_count() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(
        Rc::clone(&value),
        Rc::new(Cons(Rc::new(RefCell::new(10)), Rc::new(Nil))),
    ));
    println!("Refcnt a = {}", Rc::strong_count(&a));

    // Rc::clone is used to increase reference count
    // We do not use a.clone() to distinguish reference count increases
    // from other deep-copy kinds of clones
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    println!("Refcnt a = {}", Rc::strong_count(&a));

    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    println!("Refcnt a = {}", Rc::strong_count(&a));

    *value.borrow_mut() += 10;
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);

    drop(c);
    println!("Refcnt a = {}", Rc::strong_count(&a));
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: over quota");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent: over 90% of quota");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: over 75% of quota");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        // Storing this vector in a RefCell, enable us to modify it within the send method
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // Borrow a mutable reference of the vector
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn interior_mutability() {
        // RefCell<T> represents single ownership over the data it holds, like Box<T>. While Box<T>
        // borrowing rules are enforced at compile time, RefCell<T> rules are enforced at runtime.
        // Interior mutability means being able to modify an immutable value with a mutable borrow.
        // In certain situations it would be useful for a value to mutate itself while appearing
        // immutable to other code.
        // Considering test doubles, where a type is used in place of another type during testing.
        // MockMessenger should implement send, which takes a non mutable reference to self.
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // Borrow a non mutable reference of the vector
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    // A node does not own its parent
    parent: RefCell<Weak<Node>>,
    // A node owns its children
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn new(value: i32) -> Node {
        Node {
            value,
            parent: RefCell::new(Weak::new()), // no parent
            children: RefCell::new(vec![]),    // no children
        }
    }
}

fn weak_reference() {
    let leaf = Rc::new(Node::new(3));
    // We borrow from RefCell a reference of Weak, which is a weak reference,
    // and upgrade Weak to Rc which is a strong reference
    println!("Leaf parent = {:?}", leaf.parent.borrow().upgrade());
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // Create a weak reference from a strong reference of branch
    // to set it as parent of leaf node
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    // Weak node references are printed as (Weak)
    println!("Leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

fn main() {
    dereference();
    custom_box();
    reference_count();
    weak_reference();
}

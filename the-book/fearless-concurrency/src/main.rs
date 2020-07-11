use std::thread;
use std::time::Duration;
use std::sync::Arc;

fn spawn_a_thread() {
    // This thread will be stopped when the main thread ends
    thread::spawn(|| {
        for i in 1..10 {
            println!("Thread counting: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
}

fn join_threads() {
    // Spawn returns a join handle
    let handle = thread::spawn(|| {
        for i in 31..40 {
            println!("Join thread counting: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Calling handle's join method will wait for its thread to finish
    // If we do not call it, when main stops executing, the spawned thread will not run.
    handle.join().expect("Failed to join a thread");
}

fn move_in_thread() {
    let v = vec![1, 2, 3];

    // Without using move, the closure passed to the spawn function, will borrow v.
    // Rust won't compile it, because the thread may outlive the current one which
    // owns v. Therefore, to fix that, we move the ownership of v to the thread.
    thread::spawn(move || {
        println!("Vector is {:?}", &v);
    });
}

use std::sync::mpsc;

fn pass_messages() {
    // Create a sender and a receiver which are end points of a channel
    // Tx and Rx are traditional names
    let (tx, rx) = mpsc::channel();

    // We spawn a thread and send a message through the transmitter
    thread::spawn(move || {
        let hi = String::from("Hi");
        // If there is no receiving end, send will return an error
        // Send is non-blocking
        tx.send(hi).expect("No receiving ends");
    });

    // Recv is blocking until a message is received. If the sending end is closed
    // it will return an error as no messages will be sent.
    let received = rx.recv().expect("No values will come");
    println!("Received: {}", received);
}

fn pass_multiple_messages() {
    let (tx, rx) = mpsc::channel();

    // Send multiple messages through the transmitter
    thread::spawn(move || {
        let messages = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("a"),
            String::from("thread"),
        ];

        for m in messages {
            tx.send(m).expect("No receiving ends");
        }
    });

    // We can treat rx as an iterator
    for m in rx {
        println!("Iterating: {}", m);
    }
}

fn multiple_transmitters() {
    let (tx, rx) = mpsc::channel();

    // We can clone the transmitter
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let messages = vec![
            String::from("Hello"),
            String::from("from"),
            String::from("one"),
        ];

        for m in messages {
            tx1.send(m).expect("No receiving ends");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // They will send to the same receiver
    thread::spawn(move || {
        let messages = vec![
            String::from("Hello"),
            String::from("from"),
            String::from("two"),
        ];

        for m in messages {
            tx.send(m).expect("No receiving ends");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // This will receive messages from both threads
    for m in rx {
        println!("Got: {}", m);
    }
}

use std::sync::Mutex;

fn mutexes()
{
    // Mutex is an abbreviation for mutual exclusion.
    // Before accessing data in a mutex, we need to acquire its lock.
    // The lock keeps track of who currently has access.
    // When done with the data, it must be unlocked.
    // Let's create a Mutex<T>.
    let m = Mutex::new(5);

    {
        // Mutex is a smart pointer. Calling lock, returns another smart pointer called MutexGuard.
        // A MutexGuard releases the lock automatically when it goes out of scope.
        // Note that a mutex provides interior mutability, like Cell, and you can still have deadlocks!
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

fn shared_mutex() {
    // Rc is not safe as reference count is not updated in a thread safe way.
    // We instead need to use Atomic reference counting.
    // Arc implements the Send trait so it can be transferred between threads.
    let m = Arc::new(Mutex::new(5));
    let mut handles = vec![];

    // We can not move a mutex to multiple threads, as we need multiple owners
    for _ in 0..10 {
        let counter = Arc::clone(&m);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("m = {:?}", m);
}

fn main() {
    spawn_a_thread();
    join_threads();
    move_in_thread();
    pass_messages();
    pass_multiple_messages();
    multiple_transmitters();
    mutexes();
    shared_mutex();

    for i in 11..15 {
        println!("Main counting: {}", i);
        // Stopping the main thread allows other threads to run
        thread::sleep(Duration::from_millis(1));
    }
}

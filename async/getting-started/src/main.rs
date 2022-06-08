use std::thread;

// A Future is an asynchronous computation that can produce a value
use futures::{executor::block_on, join};

fn download(_url: &str) {
	todo!()
}

fn get_two_sites_with_threads() {
	let thread0 = thread::spawn(|| download("https://www.antoniocaggiano.eu"));
	let thread1 = thread::spawn(|| download("https://www.gameloop.it"));

	thread0.join().expect("Thread 0 panicked");
	thread1.join().expect("Thread 1 panicked");
}

// Value returned from an async fn is a Future.
// A Future needs to be run on an executor.
async fn download_async(_url: &str) {
	todo!()
}

async fn get_two_sites_with_async() {
	// Create futures that will be run later
	let future0 = download_async("https://www.antoniocaggiano.eu");
	let future1 = download_async("https://www.gameloop.it");

	// Available in the futures crate
	join!(future0, future1);
}

fn concurrent_downloading() {
	get_two_sites_with_threads();

	// This function blocks the caller until future is completed
	// Inside an async fn we could use .await, which does not block
	// the current thread, but asynchronously waits for the future.
	block_on(get_two_sites_with_async());
}

fn main() {
	concurrent_downloading();
}
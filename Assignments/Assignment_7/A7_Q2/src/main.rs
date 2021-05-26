use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

fn main() {
    let mut sample_data = Arc::new(Mutex::new(vec![1, 81, 107]));
    for i in 0..10 {
        let sample_data = Arc::clone(&sample_data);
        thread::spawn(move || { sample_data.lock().unwrap()[0] += i; }); // fails here }
        thread::sleep(Duration::from_millis(50));

        // The value sample_data was moved in the previous iteration of the loop
        // and we can’t move the ownership of sample_data into multiple threads.
    }
    println!("Result: {:#?}", sample_data);
}
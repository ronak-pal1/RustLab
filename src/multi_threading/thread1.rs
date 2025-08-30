use std::{thread, time};

// A basic program to get a idea how threads works;
pub fn run() {
    thread::spawn(|| {
        // Sleeping the spawned thread to see the thread effect
        thread::sleep(time::Duration::from_secs(5));

        println!("The spawned thread runned successfully")
    });

    println!("The main thread runned successfully");
}

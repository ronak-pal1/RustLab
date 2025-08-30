use std::{sync::mpsc, thread};

// A program which calculates the sum of 1 to 100000 by using multiple cores of your computer using message passing (channels)
// For creating channels we will use mpsc (multi producer single consumer) library
// tx is the transmitter and rx is the receiver

pub fn run() {

    let (tx, rx) = mpsc::channel::<i64>();

    // Number of cores in your system
    let cores_count:i64 = 10;

    // creating clones of transmitter such that I can transmit from different spawned threads and a single receiver would receive it (rather then creating multiple channels)

    for core in 0..cores_count {
        let clone_tx = tx.clone();

      thread::spawn(move || {

            let mut val:i64 = 0;

            for i in 10000*core..(10000*(core+1))+1 {
                val += i;
            }

            clone_tx.send(val).unwrap();

            println!("The {}th spawned thread executed successfully", core+1);
        });

    }

    // dropping the main transmitter of the created channel such that all it's clone's also dropped from memory and so that the receiver is not stuck at receiving values 

    drop(tx);


    let mut final_val:i64 = 0;

    // Receiving the values
    for val in rx {
        final_val += val;
    }

    println!("The final calculated value is {}", final_val);

}
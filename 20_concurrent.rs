//! there are too many concepts and I don`t want to write here
//! just check the book
//! - 1:1 model 
//! - M:N model (requires a larger language runtime to manage threads)
//! - runtime
//! Rust needs to have nearly no runtime and cannot compromise on being able to call into C to maintain performance
//! Rust standard library only provides an implementation of 1:1 threading

/// 
mod create_thread {
    use std::thread;
    use std::time::Duration;

    /// there is no guarantee on the order in which threads run!
    /// thus that, it can’t guarantee that the spawned thread will get to run at all
    /// he return type of thread::spawn is JoinHandle. A JoinHandle is an owned value
    /// when we call the join method on it, will wait for its thread to finish
    fn create_with_span() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });
    
        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }

        // make sure the spawned thread finishes before main exits:
        handle.join().unwrap();
    }
}

/// we can`t use variables outside of the new thread`s closure
/// thus we need to bring them in
mod bring_data {
    use std::thread;
    /// we can use the move keyword before the parameter list of a closure to force the closure to take ownership of the values it uses in the environment.
    /// This technique is especially useful when creating new threads in order to transfer ownership of values from one thread to another
    /// By adding the move keyword before the closure, we force the closure to take ownership of the values it’s using, 
    /// rather than allowing Rust to infer that it should borrow the values.
    fn move_values() {
        let v = vec![1, 2, 3];
        // there’s a problem: Rust can’t tell how long the spawned thread will run, so it doesn’t know if the reference to v will always be valid
        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        });
        handle.join().unwrap();
    }
}

/// One major tool Rust has for accomplishing message-sending concurrency is the channel
/// A channel in programming has two halves: a transmitter and a receiver
/// A channel is said to be closed if either the transmitter or receiver half is dropped.
mod message_send {
    // mpsc stands for multiple producer, single consumer
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;
    
    fn usage() {
        // tx: sending end, rx: receiving end
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });
        // block the main thread’s execution and wait until a value is sent down the channel. 
        let rec = rx.recv().unwrap();
        // The try_recv method doesn’t block, but will instead return a Result<T, E> immediately
        // we could write a loop that calls try_recv every so often, handles a message if one is available,
        // and otherwise does other work for a little while until checking again.
        let rec2 = rx.try_recv().unwrap();
        println!("get {}", rec);
    }
    // ownership transfer
    // once the value has been sent to another thread, that thread could modify or drop it before we try to use the value again

    /// we’re not calling the recv function explicitly anymore: instead, we’re treating rx as an iterator.
    /// For each value received, we’re printing it. When the channel is closed, iteration will end.
    fn usage_two() {
        let (tx, rx) = mpsc::channel();
    
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];
    
            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });
    
        for received in rx {
            println!("Got: {}", received);
        }
    }

    /// we could create multiple producers by cloning the transmitter
    fn usage_three() {
        // let (tx, rx) = mpsc::channel();
        // let tx1 = tx.clone();
    }
}

/// Mutex is an abbreviation for mutual exclusion, as in, a mutex allows only one thread to access some data at any given time. 
/// two rules when using lock:
/// - You must attempt to acquire the lock before using the data.
/// - When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock
mod shared_state {
    use std::{sync::{Mutex, Arc}, thread};

    fn usage() {
        let m = Mutex::new(5);
        {
            // To access the data inside the mutex, we use the lock method to acquire the lock
            // This call will block the current thread so it can’t do any work until it’s our turn to have the lock
            // The call to lock would fail if another thread holding the lock panicked
            // call Drop automatically when goes out of scope and release the lock
            let mut num = m.lock().unwrap();
            *num = 6;
        }
        println!("{:?}", m);
    }

    /// while use concurrent, we need to make counter to has multiple ownership
    /// Rc<T> is not safe to share across threads. 
    /// atomics work like primitive types but are safe to share across threads
    fn usage_two() {
        // let counter = Mutex::new(0);
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            // counter was moved into the pervious thread
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();

                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }
}

/// similarities between:
/// - RefCell<T> / Rc<T>
/// - Mutex<T> / Arc<T>
mod compare {
    // Mutex<T> provides interior mutability as the "Cell" family does
}

/// the std::marker traits Sync and Send
mod extension {
    // The Send marker trait indicates that ownership of values of the type implementing Send can be transferred between threads.
    // Rc<T> cannot be send
    // we don’t have to implement those traits manually
    // Manually implementing these traits involves implementing unsafe Rust code
}

fn main() {

}
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Testing Reference Counted... (no creation of threads)");
    rc();

    println!("\nTesting Atomically Reference Counted...");
    arc_mutex();

    println!("\nTesting Mutex...");
    arc_mutex_jobs();

    println!("\nTesting multiple producer, single consumer channel...");
    mpsc();
}

fn rc() {
    // Rc 'Reference Counted'
    // It provides shared ownership of a value for a single thread and
    // counts the number of owners, when counter is 0 it drops the reference
    // to this object, freeing the memory associated with it.
    // RefCell is needed to handle mutability.
    let rc = Rc::new(RefCell::new(String::from("foo")));
    println!("initial rc count: {}", Rc::strong_count(&rc)); // 1

    {
        let _rc_clone1 = Rc::clone(&rc);
        println!("rc count after first clone: {}", Rc::strong_count(&rc)); // 2

        let rc_clone2 = Rc::clone(&rc);
        println!("rc count after second clone: {}", Rc::strong_count(&rc)); // 3

        *rc_clone2.borrow_mut() = String::from("bar"); // change value
    }
    println!(
        "rc count when clones are out of scope: {}",
        Rc::strong_count(&rc)
    ); // 1
    println!("value of rc: {}", rc.borrow()); // "bar"
}

fn arc_mutex() {
    // Arc 'Atomically Reference Counted'
    // It provides shared ownership like Rc but it is thread safe.
    let id = Arc::new(Mutex::new(0));

    let id_clone = Arc::clone(&id);
    let thread1_handle = thread::spawn(move || {
        let mut id = id_clone.lock().unwrap();
        *id = 1;
    });

    let id_clone = Arc::clone(&id);
    let thread2_handle = thread::spawn(move || {
        let mut id = id_clone.lock().unwrap();
        *id = 2;
    });

    let _ = thread1_handle.join();
    let _ = thread2_handle.join();

    let id = match id.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    // the result alternates between 1 and 2,
    // depending which thread writes his value last
    println!("id = {}", *id);
}

fn mpsc() {
    // mpsc 'multiple producer, single consumer'
    // FIFO messaging system
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    thread::spawn(move || {
        tx.send(String::from("this is the first message.")).unwrap();
        tx.send(String::from("this is the second message."))
            .unwrap();
        tx.send(String::from("this is the third message.")).unwrap();
        // send() takes ownership of the sended object
    });

    let receiving_thread_handle = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        while let Ok(received) = rx.recv() {
            println!("Got: {}", received);
        }
    });
    let _ = receiving_thread_handle.join();

    // The following Code wouldn't work because rx was used in the
    // receiving thread but there must be only a single consumer:
    //
    // let received = rx.recv().unwrap();
}

// same functionality like the C/C++ programs about threads
fn arc_mutex_jobs() {
    let count = Arc::new(Mutex::new(0));

    let count_clone = Arc::clone(&count);
    let thread1_handle = thread::spawn(move || {
        do_something(&count_clone);
    });

    let count_clone = Arc::clone(&count);
    let thread2_handle = thread::spawn(move || {
        do_something(&count_clone);
    });

    let _ = thread1_handle.join();
    let _ = thread2_handle.join();
}

fn do_something(val: &Arc<Mutex<i32>>) {
    // The lock() method creates a guard like the lock_guard in C++.
    // When it goes out of scope (RAII), the mutex is unlocked.
    let mut count = val.lock().unwrap();
    *count += 1;
    println!("job {} started", count);
    thread::sleep(Duration::from_secs(1));
    println!("job {} finished", count);
}

use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

fn main() {
    let buffer = Arc::new(Mutex::new(Vec::<i32>::new()));
    println!("Starting producers and consumers..");

    let producer = {
        let producer_buffer = Arc::clone(&buffer);
        thread::spawn(move || {
            loop {
                producer_buffer.lock().unwrap().push(42);
                thread::sleep(Duration::from_millis(150));
            }
        })
    };

    // Printer
    let printer_buffer = Arc::clone(&buffer);
    thread::spawn(move || {
        loop {
            {
                let locked_printer_buffer = printer_buffer.lock().unwrap();
                println!("{:?}", locked_printer_buffer);
            }
            thread::sleep(Duration::from_millis(100));
        }
    });

    let consumer = {
        let consumer_buffer = Arc::clone(&buffer);
        thread::spawn(move || {
            loop {
                {
                    let mut locked_consumer_buffer = consumer_buffer.lock().unwrap();
                    if locked_consumer_buffer.is_empty() {
                        println!("Buffer is empty");
                    }
                    else {
                        locked_consumer_buffer.remove(0);
                    }
                }
                thread::sleep(Duration::from_millis(200));
            }
        })
    };
    producer.join().unwrap();
    consumer.join().unwrap();
}

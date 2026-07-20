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
        {
          let mut locked_producer_buffer = producer_buffer.lock().unwrap();
          if locked_producer_buffer.len() < 5 {
            locked_producer_buffer.push(42);
          }
          else {
            println!("Buffer is full");
          }
        }
        thread::sleep(Duration::from_millis(99));
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
        thread::sleep(Duration::from_millis(100));
      }
    })
  };
  producer.join().unwrap();
  consumer.join().unwrap();
}

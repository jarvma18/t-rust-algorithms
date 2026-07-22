use std::sync::{Mutex, Arc, Condvar};
use std::thread;
use std::time::Duration;
use std::collections::VecDeque;

struct Inner {
  data: VecDeque<i32>
}

struct BoundedBuffer {
  inner: Mutex<Inner>,
  capacity: usize,
  can_produce: Condvar,
  can_consume: Condvar
}

impl BoundedBuffer {
  pub fn new(capacity: usize) -> Self {
    Self {
      inner: Mutex::new(Inner {
        data: VecDeque::with_capacity(capacity),
      }),
      capacity,
      can_produce: Condvar::new(),
      can_consume: Condvar::new()
    }
  }

  pub fn produce(&self, item: i32) {
    let mut inner = self.inner.lock().unwrap();
    while inner.data.len() == self.capacity {
      println!("Buffer is full, cannot produce anymore items to it");
      inner = self.can_produce.wait(inner).unwrap();
    }
    inner.data.push_back(item);
    println!("Produced: {}. Current size: {}", item, inner.data.len());
    self.can_consume.notify_one();
  }

  pub fn consume(&self) {
    let mut inner = self.inner.lock().unwrap();
    while inner.data.is_empty() {
      println!("Buffer is empty, cannot consume anymore items from it");
      inner = self.can_consume.wait(inner).unwrap();
    }
    let item = inner.data.pop_front().unwrap();
    println!("Consumed: {}", item);
    self.can_produce.notify_one();
  }
}

fn main() {
  const CAPACITY: usize = 5;

  println!("\n");
  println!("##########################################");
  println!("Starting producer-consumer problem!");
  println!("stop by pressing CTRL + C");
  println!("##########################################");
  println!("\n");

  let buffer = Arc::new(BoundedBuffer::new(CAPACITY));
  let producer_buffer = Arc::clone(&buffer);
  let consumer_buffer = Arc::clone(&buffer);

  let producer = {
    thread::spawn(move || {
      loop {
        {
          producer_buffer.produce(42);
        }
        thread::sleep(Duration::from_millis(1000));
      }
    })
  };

  let consumer = {
    thread::spawn(move || {
      loop {
        {
          consumer_buffer.consume();
        }
        thread::sleep(Duration::from_millis(1000));
      }
    })
  };
  producer.join().unwrap();
  consumer.join().unwrap();
}

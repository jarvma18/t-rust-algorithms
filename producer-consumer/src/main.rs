use std::sync::{Mutex, Arc, Condvar};
use std::thread;
use std::time::Duration;
use std::collections::VecDeque;
use std::thread::JoinHandle;

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

  pub fn consume(&self) -> i32 {
    let mut inner = self.inner.lock().unwrap();
    while inner.data.is_empty() {
      println!("Buffer is empty, cannot consume anymore items from it");
      inner = self.can_consume.wait(inner).unwrap();
    }
    let item = inner.data.pop_front().unwrap();
    println!("Consumed: {}", item);
    self.can_produce.notify_one();
    item
  }
}

fn spawn_producer(buffer: Arc<BoundedBuffer>) -> JoinHandle<()> {
  thread::spawn(move || {
    loop {
      {
        buffer.produce(42);
      }
      thread::sleep(Duration::from_millis(1000));
    }
  })
}

fn spawn_consumer(buffer: Arc<BoundedBuffer>) -> JoinHandle<()> {
  thread::spawn(move || {
    loop {
      {
        buffer.consume();
      }
      thread::sleep(Duration::from_millis(1000));
    }
  })
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
  let producer = spawn_producer(Arc::clone(&buffer));
  let consumer = spawn_consumer(Arc::clone(&buffer));
  producer.join().unwrap();
  consumer.join().unwrap();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  // Basic test to produce one item and consumer it right away
  fn test_should_produce_and_consumer_item() {
    let buffer = Arc::new(BoundedBuffer::new(1));
    buffer.produce(1);
    assert_eq!(buffer.consume(), 1);
  }

  #[test]
  // Test that buffer is behaving with FIFO (first-in, first-out)
  // principle, checking that each produced item is consumed in
  // right order
  fn test_should_produce_and_consumer_items_in_order() {
    let buffer = Arc::new(BoundedBuffer::new(5));
    buffer.produce(1);
    buffer.produce(2);
    buffer.produce(3);
    buffer.produce(4);
    buffer.produce(5);
    assert_eq!(buffer.consume(), 1);
    assert_eq!(buffer.consume(), 2);
    assert_eq!(buffer.consume(), 3);
    assert_eq!(buffer.consume(), 4);
    assert_eq!(buffer.consume(), 5);
  }

   #[test]
   // This test reproduces the scenario, where buffer is full
   // and producer tries to add more stuff into it. Producer
   // should be blocked until more space is available in the
   // buffer.
    fn producer_blocks_when_buffer_is_full() {
      let buffer = Arc::new(BoundedBuffer::new(1));
      buffer.produce(1);
      let buffer_clone = Arc::clone(&buffer);
      let producer = thread::spawn(move || {
          buffer_clone.produce(2);
      });
      // Give the producer a chance to reach the condition variable.
      thread::sleep(Duration::from_millis(100));
      assert!(!producer.is_finished());
      buffer.consume();
      producer.join().unwrap();
      buffer.consume();
    }

    #[test]
    // This test is created for scenario, where consumer tries
    // to consume buffer when it is empty. Consumer should be
    // blocked until there is something to consume
    fn consumer_blocks_when_buffer_is_empty() {
      let buffer = Arc::new(BoundedBuffer::new(1));
      let buffer_clone = Arc::clone(&buffer);
      let consumer = thread::spawn(move || {
          buffer_clone.consume();
      });
      // Give the consumer a chance to start waiting.
      thread::sleep(Duration::from_millis(100));
      assert!(!consumer.is_finished());
      buffer.produce(42);
      consumer.join().unwrap();
    }

    #[test]
    // This is for scenario where more items are tried to
    // be produces into buffer even though its capacity has
    // been reached.
    fn buffer_respects_capacity() {
      let buffer = Arc::new(BoundedBuffer::new(2));
      buffer.produce(1);
      buffer.produce(2);
      let buffer_clone = Arc::clone(&buffer);
      let producer = thread::spawn(move || {
          buffer_clone.produce(3);
      });
      thread::sleep(Duration::from_millis(100));
      assert!(!producer.is_finished());
      buffer.consume();
      producer.join().unwrap();
      buffer.consume();
      buffer.consume();
    }
}
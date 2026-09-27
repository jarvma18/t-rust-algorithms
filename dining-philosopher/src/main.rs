use std::sync::{Mutex, Arc, Condvar};
use std::thread;
use std::time::Duration;
use std::vec::Vec;
use std::thread::JoinHandle;

#[derive(Debug, Clone)]
struct Fork {
}

#[derive(Debug, Clone)]
enum ForkPreference {
  Left,
  Right
}

#[derive(Debug, Clone)]
enum PhilosopherKind {
  Lazy,
  Impatient,
  NormalBehaving
}

#[derive(Debug)]
struct Philosopher {
  left_fork: Arc<Mutex<Fork>>,
  right_fork: Arc<Mutex<Fork>>,
  fork_preference: ForkPreference,
  dine_count: usize,
  kind: PhilosopherKind
}

#[derive(Debug)]
struct DiningTable {
  philosophers: Vec<Philosopher>
}

impl Fork {
  pub fn new() -> Self {
    Self {

    }
  }
}

impl Philosopher {
  pub fn new(left_fork: Arc<Mutex<Fork>>, right_fork: Arc<Mutex<Fork>>, fork_preference: ForkPreference, kind: PhilosopherKind) -> Self {
    Self {
      left_fork: left_fork,
      right_fork: right_fork,
      fork_preference: fork_preference,
      dine_count: 0,
      kind: kind
    }
  }
}

impl DiningTable {
  pub fn new(seat_amount: usize) -> Self {
    let mut forks = Vec::with_capacity(seat_amount);
    for n in 0..seat_amount {
      forks.push(Arc::new(Mutex::new(Fork::new())));
    }
    let mut philosophers = Vec::with_capacity(seat_amount);
    for n in 0..seat_amount {
      if n == seat_amount - 1 {
        let left_fork = Arc::clone(&forks[n]);
        let right_fork = Arc::clone(&forks[0]);
        philosophers.push(Philosopher::new(left_fork, right_fork, ForkPreference::Left, PhilosopherKind::Impatient));
      }
      else {
        let left_fork = Arc::clone(&forks[n]);
        let right_fork = Arc::clone(&forks[n+1]);
        let is_even = n % 2 == 0;
        if is_even {
          philosophers.push(Philosopher::new(left_fork, right_fork, ForkPreference::Left, PhilosopherKind::NormalBehaving));
        }
        else {
          philosophers.push(Philosopher::new(left_fork, right_fork, ForkPreference::Right, PhilosopherKind::Lazy));
        }
      }
    }
    Self {
      philosophers: philosophers
    }
  }

  pub fn dine(&self) {
    thread::scope(|s| {
      for i in 0..5 {
        s.spawn(move || {
          loop {
            println!("{:?}", self.philosophers[i]);
            thread::sleep(Duration::from_millis(1000));
          }
        });
      }
    });
  }
}

fn main() {
  println!("\n");
  println!("##########################################");
  println!("Starting dining philosophers problem!");
  println!("stop by pressing CTRL + C");
  println!("##########################################");
  println!("\n");

  // Base for dine is done, next must implement the individual
  // philosopher logic based on their attributes
  let dining_table = DiningTable::new(5);
  println!("{:?}", dining_table);
  dining_table.dine();
}
// How to actually implement this dining philosopher?
// * We need to model the philosopher entities (=thread)
// * We need to model the fork situation (but we need somehow to determine the circularity)
// * So we need to somehow model which philosopher sits besides who and which forks they have available
// * Should we model this as the real event:
//    - At the start, create table with n seats
//    - This already determines the forks -> one fork between one seat
//    - Generate philosophers to the table, each gets their own seat and fork references
//      (phils. beside each other share fork references)
// What is the approach into data?
// * A fork must have a lock mechanism
// * A philosopher can only "see" the fork beside him
// * A philosopher is either in eating or idling state
// How to model the mechanism that everybody can eat?
// * A philosopher eats max n seconds, then releases both forks
// * A philosopher puts fork back on table if he hasn't been able to lift the other one for n seconds
// * A philosopher has left/right preference, so each one either tries to grab the first fork with left/right side

// use std::sync::{Mutex, Arc, Condvar};
// use std::thread;
// use std::time::Duration;
// use std::vec::Vec;
// use std::thread::JoinHandle;

// #[derive(Debug, Clone)]
// enum ForkReference {
//   Left,
//   Right
// }

// #[derive(Debug, Clone)]
// struct Fork {
//   in_use: bool
// }

// #[derive(Debug, Clone)]
// struct Philosopher<'a> {
//   right_fork: Arc<Mutex<&'a Fork>>,
//   left_fork: Arc<Mutex<&'a Fork>>,
//   fork_preference: ForkReference
// }

// impl Philosopher<'_> {
//   pub fn new(fork_preference: ForkReference) -> Self {
//     Self {
//       right_fork: Arc::new(Mutex::new(&Fork {
//         in_use: false
//       })),
//       left_fork: Arc::new(Mutex::new(&Fork {
//         in_use: false
//       })),
//       fork_preference: fork_preference
//     }
//   }

//   pub fn set_left_fork(&self, &left_fork: <Arc><Mutex><&Fork>) {
//     self.left_fork = &left_fork;
//   }

//   pub fn dine(&self) {

//   }
// }

// fn create_philosopher(index: usize) -> Philosopher<'static> {
//   let is_even = index % 2 == 0;
//   if is_even {
//     Philosopher::new(ForkReference::Left)
//   }
//   else {
//     Philosopher::new(ForkReference::Right)
//   }
// }

// fn main() {
//   const PHILOSOPHER_AMOUNT: usize = 5;

//   println!("\n");
//   println!("##########################################");
//   println!("Starting dining philosophers problem!");
//   println!("stop by pressing CTRL + C");
//   println!("##########################################");
//   println!("\n");

//   println!("{} dining philosophers are sitting around the table..", PHILOSOPHER_AMOUNT);

//   let mut philosophers = Vec::with_capacity(PHILOSOPHER_AMOUNT);
//   for n in 0..PHILOSOPHER_AMOUNT {
//     let phil = create_philosopher(n);
//     println!("Created phil with fork preference of {:?}", phil.fork_preference);
//     philosophers.push(phil.clone());
//     // 1. philosopher handler (= creating 1. fork and attaching the other side fork for the last philosopher)
//     // center philosophers
//     // last philosopher handler, adding the other fork of 1. philosopher to make whole circle
//     // e.g. with 4 philosophers:
//     //
//     //      4.  f   3.
//     // 
//     //   f              f
//     //               
//     //      1.  f   2.
//   }
//   println!("{:?}", philosophers[0]);
// }

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
  left_fork: Arc<Fork>,
  right_fork: Arc<Fork>,
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
  pub fn new(left_fork: Arc<Fork>, right_fork: Arc<Fork>, fork_preference: ForkPreference, kind: PhilosopherKind) -> Self {
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
      forks.push(Arc::new(Fork::new()));
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
}

fn main() {
  println!("\n");
  println!("##########################################");
  println!("Starting dining philosophers problem!");
  println!("stop by pressing CTRL + C");
  println!("##########################################");
  println!("\n");

  // This works, but we'll need to implement mutex to fork thing next, otherwise
  // we are heading for right direction in this program imo
  println!("{:?}", DiningTable::new(5));
}
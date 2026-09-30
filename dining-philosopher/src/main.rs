use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;
use std::vec::Vec;

#[derive(Debug, Clone)]
struct Fork {
}

#[derive(Debug, Clone, PartialEq, Copy)]
enum ForkPreference {
  Left,
  Right
}

#[derive(Debug, Clone, PartialEq, Copy)]
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
  dine_count: Mutex<usize>,
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
      dine_count: 0.into(),
      kind: kind
    }
  }
}

impl DiningTable {
  pub fn new(seat_amount: usize) -> Self {
    let mut forks = Vec::with_capacity(seat_amount);
    for _n in 0..seat_amount {
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

  fn get_interval_for_philosopher(philosopher_kind: PhilosopherKind) -> u64 {
    match philosopher_kind {
      PhilosopherKind::Lazy => 10_000,
      PhilosopherKind::Impatient => 2_000,
      PhilosopherKind::NormalBehaving => 5_000,
    }
  }

  pub fn increment_dine_counter(&self, i: usize) {
    let mut counter = self.philosophers[i].dine_count.lock().unwrap();
    *counter += 1;
  }

  pub fn dine(&self) {
    thread::scope(|s| {
      for i in 0..5 {
        let interval = Self::get_interval_for_philosopher(self.philosophers[i].kind);
        let fork_preference = self.philosophers[i].fork_preference;
        // println!("{:?}", interval);
        s.spawn(move || {
          loop {
            // println!("{:?}", self.philosophers[i]);
            match fork_preference {
              ForkPreference::Left => {
                println!("{:?} is trying to pick up forks", i);
                let left_ = self.philosophers[i].left_fork.lock().unwrap();
                let right_ = self.philosophers[i].right_fork.lock().unwrap();
                println!("{:?} is eating", i);
                self.increment_dine_counter(i);
                thread::sleep(Duration::from_millis(interval));
              }
              ForkPreference::Right => {
                println!("{:?} is trying to pick up forks", i);
                let right_ = self.philosophers[i].right_fork.lock().unwrap();
                let left_ = self.philosophers[i].left_fork.lock().unwrap();
                println!("{:?} is eating", i);
                self.increment_dine_counter(i);
                thread::sleep(Duration::from_millis(interval));
              }
            }
            println!("{:?} ate", i);
            thread::sleep(Duration::from_millis(interval));
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

  // Now, there is initial philosopher problem solved here
  // continue with this idea:
  // fn try_to_eat(&self, i: usize, timeout: Duration) -> bool {
  //   let (first, second) = match self.philosophers[i].fork_preference {
  //       ForkPreference::Left => (
  //           &self.philosophers[i].left_fork,
  //           &self.philosophers[i].right_fork,
  //       ),
  //       ForkPreference::Right => (
  //           &self.philosophers[i].right_fork,
  //           &self.philosophers[i].left_fork,
  //       ),
  //   };

  //   let Some(_first_guard) = lock_with_timeout(first, timeout) else {
  //       println!("{i} couldn't get first fork -> thinking");
  //       return false;
  //   };

  //   let Some(_second_guard) = lock_with_timeout(second, timeout) else {
  //       println!("{i} couldn't get second fork -> thinking");
  //       return false;
  //   };

  //   println!("{i} is eating");
  //   self.increment_dine_counter(i);
  //   thread::sleep(Duration::from_millis(500));

  //   true
  // }
        //            THINKING
        //             │
        //             ▼
        //       try first fork
        //        /          \
        //   timeout          success
        //      │                 │
        //      ▼                 ▼
        //  THINKING        try second fork
        //                    /        \
        //               timeout      success
        //                  │            │
        //                  ▼            ▼
        //              THINKING       EATING
        //                               │
        //                               ▼
        //                           THINKING

  let dining_table = DiningTable::new(5);
  println!("{:?}", dining_table);
  dining_table.dine();
}
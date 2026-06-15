use std::thread;
use std::time::Duration;

fn producer() {
    println!("Hello, I am a producer!");
}

fn consumer() {
    println!("Hello, I am a consumer!");
}

fn main() {
    println!("Hello, I will start both producer and consumer tasks!");

    thread::spawn(|| {
        for _i in 1..10 {
            producer();
            thread::sleep(Duration::from_millis(100));
        }
    });

    for _i in 1..10 {
        consumer();
        thread::sleep(Duration::from_millis(200));
    }
}

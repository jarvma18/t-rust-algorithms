# Producer and consumer problem

The producer and consumer problem demonstrates synchronization problem between producers and consumers. Producers generate data and place it in the shared buffer. Consumers remove and process the data from the buffer.
This problem demonstrates how different processes can share data without conflicts.

Challenge is to ensure that:

* A producer does not add data to a full buffer
* A consumer does not remove data from an empty buffer
* Multiple producers and consumers don't access the buffer simultaneously, that prevents race conditions

More about the problem can be found here: https://www.geeksforgeeks.org/operating-systems/producer-consumer-problem-using-semaphores-set-1/

## Implementation

This project demonstrates how to implement working and safe solution for producer and consumer problem with Rust using shared memory, mutexes and conditional variables to handle concurrent communication between producer and consumer threads.

### Prerequisites

* Rust (stable)
* Cargo (build tool and package manager)

### Features

* Implements patterns for the classic producer and consumer problem.
* Thread safety for shared data access.
* Limits and checks for BoundedBuffer, which prevents unbounded growth of data and removing of data.
* Using conditional variables (Condvar) to allow producers to wait when buffer is full and consumers to wait when buffer is empty, rather than polling continuously the state of the buffer.
* Using Arc to safely share the buffer across multiple threads.

### Installation

Clone the repository and build the project

```bash
cargo build
```

### Run the example

Run the example using Cargo

```bash
cargo run
```

### Expected outputs

1. Producer logging what was produced and size of the buffer
2. Consumer logging what was consumed
3. When buffer is empty or full, there will be log about that when either producer or consumer tries to act against that state

### Code explanation

The core functionality resides in the `BoundedBuffer` struct, which manages access to the internal data (`VecDeque`) using synchronization primitives:

1.  **`Mutex<Inner>`:** Ensures that only one thread at a time can modify the internal queue (`data`).
2.  **`Condvar::can_produce`:** Used by producers. If the buffer is full, the producer calls `wait()` on this condition variable, pausing execution until a consumer signals that space is available.
3.  **`Condvar::can_consume`:** Used by consumers. If the buffer is empty, the consumer calls `wait()` on this condition variable, pausing execution until a producer signals that an item is available.

### Key Functions:

*   **`produce(item: i32)`:** Attempts to add an item. If full, it blocks and waits for a signal from a consumer.
*   **`consume()`:** Attempts to remove an item. If empty, it blocks and waits for a signal from a producer.

## Technical details (rust primitives)

| Primitive | Purpose in this Context |
| :--- | :--- |
| **`Arc<T>`** | Allows multiple threads to safely share ownership of the `BoundedBuffer`. |
| **`Mutex<T>`** | Ensures mutual exclusion. Only the thread holding the lock can access and modify the buffer data. |
| **`Condvar`** | Facilitates blocking synchronization. It allows threads to wait efficiently until a specific condition (buffer full/empty) is met, signaled by another thread (`notify_one`). |
| **`VecDeque<i32>`** | The underlying data structure used for efficient, double-ended queue operations (FIFO). |

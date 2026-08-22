# Dining philosopher

There are n philosophers sitting in a round table. There is one fork between philosophers and when a philosophers eat, they need two forks to be able to do that. The challenge demonstrates how to share resources without getting into deadlock situation, starvation (as n philosophers cannot eat) and mutual exclusion.

Challenge is to ensure that:

* Philosophers don't end up into deadlock situation (e.g. every philosopher lifting lefts-side fork)
* Philosophers don't end up into starvation (cannot eat at all)
* Philosophers that sit beside each other cannot eat at the same time (only one of them can lift the fork)

More about the problem can be found here: https://www.geeksforgeeks.org/operating-systems/dining-philosopher-problem-using-semaphores/

## Implementation

TODO  

### Prerequisites

* Rust (stable)
* Cargo (build tool and package manager)

### Features

TODO

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

TODO

### Code explanation

TODO

### Key Functions:

TODO

## Technical details (rust primitives)

TODO

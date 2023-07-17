# tpx

Run simple coroutines in a thread pool.

- WIP
- Use a thread pool to spawn lightweight coroutines
- Meant for CPU-bound tasks that work cooperatively
- Continuation-passing style: callbacks automatically scheduled to run in a
  pool of work-stealing threads
- Super simple and fast. No async runtime.

## Example

```rust
use std::{thread::sleep, time::Duration};
use tpx::{continue_with, Cont::DONE, Executor};

fn main() {

    let exec = Executor::init();

    // Spawn 3 tasks, each stopping in the middle and scheduling
    // its own continuation
    for i in 0..3 {
        exec.spawn(move || {
            println!("Hello from task {i}");

            // Some computation to be passed to continuation
            let j = i * 3;
            sleep(Duration::from_millis(123));

            // Yield to the executor.  
            continue_with(move || {
                println!("Hello from continuation {i}:  Result: {j}");
                DONE
            })
        });
    }

    // TODO: block executor until all tasks are done.
    sleep(Duration::from_secs(3));
}

```

use std::{
    thread::sleep,
    time::Duration,
};

use tpx::{
    continue_with,
    Ctn::DONE,
    Executor,
};

fn main() {
    let exec = Executor::init();

    // Spawn 3 tasks: each task schedules its own continuation
    for i in 0..3 {
        exec.spawn(move || {
            println!("Hello from task {i}");

            // Some data to passed to  the continuation
            let j = i * 3;
            sleep(Duration::from_millis(123));

            // Yield to the executor.
            continue_with(move || {
                println!("Hello from continuation {i}:  Result: {j}");
                DONE
            })
        });
    }

    // TODO: block on the executor until all tasks are done.
    sleep(Duration::from_secs(3));
}

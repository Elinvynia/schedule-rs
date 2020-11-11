# schedule-rs

A simple scheduling library inspired by Python's `schedule`.

## Simple usage
```rust
use schedule_rs::prelude::*;

fn main() {
    let function = || {
        println!("Hello World!");
    }
    let task = Schedule::every().minute().run(function);
    TaskRunner::one(task).run();

    std::thread::park();
}
```

[![ci-badge]][ci] [![docs-badge]][docs] [![crate-version]][crate-link]

# schedule-rs

A simple scheduling library inspired by Python's `schedule`.

## Sample usage
```rust
use schedule_rs::prelude::*;

fn main() {
    let function = || {
        println!("Hello World!");
    };
    let task = Schedule::every().minute().run(function);
    TaskRunner::one(task).run();

    std::thread::park();
}
```

[ci]: https://github.com/Elinvynia/schedule-rs/actions?query=workflow%3ARust
[ci-badge]: https://img.shields.io/github/workflow/status/Elinvynia/schedule-rs/Rust/master?style=flat-square
[docs]: https://docs.rs/schedule-rs
[docs-badge]: https://img.shields.io/badge/docs-online-5023dd.svg?style=flat-square
[crate-link]: https://crates.io/crates/schedule-rs
[crate-version]: https://img.shields.io/crates/v/schedule-rs.svg?style=flat-square

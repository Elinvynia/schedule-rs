//! [![ci-badge][]][ci] [![docs-badge][]][docs] [![crate-version]][crate-link]
//!
//! # schedule-rs
//!
//! A simple scheduling library inspired by Python's `schedule`.
//!
//! ## Sample usage
//! ```rust,no_run
//! use schedule_rs::prelude::*;
//!
//! let function = || {
//!     println!("Hello World!");
//! };
//! let task = Schedule::every().minute().run(function);
//! TaskRunner::one(task).run();
//!
//! std::thread::park();
//! ```
//!
//! [ci]: https://github.com/Elinvynia/schedule-rs/actions?query=workflow%3ARust
//! [ci-badge]: https://img.shields.io/github/workflow/status/Elinvynia/schedule-rs/Rust/master?style=flat-square
//! [docs]: https://docs.rs/schedule-rs
//! [docs-badge]: https://img.shields.io/badge/docs-online-5023dd.svg?style=flat-square
//! [crate-link]: https://crates.io/crates/schedule-rs
//! [crate-version]: https://img.shields.io/crates/v/schedule-rs.svg?style=flat-square

#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

use chrono::prelude::*;
use chrono::Duration;
use std::thread;

///! Basic re-exports.
pub mod prelude {
    pub use crate::Schedule;
    pub use crate::TaskRunner;
}

/// Base type for creating tasks.
#[derive(Debug, Copy, Clone)]
pub struct Schedule;

impl Schedule {
    /// Run a task every single interval of time.
    pub fn every() -> ScheduleBuilderOne {
        ScheduleBuilderOne
    }

    /// Run a task every X intervals of time.
    pub fn every_num(amount: u32) -> ScheduleBuilderOnes {
        ScheduleBuilderOnes { amount }
    }
}

/// Builder for time values of 1 (minute, hour, etc).
#[derive(Debug)]
pub struct ScheduleBuilderOne;

impl ScheduleBuilderOne {
    /// Run a task every minute.
    pub fn minute(self) -> ScheduleBuilderTwo {
        ScheduleBuilderTwo {
            interval: Interval::Minute(1),
            limit: None,
        }
    }

    /// Run a task every hour.
    pub fn hour(self) -> ScheduleBuilderTwo {
        ScheduleBuilderTwo {
            interval: Interval::Hour(1),
            limit: None,
        }
    }

    /// Run a task every day.
    pub fn day(self) -> ScheduleBuilderTwo {
        ScheduleBuilderTwo {
            interval: Interval::Day(1),
            limit: None,
        }
    }

    /// Run a task every week.
    pub fn week(self) -> ScheduleBuilderTwo {
        ScheduleBuilderTwo {
            interval: Interval::Week(1),
            limit: None,
        }
    }

    /// Run a task every month.
    pub fn month(self) -> ScheduleBuilderTwo {
        ScheduleBuilderTwo {
            interval: Interval::Month(1),
            limit: None,
        }
    }

    /// Run a task every year.
    pub fn year(self) -> ScheduleBuilderTwo {
        ScheduleBuilderTwo {
            interval: Interval::Year(1),
            limit: None,
        }
    }

    /// Run a task on every Monday.
    pub fn monday(self) -> ScheduleBuilderTwoDates {
        ScheduleBuilderTwoDates {
            interval: Interval::Week(1),
            limit: None,
            day: Weekday::Mon,
            time: None,
        }
    }

    /// Run a task on every Tuesday.
    pub fn tuesday(self) -> ScheduleBuilderTwoDates {
        ScheduleBuilderTwoDates {
            interval: Interval::Week(1),
            limit: None,
            day: Weekday::Tue,
            time: None,
        }
    }

    /// Run a task on every Wednesday.
    pub fn wednesday(self) -> ScheduleBuilderTwoDates {
        ScheduleBuilderTwoDates {
            interval: Interval::Week(1),
            limit: None,
            day: Weekday::Wed,
            time: None,
        }
    }

    /// Run a task on every Thursday.
    pub fn thursday(self) -> ScheduleBuilderTwoDates {
        ScheduleBuilderTwoDates {
            interval: Interval::Week(1),
            limit: None,
            day: Weekday::Thu,
            time: None,
        }
    }

    /// Run a task on every Friday.
    pub fn friday(self) -> ScheduleBuilderTwoDates {
        ScheduleBuilderTwoDates {
            interval: Interval::Week(1),
            limit: None,
            day: Weekday::Fri,
            time: None,
        }
    }

    /// Run a task on every Saturday.
    pub fn saturday(self) -> ScheduleBuilderTwoDates {
        ScheduleBuilderTwoDates {
            interval: Interval::Week(1),
            limit: None,
            day: Weekday::Sat,
            time: None,
        }
    }

    /// Run a task on every Sunday.
    pub fn sunday(self) -> ScheduleBuilderTwoDates {
        ScheduleBuilderTwoDates {
            interval: Interval::Week(1),
            limit: None,
            day: Weekday::Sun,
            time: None,
        }
    }
}

/// Builder for time values of more than one (minutes, hours, etc).
#[derive(Debug, Copy, Clone)]
pub struct ScheduleBuilderOnes {
    amount: u32,
}

impl ScheduleBuilderOnes {
    /// Run a task every X minutes.
    pub fn minutes(self) -> ScheduleBuilderTwo {
        ScheduleBuilderTwo {
            interval: Interval::Minute(self.amount),
            limit: None,
        }
    }

    /// Run a task every X hours.
    pub fn hours(self) -> ScheduleBuilderTwo {
        ScheduleBuilderTwo {
            interval: Interval::Hour(self.amount),
            limit: None,
        }
    }

    /// Run a task every X days.
    pub fn days(self) -> ScheduleBuilderTwo {
        ScheduleBuilderTwo {
            interval: Interval::Day(self.amount),
            limit: None,
        }
    }

    /// Run a task every X weeks.
    pub fn weeks(self) -> ScheduleBuilderTwo {
        ScheduleBuilderTwo {
            interval: Interval::Week(self.amount),
            limit: None,
        }
    }

    /// Run a task every X months.
    pub fn months(self) -> ScheduleBuilderTwo {
        ScheduleBuilderTwo {
            interval: Interval::Month(self.amount),
            limit: None,
        }
    }

    /// Run a task every X years.
    pub fn years(self) -> ScheduleBuilderTwo {
        ScheduleBuilderTwo {
            interval: Interval::Year(self.amount),
            limit: None,
        }
    }

    /// Run a task every X Mondays.
    pub fn monday(self) -> ScheduleBuilderTwoDates {
        ScheduleBuilderTwoDates {
            interval: Interval::Week(self.amount),
            limit: None,
            day: Weekday::Mon,
            time: None,
        }
    }

    /// Run a task every X Tuesdays.
    pub fn tuesday(self) -> ScheduleBuilderTwoDates {
        ScheduleBuilderTwoDates {
            interval: Interval::Week(self.amount),
            limit: None,
            day: Weekday::Tue,
            time: None,
        }
    }

    /// Run a task every X Wednesdays.
    pub fn wednesday(self) -> ScheduleBuilderTwoDates {
        ScheduleBuilderTwoDates {
            interval: Interval::Week(self.amount),
            limit: None,
            day: Weekday::Wed,
            time: None,
        }
    }

    /// Run a task every X Thursdays.
    pub fn thursday(self) -> ScheduleBuilderTwoDates {
        ScheduleBuilderTwoDates {
            interval: Interval::Week(1),
            limit: None,
            day: Weekday::Thu,
            time: None,
        }
    }

    /// Run a task every X Fridays.
    pub fn friday(self) -> ScheduleBuilderTwoDates {
        ScheduleBuilderTwoDates {
            interval: Interval::Week(1),
            limit: None,
            day: Weekday::Fri,
            time: None,
        }
    }

    /// Run a task every X Saturdays.
    pub fn saturday(self) -> ScheduleBuilderTwoDates {
        ScheduleBuilderTwoDates {
            interval: Interval::Week(1),
            limit: None,
            day: Weekday::Sat,
            time: None,
        }
    }

    /// Run a task every X Sundays.
    pub fn sunday(self) -> ScheduleBuilderTwoDates {
        ScheduleBuilderTwoDates {
            interval: Interval::Week(1),
            limit: None,
            day: Weekday::Sun,
            time: None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Interval {
    Minute(u32),
    Hour(u32),
    Day(u32),
    Week(u32),
    Month(u32),
    Year(u32),
}

/// Builder step that allows you to set a limit and provide the function.
#[derive(Debug)]
pub struct ScheduleBuilderTwo {
    interval: Interval,
    limit: Option<u64>,
}

impl ScheduleBuilderTwo {
    /// Limits the amount of times the function will run, optional.
    pub fn limit(mut self, amount: u64) {
        self.limit = Some(amount)
    }

    /// Finishes building the task with a provided function.
    pub fn run<F: Fn() + Send + 'static>(self, fun: F) -> Scheduled<F> {
        Scheduled {
            execution_time: next_interval(self.interval),
            execution_interval: self.interval,
            execution_limit: self.limit,
            function: fun,
        }
    }
}

/// Builder step that allows you to set a limit and provide the function.
/// Also allows to set the time of day.
#[derive(Debug)]
pub struct ScheduleBuilderTwoDates {
    interval: Interval,
    limit: Option<u64>,
    day: Weekday,
    time: Option<NaiveTime>,
}

impl ScheduleBuilderTwoDates {
    /// Limits the amount of times the function will run, optional.
    pub fn limit(mut self, amount: u64) {
        self.limit = Some(amount)
    }

    /// Sets the time of day when the function will run, optional.
    pub fn at(mut self, time: &str) {
        let naive_time = NaiveTime::parse_from_str(time, "%H:%M").unwrap();
        self.time = Some(naive_time)
    }

    /// Finishes building the task with a provided function.
    pub fn run<F: Fn() + Send + 'static>(self, fun: F) -> Scheduled<F> {
        Scheduled {
            execution_time: first_day_exec(self.day, self.time),
            execution_interval: self.interval,
            execution_limit: self.limit,
            function: fun,
        }
    }
}

fn first_day_exec(day: Weekday, time: Option<NaiveTime>) -> DateTime<Utc> {
    let now = Utc::now();
    let current_day = now.day();
    let current_weekday = now.weekday().number_from_monday();
    let to_add = (current_weekday + day.number_from_monday()) % 7;
    let with_day = Utc::now().with_day(current_day + to_add).unwrap();
    match time {
        Some(t) => with_day
            .with_hour(t.hour())
            .unwrap()
            .with_minute(t.minute())
            .unwrap()
            .with_second(t.second())
            .unwrap(),
        None => with_day,
    }
}

fn next_interval(i: Interval) -> DateTime<Utc> {
    match i {
        Interval::Minute(a) => Utc::now() + Duration::minutes(a as i64),
        Interval::Hour(a) => Utc::now() + Duration::hours(a as i64),
        Interval::Day(a) => Utc::now() + Duration::days(a as i64),
        Interval::Week(a) => Utc::now() + Duration::weeks(a as i64),
        Interval::Month(a) => {
            let time = Utc::now();
            let month = time.month();
            time.with_month(month + a).unwrap()
        }
        Interval::Year(a) => {
            let time = Utc::now();
            let year = time.year();
            time.with_year(year + a as i32).unwrap()
        }
    }
}

/// A built task ready to be run by a runner.
#[derive(Debug)]
pub struct Scheduled<F: Fn() + Send + 'static> {
    execution_time: DateTime<Utc>,
    execution_interval: Interval,
    execution_limit: Option<u64>,
    function: F,
}

/// Runner that runs built tasks.
#[derive(Debug)]
pub struct TaskRunner<F: Fn() + Send + 'static> {
    tasks: Vec<Scheduled<F>>,
}

impl<F: Fn() + Send + 'static> TaskRunner<F> {
    /// Create a new runner for running multiple tasks.
    pub fn new(tasks: Vec<Scheduled<F>>) -> TaskRunner<F> {
        TaskRunner { tasks }
    }

    /// Create a new runner for running a single task.
    pub fn one(task: Scheduled<F>) -> TaskRunner<F> {
        TaskRunner { tasks: vec![task] }
    }

    /// Start the runner.
    pub fn run(mut self) {
        thread::spawn(move || loop {
            let current_time = Utc::now();
            for task in self.tasks.iter_mut() {
                if current_time < task.execution_time {
                    continue;
                };
                (task.function)();
                if let Some(x) = task.execution_limit {
                    task.execution_limit = Some(x - 1);
                };
                task.execution_time = next_interval(task.execution_interval);
            }
            self.tasks = self
                .tasks
                .into_iter()
                .filter(|x| match x.execution_limit {
                    Some(x) => x < 1,
                    None => false,
                })
                .collect();
            thread::sleep(Duration::seconds(5).to_std().unwrap())
        });
    }
}

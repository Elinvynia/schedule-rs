use chrono::prelude::*;
use chrono::Duration;
use std::thread;

pub mod prelude {
    pub use crate::Schedule;
    pub use crate::TaskRunner;
}

#[derive(Debug, Copy, Clone)]
pub struct Schedule;

impl Schedule {
    pub fn every() -> ScheduleBuilderOne {
        ScheduleBuilderOne
    }
    pub fn every_num(amount: u32) -> ScheduleBuilderOnes {
        ScheduleBuilderOnes { amount }
    }
}

#[derive(Debug)]
pub struct ScheduleBuilderOne;

impl ScheduleBuilderOne {
    pub fn minute(self) -> ScheduleBuilderTwo {
        ScheduleBuilderTwo {
            interval: Interval::Minute(1),
            limit: None,
        }
    }
    pub fn hour(self) -> ScheduleBuilderTwo {
        ScheduleBuilderTwo {
            interval: Interval::Hour(1),
            limit: None,
        }
    }
    pub fn day(self) -> ScheduleBuilderTwo {
        ScheduleBuilderTwo {
            interval: Interval::Day(1),
            limit: None,
        }
    }
    pub fn week(self) -> ScheduleBuilderTwo {
        ScheduleBuilderTwo {
            interval: Interval::Week(1),
            limit: None,
        }
    }
    pub fn month(self) -> ScheduleBuilderTwo {
        ScheduleBuilderTwo {
            interval: Interval::Month(1),
            limit: None,
        }
    }
    pub fn year(self) -> ScheduleBuilderTwo {
        ScheduleBuilderTwo {
            interval: Interval::Year(1),
            limit: None,
        }
    }
    pub fn monday(self) -> ScheduleBuilderTwoDates {
        ScheduleBuilderTwoDates {
            interval: Interval::Week(1),
            limit: None,
            day: Weekday::Mon,
            time: None,
        }
    }
    pub fn tuesday(self) -> ScheduleBuilderTwoDates {
        ScheduleBuilderTwoDates {
            interval: Interval::Week(1),
            limit: None,
            day: Weekday::Tue,
            time: None,
        }
    }
    pub fn wednesday(self) -> ScheduleBuilderTwoDates {
        ScheduleBuilderTwoDates {
            interval: Interval::Week(1),
            limit: None,
            day: Weekday::Wed,
            time: None,
        }
    }
    pub fn thursday(self) -> ScheduleBuilderTwoDates {
        ScheduleBuilderTwoDates {
            interval: Interval::Week(1),
            limit: None,
            day: Weekday::Thu,
            time: None,
        }
    }
    pub fn friday(self) -> ScheduleBuilderTwoDates {
        ScheduleBuilderTwoDates {
            interval: Interval::Week(1),
            limit: None,
            day: Weekday::Fri,
            time: None,
        }
    }
    pub fn saturday(self) -> ScheduleBuilderTwoDates {
        ScheduleBuilderTwoDates {
            interval: Interval::Week(1),
            limit: None,
            day: Weekday::Sat,
            time: None,
        }
    }
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
pub struct ScheduleBuilderOnes {
    amount: u32,
}

impl ScheduleBuilderOnes {
    pub fn minutes(self) -> ScheduleBuilderTwo {
        ScheduleBuilderTwo {
            interval: Interval::Minute(self.amount),
            limit: None,
        }
    }
    pub fn hours(self) -> ScheduleBuilderTwo {
        ScheduleBuilderTwo {
            interval: Interval::Hour(self.amount),
            limit: None,
        }
    }
    pub fn days(self) -> ScheduleBuilderTwo {
        ScheduleBuilderTwo {
            interval: Interval::Day(self.amount),
            limit: None,
        }
    }
    pub fn weeks(self) -> ScheduleBuilderTwo {
        ScheduleBuilderTwo {
            interval: Interval::Week(self.amount),
            limit: None,
        }
    }
    pub fn months(self) -> ScheduleBuilderTwo {
        ScheduleBuilderTwo {
            interval: Interval::Month(self.amount),
            limit: None,
        }
    }
    pub fn years(self) -> ScheduleBuilderTwo {
        ScheduleBuilderTwo {
            interval: Interval::Year(self.amount),
            limit: None,
        }
    }
    pub fn monday(self) -> ScheduleBuilderTwoDates {
        ScheduleBuilderTwoDates {
            interval: Interval::Week(self.amount),
            limit: None,
            day: Weekday::Mon,
            time: None,
        }
    }
    pub fn tuesday(self) -> ScheduleBuilderTwoDates {
        ScheduleBuilderTwoDates {
            interval: Interval::Week(self.amount),
            limit: None,
            day: Weekday::Tue,
            time: None,
        }
    }
    pub fn wednesday(self) -> ScheduleBuilderTwoDates {
        ScheduleBuilderTwoDates {
            interval: Interval::Week(self.amount),
            limit: None,
            day: Weekday::Wed,
            time: None,
        }
    }
    pub fn thursday(self) -> ScheduleBuilderTwoDates {
        ScheduleBuilderTwoDates {
            interval: Interval::Week(1),
            limit: None,
            day: Weekday::Thu,
            time: None,
        }
    }
    pub fn friday(self) -> ScheduleBuilderTwoDates {
        ScheduleBuilderTwoDates {
            interval: Interval::Week(1),
            limit: None,
            day: Weekday::Fri,
            time: None,
        }
    }
    pub fn saturday(self) -> ScheduleBuilderTwoDates {
        ScheduleBuilderTwoDates {
            interval: Interval::Week(1),
            limit: None,
            day: Weekday::Sat,
            time: None,
        }
    }
    pub fn sunday(self) -> ScheduleBuilderTwoDates {
        ScheduleBuilderTwoDates {
            interval: Interval::Week(1),
            limit: None,
            day: Weekday::Sun,
            time: None,
        }
    }
}

#[derive(Copy, Clone)]
enum Interval {
    Minute(u32),
    Hour(u32),
    Day(u32),
    Week(u32),
    Month(u32),
    Year(u32),
}

pub struct ScheduleBuilderTwo {
    interval: Interval,
    limit: Option<u64>,
}

impl ScheduleBuilderTwo {
    pub fn limit(mut self, amount: u64) {
        self.limit = Some(amount)
    }
    pub fn run<F: Fn() + Send + 'static>(self, fun: F) -> Scheduled<F> {
        Scheduled {
            execution_time: next_interval(self.interval),
            execution_interval: self.interval,
            execution_limit: self.limit,
            function: fun,
        }
    }
}

pub struct ScheduleBuilderTwoDates {
    interval: Interval,
    limit: Option<u64>,
    day: Weekday,
    time: Option<NaiveTime>,
}

impl ScheduleBuilderTwoDates {
    pub fn limit(mut self, amount: u64) {
        self.limit = Some(amount)
    }
    pub fn at(mut self, time: &str) {
        let naive_time = NaiveTime::parse_from_str(time, "%H:%M").unwrap();
        self.time = Some(naive_time)
    }
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
        Some(t) => with_day.with_hour(t.hour()).unwrap().with_minute(t.minute()).unwrap().with_second(t.second()).unwrap(),
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
        },
        Interval::Year(a) => {
            let time = Utc::now();
            let year = time.year();
            time.with_year(year + a as i32).unwrap()
        },
    }
}

pub struct Scheduled<F: Fn() + Send + 'static> {
    execution_time: DateTime<Utc>,
    execution_interval: Interval,
    execution_limit: Option<u64>,
    function: F,
}

pub struct TaskRunner<F: Fn() + Send + 'static> {
    tasks: Vec<Scheduled<F>>,
}

impl<F: Fn() + Send + 'static> TaskRunner<F> {
    pub fn new(tasks: Vec<Scheduled<F>>) -> TaskRunner<F> {
        TaskRunner { tasks }
    }

    pub fn one(task: Scheduled<F>) -> TaskRunner<F> {
        TaskRunner { tasks: vec![task] }
    }

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

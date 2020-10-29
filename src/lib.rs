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
            interval: Duration::minutes(1),
            limit: None,
        }
    }
    pub fn hour(self) -> ScheduleBuilderTwo {
        ScheduleBuilderTwo {
            interval: Duration::hours(1),
            limit: None,
        }
    }
    pub fn week(self) -> ScheduleBuilderTwo {
        ScheduleBuilderTwo {
            interval: Duration::weeks(1),
            limit: None,
        }
    }
    pub fn month(self) -> ScheduleBuilderTwo {
        ScheduleBuilderTwo {
            interval: Duration::days(30),
            limit: None,
        }
    }
    pub fn year(self) -> ScheduleBuilderTwo {
        ScheduleBuilderTwo {
            interval: Duration::days(365),
            limit: None,
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
            interval: Duration::minutes(self.amount.into()),
            limit: None,
        }
    }
    pub fn hours(self) -> ScheduleBuilderTwo {
        ScheduleBuilderTwo {
            interval: Duration::hours(self.amount.into()),
            limit: None,
        }
    }
    pub fn weeks(self) -> ScheduleBuilderTwo {
        ScheduleBuilderTwo {
            interval: Duration::weeks(self.amount.into()),
            limit: None,
        }
    }
    pub fn months(self) -> ScheduleBuilderTwo {
        ScheduleBuilderTwo {
            interval: Duration::days((self.amount * 30).into()),
            limit: None,
        }
    }
    pub fn years(self) -> ScheduleBuilderTwo {
        ScheduleBuilderTwo {
            interval: Duration::days((self.amount * 365).into()),
            limit: None,
        }
    }
}

pub struct ScheduleBuilderTwo {
    interval: Duration,
    limit: Option<u64>,
}

impl ScheduleBuilderTwo {
    pub fn limit(mut self, amount: u64) {
        self.limit = Some(amount)
    }
    pub fn run<F: Fn() + Send + 'static>(self, fun: F) -> Scheduled<F> {
        let limit = self.limit.unwrap_or(u64::MAX);
        Scheduled {
            execution_time: Utc::now() + self.interval,
            execution_interval: self.interval,
            execution_amount: limit,
            function: fun,
        }
    }
}

pub struct Scheduled<F: Fn() + Send + 'static> {
    execution_time: DateTime<Utc>,
    execution_interval: Duration,
    execution_amount: u64,
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
                task.execution_amount -= 1;
                task.execution_time = current_time + task.execution_interval;
            }
            self.tasks = self
                .tasks
                .into_iter()
                .filter(|x| x.execution_amount > 0)
                .collect();
            thread::sleep(Duration::seconds(5).to_std().unwrap())
        });
    }
}

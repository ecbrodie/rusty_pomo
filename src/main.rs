extern crate chrono;
use chrono::{DateTime, Duration, Utc}; // TODO: Why am I getting an IDE warning about Duration?
use std::fmt::{Display, Formatter, Result};

const TIMER_IN_SECONDS: i64 = 5;

struct Timer {
    end_time: DateTime<Utc>,
}

impl Timer {
    fn new() -> Timer {
        Timer {
            // end_time: SystemTime::now().checked_add(Duration::from_secs(5)).expect("Desired timer end time is too far in the future."),
            end_time: Utc::now() + Duration::seconds(TIMER_IN_SECONDS),
        }
    }

    fn seconds_remaining(&self) -> i64 {
        let now = Utc::now();
        (self.end_time - now).num_seconds() // TODO: Is subtraction here correct? Also, why does num_seconds() not autocomplete?
    }
}

impl Display for Timer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.end_time.timestamp())
    }
}

fn main() {
    let timer = Timer::new();
    println!("Starting a timer that ends at {}", timer);

    let mut previous_seconds_remaining = TIMER_IN_SECONDS;
    loop {
        let seconds_remaining = timer.seconds_remaining();

        if seconds_remaining < previous_seconds_remaining {
            println!("{} seconds remaining...", seconds_remaining);
        }
        previous_seconds_remaining = seconds_remaining;

        if seconds_remaining <= 0 {
            break;
        }
    }

    println!("DING DING DING! Times up.");
}

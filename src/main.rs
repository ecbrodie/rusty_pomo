extern crate chrono;
extern crate crossterm;

use chrono::{DateTime, Duration, Utc};
use std::io::{stdout, Stdout, Write};

use crossterm::{cursor::*, terminal::*, ExecutableCommand, Result};

const TIMER_IN_SECONDS: i64 = 5;

struct Timer {
    duration_remaining: Duration,
    end_time: DateTime<Utc>,
}

impl Timer {
    fn new() -> Timer {
        let full_duration = Duration::seconds(TIMER_IN_SECONDS);
        Timer {
            end_time: Utc::now() + full_duration,
            duration_remaining: full_duration
        }
    }

    fn update(&mut self) {
        let now = Utc::now();
        self.duration_remaining = self.end_time - now;
    }

    fn seconds_remaining(&self) -> i64 {
        self.duration_remaining.num_seconds()
    }

    fn done(&self) -> bool {
        self.seconds_remaining() <= 0
    }

    fn as_clock(&self) -> String {
        let num_minutes = self.duration_remaining.num_minutes();
        let remainder_seconds =  self.seconds_remaining() - num_minutes * 60;
        String::from(format!("{:02}:{:02}", num_minutes, remainder_seconds))
    }
}

fn clear_line(stdout: &mut Stdout) -> Result<()> {
    stdout
        .execute(Clear(ClearType::CurrentLine))?
        .execute(RestorePosition)?;

    Ok(())
}

fn main() -> Result<()> {
    let mut timer = Timer::new();
    println!("Starting a timer that lasts {} seconds ({})", TIMER_IN_SECONDS, timer.as_clock());

    let mut stdout = stdout();

    let mut previous_seconds_remaining = TIMER_IN_SECONDS;
    stdout.execute(SavePosition)?;
    loop {
        timer.update();

        if timer.seconds_remaining() < previous_seconds_remaining {
            clear_line(&mut stdout)?;
            print!("{}", timer.as_clock());
            stdout.flush()?;
        }
        previous_seconds_remaining = timer.seconds_remaining();

        if timer.done() {
            println!();
            break;
        }
    }

    println!("DING DING DING! Times up.");

    Ok(())
}

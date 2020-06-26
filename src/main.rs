extern crate chrono;
extern crate crossterm;

use chrono::{DateTime, Duration, Utc};
use std::io::{stdout, Stdout, Write};

use crossterm::{cursor::*, terminal::*, ExecutableCommand, Result};

const TIMER_IN_SECONDS: i64 = 5;

struct Timer {
    end_time: DateTime<Utc>,
}

impl Timer {
    fn new() -> Timer {
        Timer {
            end_time: Utc::now() + Duration::seconds(TIMER_IN_SECONDS),
        }
    }

    fn seconds_remaining(&self) -> i64 {
        let now = Utc::now();
        (self.end_time - now).num_seconds()
    }
}

fn clear_line(stdout: &mut Stdout) -> Result<()> {
    stdout
        .execute(Clear(ClearType::CurrentLine))?
        .execute(RestorePosition)?;

    Ok(())
}

fn main() -> Result<()> {
    let timer = Timer::new();
    println!("Starting a timer that lasts {} seconds", TIMER_IN_SECONDS);

    let mut stdout = stdout();

    let mut previous_seconds_remaining = TIMER_IN_SECONDS;
    stdout.execute(SavePosition)?;
    loop {
        let seconds_remaining = timer.seconds_remaining();

        if seconds_remaining < previous_seconds_remaining {
            clear_line(&mut stdout)?;
            print!("{} seconds remaining...", seconds_remaining);
            stdout.flush()?;
        }
        previous_seconds_remaining = seconds_remaining;

        if seconds_remaining <= 0 {
            clear_line(&mut stdout)?;
            break;
        }
    }

    println!("DING DING DING! Times up.");

    Ok(())
}

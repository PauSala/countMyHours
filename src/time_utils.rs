use anyhow::{bail, Result};
use chrono::offset::LocalResult;
use chrono::{Datelike, TimeZone};
use regex::Regex;
use std::fmt;
use std::ops::{Add, Sub};
use std::str::FromStr;

pub fn is_valid_time(time: &str) -> bool {
    let re = Regex::new(r"^-?(\d+):[0-5][0-9]$").unwrap();
    re.is_match(time)
}

#[derive(Debug)]
pub struct Time {
    pub minutes: i32,
}

impl Time {
    pub fn absolute(&self) -> String {
        Time {
            minutes: self.minutes.abs(),
        }
        .to_string()
    }
}

impl FromStr for Time {
    type Err = anyhow::Error;

    fn from_str(time: &str) -> Result<Time, Self::Err> {
        if !is_valid_time(time) {
            bail!(format!(
                "Time should be in the format HH:MM, received {time}"
            ))
        }
        let sign = if time.starts_with('-') { -1 } else { 1 };
        let time: Vec<&str> = time.trim_start_matches('-').split(":").collect();
        Ok(Time {
            minutes: sign * (time[0].parse::<i32>()? * 60 + time[1].parse::<i32>()?),
        })
    }
}

impl Add for Time {
    type Output = Time;

    fn add(self, other: Time) -> Time {
        Time {
            minutes: self.minutes + other.minutes,
        }
    }
}

impl Sub for Time {
    type Output = Time;

    fn sub(self, other: Time) -> Time {
        Time {
            minutes: self.minutes - other.minutes,
        }
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign = if self.minutes < 0 { "-" } else { "" };
        let minutes = self.minutes.abs();
        write!(f, "{}{:02}:{:02}", sign, minutes / 60, minutes % 60)
    }
}

pub fn get_working_days(mut from: u32) -> i64 {
    let now = chrono::offset::Local::now();
    let month = now.month();
    let year = now.year();
    let mut working_days = 0;
    while let LocalResult::Single(start) =
        chrono::Local.with_ymd_and_hms(year, month, from, 0, 0, 0)
    {
        match start.weekday() {
            chrono::Weekday::Sat | chrono::Weekday::Sun => {}
            _ => working_days += 1,
        }
        from += 1;
    }
    working_days
}

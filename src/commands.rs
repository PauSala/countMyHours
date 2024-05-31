use crate::{
    file_utils::{append_to_file, read_counter},
    time_utils::{get_working_days, Time},
};
use anyhow::{Ok, Result};
use chrono::Datelike;
use chrono::{DateTime, Local};
use colored::*;
use std::str::FromStr;

pub fn handle_add_command(time_str: &str) -> Result<()> {
    let counter = read_counter()?;
    let time = Time::from_str(time_str)?;
    let tmp = Time::from_str("08:00")? - time;
    let new_time = counter + tmp;
    let now: DateTime<Local> = Local::now();
    let metadata = format!("{}\t{}", now.format("%Y:%m:%d"), &time_str);
    append_to_file(&metadata)?;
    append_to_file(&new_time.to_string())?;
    Ok(())
}

pub fn handle_list_command() -> Result<()> {
    let counter = read_counter()?;
    if counter.minutes == 0 {
        println!(
            "{} {}",
            "-".color(colored::Color::TrueColor {
                r: 242,
                g: 167,
                b: 102
            }),
            "You have worked exactly the amount you should!".white()
        );
        return Ok(());
    }
    if counter.minutes < 0 {
        println!(
            "{} {}",
            "-".color(colored::Color::TrueColor {
                r: 242,
                g: 167,
                b: 102
            }),
            format!(
                "You have worked {} more than you should",
                counter.absolute()
            )
            .color(colored::Color::TrueColor {
                r: 109,
                g: 242,
                b: 162
            })
        );
        return Ok(());
    }
    println!(
        "{} {}",
        "-".color(colored::Color::TrueColor {
            r: 242,
            g: 167,
            b: 102
        }),
        format!("You have worked {} less than you should", counter).color(
            colored::Color::TrueColor {
                r: 240,
                g: 102,
                b: 132
            }
        )
    );
    Ok(())
}

pub fn handle_distribute_command(number_of_days: i32) -> Result<()> {
    let counter = read_counter()?;
    if counter.minutes == 0 {
        println!(
            "{} {}",
            "-".color(colored::Color::TrueColor {
                r: 242,
                g: 167,
                b: 102
            }),
            "Nothing to distribute, your balance is 0".white()
        );
        return Ok(());
    }
    let time_per_day = Time {
        minutes: counter.minutes / number_of_days,
    };
    let journey = Time::from_str("08:00")?;
    let total = journey + time_per_day;
    if counter.minutes < 0 {
        println!(
            "{} {}",
            "-".color(colored::Color::TrueColor {
                r: 242,
                g: 167,
                b: 102
            }),
            format!(
                "You should work {} per day during {} days",
                total, number_of_days
            )
            .color(colored::Color::TrueColor {
                r: 109,
                g: 242,
                b: 162
            }),
        );
        return Ok(());
    }
    println!(
        "{} {}",
        "-".color(colored::Color::TrueColor {
            r: 242,
            g: 167,
            b: 102
        }),
        format!(
            "You should work {} per day during {} days",
            total, number_of_days
        )
        .color(colored::Color::TrueColor {
            r: 240,
            g: 102,
            b: 132
        })
    );
    Ok(())
}

pub fn resume() {
    let working_days = get_working_days(1);
    let now = chrono::Local::now();
    let today = get_working_days(now.day());
    let counter = read_counter().unwrap();

    println!(
        "{} {}",
        "⏀".color(colored::Color::TrueColor {
            r: 242,
            g: 167,
            b: 102
        }),
        format!(
            "{} WDays\t|\t{} left\t|\t{} pending",
            working_days, today, counter
        )
        .color(colored::Color::TrueColor {
            r: 240,
            g: 125,
            b: 236
        }),
    );
}

pub fn count_hours(mode: &str) -> i64{
    let working_days = get_working_days(1);
    let now = chrono::Local::now();
    let today = get_working_days(now.day());
    match mode {
        "t" => working_days * 8,
        "p" => today * 8,
        _ => panic!("Wrong parameter")
    }
}

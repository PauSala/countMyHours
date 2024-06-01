use crate::{
    config_loader::Config,
    file_utils::{append_to_file, read_counter},
    time_utils::{get_working_days, Time},
};
use anyhow::{bail, Ok, Result};
use chrono::Datelike;
use chrono::{DateTime, Local};
use colored::*;
use std::str::FromStr;

pub fn handle_add_command(time_str: &str, config: &Config) -> Result<()> {
    let counter = read_counter()?;
    let time = Time::from_str(time_str)?;
    let tmp = Time::from_str(&config.schedule.daily_hours)? - time;
    let new_time = counter + tmp;
    let now: DateTime<Local> = Local::now();
    let metadata = format!("{}\t{}", now.format("%Y:%m:%d"), &time_str);
    append_to_file(&metadata)?;
    append_to_file(&new_time.to_string())?;
    Ok(())
}

pub fn format_add_command(time_str: &str, config: &Config) {
    println!(
        "{} {}",
        config
            .list_icon
            .color(config.colors.primary.to_rgb().to_colored()),
        format!("Added {} as your daily worktime", time_str)
            .color(config.colors.secondary.to_rgb().to_colored()),
    );
}

pub fn handle_balance_command() -> Result<Time> {
    let counter = read_counter()?;
    Ok(counter)
}

pub fn format_balance_command(counter: Time, config: &Config) {
    if counter.minutes == 0 {
        println!(
            "{} {}",
            config
                .list_icon
                .color(config.colors.primary.to_rgb().to_colored()),
            "You have worked exactly the amount you should!"
                .color(config.colors.primary.to_rgb().to_colored())
        );
        return;
    }
    if counter.minutes < 0 {
        println!(
            "{} {}",
            config
                .list_icon
                .color(config.colors.primary.to_rgb().to_colored()),
            format!(
                "You have worked {} more than you should",
                counter.absolute()
            )
            .color(config.colors.surplus.to_rgb().to_colored())
        );
        return;
    }
    println!(
        "{} {}",
        config
            .list_icon
            .color(config.colors.primary.to_rgb().to_colored()),
        format!("You have worked {} less than you should", counter)
            .color(config.colors.deficit.to_rgb().to_colored())
    );
}

pub fn handle_distribute_command(
    number_of_days: i32,
    config: &Config,
) -> Result<(Time, Time, i32)> {
    if number_of_days == 0 {
        bail!("You could work endlessly and still find yourself at the beginning, even as the universe stretches into eternity (can't divide by zero)")
    }
    let counter = read_counter()?;
    if counter.minutes == 0 {
        return Ok((counter, Time::from_str("00:00")?, 0));
    }
    let time_per_day = Time {
        minutes: counter.minutes / number_of_days,
    };
    let journey = Time::from_str(&config.schedule.daily_hours)?;
    let total = journey + time_per_day;
    return Ok((counter, total, number_of_days));
}

pub fn format_distribute_command(counter: Time, time: Time, days: i32, config: &Config) {
    if counter.minutes == 0 {
        println!(
            "{} {}",
            config
                .list_icon
                .color(config.colors.primary.to_rgb().to_colored()),
            "Nothing to distribute, your balance is 0"
                .color(config.colors.primary.to_rgb().to_colored())
        );
        return;
    }
    if counter.minutes < 0 {
        println!(
            "{} {}",
            config
                .list_icon
                .color(config.colors.primary.to_rgb().to_colored()),
            format!("You should work {} per day during {} days", time, days)
                .color(config.colors.surplus.to_rgb().to_colored()),
        );
        return;
    }
    println!(
        "{} {}",
        config
            .list_icon
            .color(config.colors.primary.to_rgb().to_colored()),
        format!("You should work {} per day during {} days", time, days)
            .color(config.colors.deficit.to_rgb().to_colored()),
    );
}

pub fn handle_summarize_command(config: &Config) {
    let working_days = get_working_days(1);
    let now = chrono::Local::now();
    let today = get_working_days(now.day());
    let counter = read_counter().unwrap();

    println!(
        "{} {}",
        config
            .summary_icon
            .color(config.colors.primary.to_rgb().to_colored()),
        format!(
            "{} WDays\t|\t{} left\t|\t{} pending",
            working_days, today, counter
        )
        .color(config.colors.secondary.to_rgb().to_colored()),
    );
}

pub fn handle_count_hours(mode: &str, config: &Config) -> Result<i64> {
    let working_days = get_working_days(1);
    let now = chrono::Local::now();
    let today = get_working_days(now.day());
    let working_minutes = Time::from_str(&config.schedule.daily_hours)?;
    let res;
    match mode {
        "t" => res = working_days * working_minutes.minutes as i64 / 60,
        "p" => res = today * working_minutes.minutes as i64 / 60,
        _ => panic!("Wrong parameter"),
    }
    Ok(res)
}

pub fn format_count_hours(hours: i64, config: &Config) {
    println!(
        "{} {}",
        config
            .list_icon
            .to_string()
            .color(config.colors.primary.to_rgb().to_colored()),
        format!("Working hours count: {}", &hours)
            .color(config.colors.secondary.to_rgb().to_colored())
    );
}

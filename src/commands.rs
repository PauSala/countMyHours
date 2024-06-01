use crate::{
    config_loader::Config,
    file_utils::{append_to_file, read_counter},
    formatter::color_format,
    time_utils::{get_working_days, Time},
};
use anyhow::{Ok, Result};
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
    println!(
        "{} {}",
        config
            .list_icon
            .color(config.colors.primary.to_rgb().to_colored()),
        format!("Added {} as your daily worktime", time_str)
            .color(config.colors.secondary.to_rgb().to_colored()),
    );
    Ok(())
}

pub fn handle_balance_command(config: &Config) -> Result<()> {
    let counter = read_counter()?;
    if counter.minutes == 0 {
        println!(
            "{} {}",
            config
                .list_icon
                .color(config.colors.primary.to_rgb().to_colored()),
            "You have worked exactly the amount you should!"
                .color(config.colors.primary.to_rgb().to_colored())
        );
        return Ok(());
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
        return Ok(());
    }
    println!(
        "{} {}",
        config
            .list_icon
            .color(config.colors.primary.to_rgb().to_colored()),
        format!("You have worked {} less than you should", counter)
            .color(config.colors.deficit.to_rgb().to_colored())
    );
    Ok(())
}

pub fn handle_distribute_command(number_of_days: i32, config: &Config) -> Result<()> {
    let counter = read_counter()?;
    if counter.minutes == 0 {
        println!(
            "{} {}",
            config
                .list_icon
                .color(config.colors.primary.to_rgb().to_colored()),
            "Nothing to distribute, your balance is 0"
                .color(config.colors.secondary.to_rgb().to_colored())
        );
        return Ok(());
    }
    let time_per_day = Time {
        minutes: counter.minutes / number_of_days,
    };
    let journey = Time::from_str(&config.schedule.daily_hours)?;
    let total = journey + time_per_day;
    if counter.minutes < 0 {
        println!(
            "{} {}",
            config
                .list_icon
                .color(config.colors.primary.to_rgb().to_colored()),
            format!(
                "You should work {} per day during {} days",
                total, number_of_days
            )
            .color(config.colors.surplus.to_rgb().to_colored()),
        );
        return Ok(());
    }
    println!(
        "{} {}",
        config
            .list_icon
            .color(config.colors.primary.to_rgb().to_colored()),
        format!(
            "You should work {} per day during {} days",
            total, number_of_days
        )
        .color(config.colors.deficit.to_rgb().to_colored()),
    );
    Ok(())
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

pub fn handle_count_hours(mode: &str, config: &Config) -> Result<()> {
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
    println!(
        "{}",
        color_format(vec![
            (
                config.list_icon.to_string(),
                config.colors.primary.to_rgb().to_colored()
            ),
            (
                format!("Working hours count: {}", &res),
                config.colors.secondary.to_rgb().to_colored()
            )
        ])
    );
    Ok(())
}

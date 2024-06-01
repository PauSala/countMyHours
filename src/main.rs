use anyhow::Error;
use clap::Parser;
use colored::*;
use commands::{
    format_add_command, format_balance_command, format_count_hours, format_distribute_command,
    handle_balance_command, handle_count_hours, handle_distribute_command,
    handle_summarize_command,
};
use config_loader::Config;
use file_utils::{delete_last_two_lines, get_config_file_path, init_balance};
use formatter::to_table;

use crate::commands::handle_add_command;
pub mod commands;
pub mod config_loader;
pub mod file_utils;
pub mod formatter;
pub mod time_utils;

const ABOUT_TEXT: &str = "
 ______________________________________________________________________________________
|                                                                                      |
|   Count My Hours! (cmh)                                                              |
|                                                                                      |
|   - A simple CLI to manage your daily worktime.                                      |
|   - Add your total hours every day to keep track of your worktime.                   |
|   - If you have worked more or less than 8 hours, adds or subtracts the difference.  |
|   - You can also distribute your debt/surplus over given days.                       |
|                                                                                      |
 --------------------------------------------------------------------------------------
";

#[derive(Parser)]
#[command(
    version,
    about = "A CLI to manage your daily worktime",
    long_about = ABOUT_TEXT,
    )]
struct Cli {
    /// Adds [hours]:[minutes] to your daily worktime
    #[arg(short, long, value_name = "[hours]:[minutes]")]
    add: Option<String>,

    /// counts the total(t) or pending(p) hours for this month
    #[arg(short, long, value_name = "[t|p]")]
    count: Option<String>,

    /// Lists your current debt/surplus of worktime
    #[arg(short, long)]
    balance: bool,

    /// Summarizes current status
    #[arg(short, long)]
    summarize: bool,

    /// Distributes your current debt/surplus of worktime
    /// over given days, defaults to 5 days
    #[arg(short, long, value_name = "number", num_args = 0..=1, default_missing_value = "5")]
    distribute: Option<i32>,

    /// Undo last addition of time, cannot be used with other flags
    #[arg(short, long)]
    undo: bool,

    /// Sets current balance to zero
    #[arg(short, long)]
    init_balance: bool,

    /// Get raw results, not prettified
    #[arg(short, long)]
    raw: bool,
}

fn main() {
    let cli = Cli::parse();
    let config: Config = Config::from_file(get_config_file_path());
    let mut prettify = true;
    let mut output: Vec<(&str, String)> = Vec::new();

    if cli.raw {
        prettify = false;
    }

    if cli.undo && cli.add.is_some()
        || cli.undo && cli.balance
        || cli.undo && cli.distribute.is_some()
        || cli.undo && cli.init_balance
        || cli.undo && cli.summarize
        || cli.undo && cli.count.is_some()
    {
        println!(
            "{} {}",
            "error:".color(config.colors.error.to_rgb().to_colored()),
            "Undo is not allowed with other flags".white()
        );
        return;
    }

    if cli.undo {
        delete_last_two_lines().unwrap();
        return;
    }

    if cli.init_balance {
        init_balance().unwrap();
    }

    if let Some(add) = cli.add.as_deref() {
        let e = handle_add_command(add, &config);
        match e {
            Ok(_) => {
                if prettify {
                    format_add_command(add, &config);
                }
            }
            Err(e) => format_error(e, &config),
        }
    }

    if cli.balance {
        let balance = handle_balance_command();
        match balance {
            Err(e) => format_error(e, &config),
            Ok(balance) => {
                if prettify {
                    format_balance_command(balance, &config);
                } else {
                    output.push(("Balance", format!("{}", balance)));
                }
            }
        }
    }

    if let Some(distribute) = cli.distribute {
        let e = handle_distribute_command(distribute, &config);
        match e {
            Err(e) => format_error(e, &config),
            Ok((counter, time, days)) => {
                if prettify {
                    format_distribute_command(counter, time, days, &config);
                } else {
                    output.push(("Distribute", format!("{}", time)));
                }
            }
        }
    }

    if let Some(value) = cli.count {
        let hours = handle_count_hours(&value, &config);
        match hours {
            Err(e) => format_error(e, &config),
            Ok(hours) => {
                if prettify {
                    format_count_hours(hours, &config);
                } else {
                    output.push(("CountHours", format!("{}", hours)));
                }
            }
        }
    }

    if cli.summarize {
        handle_summarize_command(&config);
    }

    to_table(&output, &config);
}

pub fn format_error(e: Error, config: &Config) {
    println!(
        "{} {}",
        "error:".color(config.colors.error.to_rgb().to_colored()),
        format!("{}", e).white()
    );
}

use clap::Parser;
use colored::*;
use commands::{count_hours, handle_distribute_command, handle_list_command, resume};
use file_utils::delete_last_two_lines;
use formatter::color_format;

use crate::commands::handle_add_command;
pub mod commands;
pub mod file_utils;
pub mod time_utils;
pub mod formatter;

const ABOUT_TEXT: &str = "
 ______________________________________________________________________________________
|                                                                                      |
|   Count My Hours! (cmh)                                                              |
|                                                                                      |
|   - A simple CLI to manage your daily worktime.                                      |
|   - Add your total hours every day to keep track of your worktime.                   |
|   - If you have worked more or less than 8 hours, adds or subtracts the difference.  |
|   - You can also distribute your debt/surplus over given days.                       |
|   - Assumes 40 hours workweek, 8 hours workday.                                      |
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

    /// Resumes your current status
    #[arg(short, long)]
    resume: bool,

    /// Distributes your current debt/surplus of worktime
    /// over given days, defaults to 5 days
    #[arg(short, long, value_name = "number", num_args = 0..=1, default_missing_value = "5")]
    distribute: Option<i32>,

    /// Undo last addition of time, cannot be used with other flags
    #[arg(short, long)]
    undo: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.undo && cli.add.is_some()
        || cli.undo && cli.balance
        || cli.undo && cli.distribute.is_some()
    {
        println!(
            "{} {}",
            "error:".red(),
            "Undo is not allowed with other flags".white()
        );
        return;
    }

    if cli.undo {
        delete_last_two_lines().unwrap();
        return;
    }

    if let Some(time) = cli.add.as_deref() {
        handle_add_command(time).unwrap();
        println!(
            "{} {}",
            "-".color(colored::Color::TrueColor {
                r: 242,
                g: 167,
                b: 102
            }),
            format!("Added {} as your daily worktime", time).color(colored::Color::TrueColor {
                r: 240,
                g: 125,
                b: 236
            }),
        );
    }

    if cli.balance {
        handle_list_command().unwrap();
    }

    if let Some(distribute) = cli.distribute {
        handle_distribute_command(distribute).unwrap();
    }

    if cli.resume {
        resume();
    }

    if let Some(value) = cli.count {
        let res = count_hours(&value);
        println!("{}", color_format(vec![(&res, colored::Color::Red)] ))
    }
}

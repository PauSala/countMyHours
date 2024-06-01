use std::{cmp::max, fmt::Display};

use colored::*;

use crate::config_loader::Config;
pub fn color_format<T: Display>(strs_and_colors: Vec<(T, Color)>) -> String {
    strs_and_colors
        .into_iter()
        .map(|(str, color)| format!("{}", str).color(color).to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn to_table(data: &Vec<(&str, String)>, config: &Config) {
    if data.is_empty() {
        return;
    }

    let max_title_len = data.iter().map(|(title, _)| title.len()).max().unwrap_or(0);
    let max_value_len = data.iter().map(|(_, value)| value.len()).max().unwrap_or(0);
    let max = max(max_title_len, max_value_len);

    let mut header = String::new();
    let mut results = String::new();
    for (k, v) in data {
        header.push_str(&format!("{:<width$} ", k, width = max));
        results.push_str(&format!("{:<width$} ", v, width = max));
    }
    println!(
        "{}\n{}",
        header.color(config.colors.primary.to_rgb().to_colored()),
        results.color(config.colors.secondary.to_rgb().to_colored())
    );
}

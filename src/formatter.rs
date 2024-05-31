use std::fmt::Display;

use colored::*;
pub fn color_format<T: Display>(strs_and_colors: Vec<(T, Color)>) -> String {
    strs_and_colors
        .into_iter()
        .map(|(str, color)| format!("{}", str).color(color).to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

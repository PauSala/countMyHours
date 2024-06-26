use std::io::BufReader;
use std::{fs::File, path::PathBuf};

use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub struct RgbColor {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Deserialize, Debug)]
pub struct HexColor(String);

impl HexColor {
    pub fn to_rgb(&self) -> RgbColor {
        RgbColor::from_hex(&self.0)
    }
}

impl RgbColor {
    pub fn to_colored(&self) -> colored::Color {
        colored::Color::TrueColor {
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }
    pub fn from_hex(hex: &str) -> RgbColor {
        if hex.len() != 7 {
            return RgbColor { r: 0, g: 0, b: 0 };
        }
        if !hex.starts_with('#') {
            return RgbColor { r: 0, g: 0, b: 0 };
        }
        let hex = hex.trim_start_matches('#').to_lowercase();
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
        RgbColor { r, g, b }
    }
}

#[derive(Deserialize, Debug)]
pub struct Colors {
    pub primary: HexColor,
    pub secondary: HexColor,
    pub surplus: HexColor,
    pub deficit: HexColor,
    pub error: HexColor,
}

impl Colors {
    fn default() -> Self {
        Colors {
            primary: HexColor("#F8E16C".to_string()),
            secondary: HexColor("#F07DEC".to_string()),
            surplus: HexColor("#6DF3A2".to_string()),
            deficit: HexColor("#F06684".to_string()),
            error: HexColor("#00FF00".to_string()),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Schedule {
    pub daily_hours: String,
}

impl Schedule {
    fn default() -> Self {
        Schedule {
            daily_hours: "08:00".to_string(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub colors: Colors,
    pub schedule: Schedule,
    pub list_icon: String,
    pub summary_icon: String,
}

impl Config {
    pub fn default() -> Self {
        Config {
            colors: Colors::default(),
            schedule: Schedule::default(),
            list_icon: " ".to_string(),
            summary_icon: "⏀".to_string(),
        }
    }
}

impl Config {
    pub fn from_file(path: PathBuf) -> Config {
        let file = File::open(path);
        match file {
            Ok(file) => {
                let reader = BufReader::new(file);
                let config: Config =
                    serde_json::from_reader(reader).unwrap_or_else(|_| Config::default());
                config
            }
            Err(_) => {
                return Config::default();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_from_hex() {
        let result = RgbColor::from_hex("#2F3E01");
        assert_eq!(result.r, 47);
        assert_eq!(result.g, 62);
        assert_eq!(result.b, 1);
    }
}

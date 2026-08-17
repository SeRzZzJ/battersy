use std::error::Error;

use std::fs::read_to_string;

const STATUS_PATH: &str = "/sys/class/power_supply/BAT0/status";
const CAPACITY_PATH: &str = "/sys/class/power_supply/BAT0/capacity";

pub enum Colors {
    Red,
    Green,
    Yellow,
    Reset,
}

impl Colors {
    pub fn as_ascii(&self) -> &'static str {
        match self {
            Self::Red => "\x1b[31m",
            Self::Green => "\x1b[32m",
            Self::Yellow => "\x1b[33m",
            Self::Reset => "\x1b[0m",
        }
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let status_raw = read_to_string(STATUS_PATH)?;
    let capacity_raw = read_to_string(CAPACITY_PATH)?;

    println!("{}", format_battery_output(&status_raw, &capacity_raw)?);
    Ok(())
}

fn format_battery_output(status_raw: &str, capacity_raw: &str) -> Result<String, Box<dyn Error>> {
    let status = status_raw.trim();

    let sc = status_color(status);
    let se = status_emoji(status);

    let capacity = capacity_raw.trim().parse::<i32>()?;

    let cc = capacity_color(capacity);
    let pb = create_progress_bar(capacity);

    Ok(format!(
        "{}{}{}{}{}",
        sc.as_ascii(),
        se,
        cc.as_ascii(),
        pb,
        Colors::Reset.as_ascii()
    ))
}

fn status_color(status: &str) -> Colors {
    match status {
        "Charging" => Colors::Red,
        "Not charging" => Colors::Green,
        "Discharging" => Colors::Yellow,
        _ => Colors::Reset,
    }
}

fn status_emoji(status: &str) -> &'static str {
    match status {
        "Charging" => "🡅",
        "Not charging" => "✔",
        "Discharging" => "🡇",
        _ => "✔",
    }
}

fn capacity_color(capacity: i32) -> Colors {
    match capacity {
        0..=19 => Colors::Red,
        20..=49 => Colors::Yellow,
        50..=100 => Colors::Green,
        _ => Colors::Reset,
    }
}

fn create_progress_bar(capacity: i32) -> String {
    let hashes = (capacity / 10) as usize;
    let dashes = 10 - hashes;
    format!(
        "{}% [{}{}]",
        capacity,
        "#".repeat(hashes),
        "-".repeat(dashes),
    )
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_status_color() {
        assert!(matches!(status_color("Charging"), Colors::Red));
        assert!(matches!(status_color("Not charging"), Colors::Green));
        assert!(matches!(status_color("Discharging"), Colors::Yellow));
        assert!(matches!(status_color("Unknown"), Colors::Reset));
    }

    #[test]
    fn test_capacity_color() {
        assert!(matches!(capacity_color(10), Colors::Red));
        assert!(matches!(capacity_color(35), Colors::Yellow));
        assert!(matches!(capacity_color(85), Colors::Green));
        assert!(matches!(capacity_color(150), Colors::Reset));
    }

    #[test]
    fn test_create_progress_bar() {
        assert_eq!(create_progress_bar(0), "0% [----------]");
        assert_eq!(create_progress_bar(50), "50% [#####-----]");
        assert_eq!(create_progress_bar(100), "100% [##########]");
    }

    #[test]
    fn test_format_battery_output() {
        let result = format_battery_output("Charging\n", "42\n").unwrap();
        assert_eq!(result, "\x1b[31m🡅\x1b[33m42% [####------]\x1b[0m");
    }
}

//! Layout utilities that convert raw file inputs into colorized text strings.

use crate::{BatteryStatus, Colors};
use std::error::Error;

/// Synthesizes state metrics into formatted console text blocks.
///
/// # Errors
///
/// Returns an error if state parameters fail string mappings or integers can't parse cleanly.
pub fn format_battery_output(
    status_raw: &str,
    capacity_raw: &str,
) -> Result<String, Box<dyn Error>> {
    let status: BatteryStatus = status_raw.trim().parse()?;

    let sc = status.color();
    let se = status.emoji();

    let capacity = capacity_raw.trim().parse::<i32>()?;

    let cc = capacity_color(capacity);
    let pb = create_progress_bar(capacity);

    Ok(format!(
        "{}{}{} {}{}",
        sc.as_ascii(),
        se,
        cc.as_ascii(),
        pb,
        Colors::Reset.as_ascii()
    ))
}

/// Evaluates absolute boundaries to resolve visual profile tones.
const fn capacity_color(capacity: i32) -> Colors {
    match capacity {
        0..=19 => Colors::Red,
        20..=49 => Colors::Yellow,
        50..=100 => Colors::Green,
        _ => Colors::Reset,
    }
}
/// Assembles visual loading sequences mapped precisely against percentile levels.
#[must_use]
pub fn create_progress_bar(capacity: i32) -> String {
    let hashes = usize::try_from(capacity / 10).unwrap_or(0);
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
    fn test_format_battery_output() {
        let result = format_battery_output("Charging\n", "42\n").unwrap();
        assert_eq!(result, "\x1b[33m🡅\x1b[33m 42% [####------]\x1b[0m");
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
}

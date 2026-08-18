//! Domain model components and custom string parser traits for Battersy.

use std::convert::Infallible;
use std::str::FromStr;

/// ANSI terminal formatting colors used to stylize output text streams.
#[derive(Debug, PartialEq, Eq)]
pub enum Colors {
    /// Failure states, critical limits, or discharging flags.
    Red,
    /// Balanced operating capacities or fully operational symbols.
    Green,
    /// Warning zones or active charging streams.
    Yellow,
    /// Resets formatting back to system defaults.
    Reset,
}

impl Colors {
    /// Maps variant formatting onto raw ANSI sequences.
    #[must_use]
    pub const fn as_ascii(&self) -> &'static str {
        match self {
            Self::Red => "\x1b[31m",
            Self::Green => "\x1b[32m",
            Self::Yellow => "\x1b[33m",
            Self::Reset => "\x1b[0m",
        }
    }
}

/// Represents localized battery activity conditions.
pub enum BatteryStatus {
    /// Connected to external power; filling cell storage.
    Charging,
    /// Connected but stationary storage profiles.
    NotCharging,
    /// Relying completely on battery reservoir resources.
    Discharging,
    /// Unrecognized parameters received from kernel streams.
    Unknown,
}

impl FromStr for BatteryStatus {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Charging" => Ok(Self::Charging),
            "Not charging" => Ok(Self::NotCharging),
            "Discharging" => Ok(Self::Discharging),
            _ => Ok(Self::Unknown),
        }
    }
}

impl BatteryStatus {
    /// Extracts intended highlight settings based on status variant.
    #[must_use]
    pub const fn color(&self) -> Colors {
        match self {
            Self::Charging => Colors::Yellow,
            Self::NotCharging => Colors::Green,
            Self::Discharging => Colors::Red,
            Self::Unknown => Colors::Reset,
        }
    }

    /// Provides matching indicator characters representing cell activities.
    #[must_use]
    pub const fn emoji(&self) -> &'static str {
        match self {
            Self::Charging => "🡅",
            Self::NotCharging | Self::Unknown => "✔",
            Self::Discharging => "🡇",
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_status_color() {
        assert!(matches!(BatteryStatus::Charging.color(), Colors::Yellow));
        assert!(matches!(BatteryStatus::NotCharging.color(), Colors::Green));
        assert!(matches!(BatteryStatus::Discharging.color(), Colors::Red));
        assert!(matches!(BatteryStatus::Unknown.color(), Colors::Reset));
    }

    #[test]
    fn test_status_emoji() {
        assert!(matches!(BatteryStatus::Charging.emoji(), "🡅"));
        assert!(matches!(BatteryStatus::NotCharging.emoji(), "✔"));
        assert!(matches!(BatteryStatus::Discharging.emoji(), "🡇"));
        assert!(matches!(BatteryStatus::Unknown.emoji(), "✔"));
    }
}

//! # Battersy Library
//!
//! Exposes functional conveyors and state layout engine workflows
//! designed to query virtual systems tracking hardware electrical profiles.

use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;

use crate::{
    formatter::format_battery_output,
    kinds::{BatteryStatus, Colors},
    parser::parse_args,
};

pub mod formatter;
pub mod kinds;
pub mod parser;

/// Pulls arguments to evaluate, filter, and stream metrics straight out of sysfs devices.
///
/// # Errors
///
/// Returns an execution dynamic error variant if operational device parameters point
/// to invalid system targets, or if file interactions crash.
pub fn run(args: &mut impl Iterator<Item = String>) -> Result<(), Box<dyn Error>> {
    let battery_path = parse_args(args)?;

    let mut work_dir = PathBuf::from("/sys/class/power_supply").join(battery_path);
    work_dir.push("status");
    let status_raw = read_to_string(&work_dir)?;
    work_dir.pop();
    work_dir.push("capacity");
    let capacity_raw = read_to_string(&work_dir)?;

    println!("{}", format_battery_output(&status_raw, &capacity_raw)?);
    Ok(())
}

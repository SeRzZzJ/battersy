//! Modular verification steps implementing a functional arg-parsing pipeline.

/// Processes the command-line argument conveyor sequence to extract targeted device locations.
///
/// # Errors
///
/// Returns an error string if flag verification rules break or arguments are absent.
pub fn parse_args(args: &mut impl Iterator<Item = String>) -> Result<String, String> {
    verify_flag(args.next().as_ref())?;
    let dir_path = extract_battery_path(args.next().as_ref())?;
    Ok(dir_path)
}

/// Validates that the input flag token matches exact system parameter setups.
///
/// # Errors
///
/// Returns an error if an unknown switch is applied or no parameters are submitted.
pub fn verify_flag(arg: Option<&String>) -> Result<(), String> {
    // arg.as_ref().map_or_else(|| Err("Missing arguments. Expected '-d <path>'".into()), |a| if a == "-d" {
    //                 Ok(())
    //             } else {
    //                 Err("Unknown the argument. Expected '-d'".into())
    //             })`
    // 2. clippy: try: `arg.as_ref().map_or_else(|| Err("Missing arguments. Expected '-d <path>'".into()), |a| if a == "-d" {
    //                    Ok(())
    //                } else {
    //                    Err("Unknown the argument. Expected '-d'".into())
    //                })

    arg.map_or_else(
        || Err("Missing arguments. Expected '-d <path>'".into()),
        |a| {
            if a == "-d" {
                Ok(())
            } else {
                Err("Unknown the argument. Expected '-d'".into())
            }
        },
    )

    // match arg {
    //     Some(ref a) => {
    //         if a == "-d" {
    //             Ok(())
    //         } else {
    //             Err("Unknown the argument. Expected '-d'".into())
    //         }
    //     }
    //     None => Err("Missing arguments. Expected '-d <path>'".into()),
    // }
}

/// Grabs directory names out of token streams while scrubbing leading path noise.
///
/// # Errors
///
/// Returns an error if the path segment parameters are missing entirely.
pub fn extract_battery_path(arg: Option<&String>) -> Result<String, String> {
    let raw_path = arg.ok_or("A flag -d requires a path argument.")?;
    let cleaned_path = raw_path.trim_start_matches('/').to_string();
    Ok(cleaned_path)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_verify_flag_correct() {
        assert!(verify_flag(Some(&"-d".to_string())).is_ok());
    }

    #[test]
    fn test_verify_flag_wrong() {
        let result = verify_flag(Some(&"-x".to_string()));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Unknown the argument. Expected '-d'");
    }

    #[test]
    fn test_verify_flag_missing() {
        let result = verify_flag(None);
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_path_present() {
        assert_eq!(
            extract_battery_path(Some(&"BAT1".to_string())).unwrap(),
            "BAT1"
        );
    }

    #[test]
    fn test_extract_path_missing() {
        let result = extract_battery_path(None);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "A flag -d requires a path argument.");
    }

    #[test]
    fn test_full_conveyor_success() {
        let mut input_args = vec!["-d".to_string(), "BAT0".to_string()].into_iter();
        let parsed = parse_args(&mut input_args).unwrap();
        assert_eq!(parsed, "BAT0");
    }

    #[test]
    fn test_extract_path_with_leading_slash() {
        assert_eq!(
            extract_battery_path(Some(&"/BAT0".to_string())).unwrap(),
            "BAT0"
        );
    }

    #[test]
    fn test_extract_path_with_multiple_slashes() {
        assert_eq!(
            extract_battery_path(Some(&"///BAT1".to_string())).unwrap(),
            "BAT1"
        );
    }

    #[test]
    fn test_extract_path_standard() {
        assert_eq!(
            extract_battery_path(Some(&"BAT0".to_string())).unwrap(),
            "BAT0"
        );
    }
}

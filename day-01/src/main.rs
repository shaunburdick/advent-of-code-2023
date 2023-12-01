use clap::Parser;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::{Path, PathBuf},
    process,
};

use std::error::Error;
use thiserror::Error;

#[derive(Parser)]
struct Cli {
    input_file: PathBuf,
}

fn main() {
    // Get command line arguments
    let args = Cli::parse();

    // Read file from CLI arg
    if let Ok(lines) = read_lines(&args.input_file) {
        // Iterate over lines, converting alpha numbers to numbers, then getting the first and last
        let numbers =
            lines.flat_map(|line| get_calibration_number(alpha_to_numeric(line.unwrap())));

        // Print the sum of all the numbers
        println!("{:?}", numbers.sum::<usize>());
    } else {
        eprintln!("Could not read file: {}", args.input_file.display());
        process::exit(1);
    }
}

#[derive(Error, Debug)]
enum ParseCalibrationError {
    #[error("Unable to find two digits")]
    MissingDigit,
    #[error("Unable to parse characters into number: {0}")]
    InvalidNumber(String),
}

/// Given a string, it will get first and last ascii digits
/// combine them, and parse them into an unsigned integer
///
/// Arguments:
/// - s: The string to parse a calibration number from
fn get_calibration_number(s: String) -> Result<usize, impl Error> {
    let mut numbers: (Option<char>, Option<char>) = (None, None);

    // loop through the string, char by char until we find a digit
    for c in s.chars() {
        if numbers.0.is_none() && c.is_ascii_digit() {
            numbers.0.replace(c);
            break;
        }
    }

    // loop through the string in reverse, char by char until we find a digit
    for c in s.chars().rev() {
        if numbers.1.is_none() && c.is_ascii_digit() {
            numbers.1.replace(c);
            break;
        }
    }

    match numbers {
        (Some(first), Some(second)) => {
            let combined = format!("{first}{second}");
            combined
                .parse::<usize>()
                .map_err(|_| ParseCalibrationError::InvalidNumber(combined))
        }
        _ => Err(ParseCalibrationError::MissingDigit),
    }
}

/// In a string, converts any alpha numbers (one, two, three)
/// into their numerical equivalent (1, 2, 3)
///
/// Examples:
/// - one2three -> 123
/// - abtwocdeeightfg -> ab2cde8fg
///
/// Arguments:
/// - s: The string to convert
fn alpha_to_numeric(s: String) -> String {
    let mut final_string = String::with_capacity(s.len());
    // The index of the next character to visit
    let mut visit_index = 0;

    // The index of the next character to include in the final string
    let mut include_index = 0;

    // Indicate that we matched an alpha number representation
    let mut matched: bool;

    // The Alpha and Digit version of a number
    let alpha_numbers = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    while visit_index < s.len() {
        // take a slice from index forward
        let slice = &s[visit_index..];
        // So far we haven't matched
        matched = false;

        // for each alpha number tuple
        for alpha_number in alpha_numbers {
            if slice.starts_with(alpha_number.0) {
                // add digit to final string
                final_string.push(alpha_number.1);
                // don't include the characters in the number
                include_index = visit_index + alpha_number.0.len();
                // visit the last character of the word
                visit_index += alpha_number.0.len() - 1;
                // we found a match!
                matched = true;
                break;
            }
        }

        // If we visited a char we should include, let's include it
        if visit_index == include_index {
            final_string.push(slice.chars().next().unwrap());
            include_index += 1;
        }

        // if we didn't match anything, let's move to the next char
        if !matched {
            visit_index += 1;
        }
    }

    // We reserved memory for the worst case, we never found a number
    // we can now release any memory we didn't use
    final_string.shrink_to_fit();
    final_string
}

// Yoinked from the "efficient" example on rust by example
// @see https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

#[cfg(test)]
mod tests_day_01 {
    use super::{alpha_to_numeric, get_calibration_number};

    #[test]
    fn test_get_calibration_number() {
        assert_eq!(get_calibration_number(String::from("1abc2")).unwrap(), 12);
        assert_eq!(
            get_calibration_number(String::from("pqr3stu8vwx")).unwrap(),
            38
        );
        assert_eq!(
            get_calibration_number(String::from("a1b2c3d4e5f")).unwrap(),
            15
        );
        assert_eq!(
            get_calibration_number(String::from("treb7uchet")).unwrap(),
            77
        );
    }

    #[test]
    fn test_number_alpha_to_numeric() {
        assert_eq!(
            alpha_to_numeric(String::from("onetwo3fourpickle")),
            "1234pickle"
        );

        assert_eq!(alpha_to_numeric(String::from("twone3eightwo")), "21382");
    }

    #[test]
    fn test_converted_calibration_numbers() {
        assert_eq!(
            get_calibration_number(alpha_to_numeric(String::from("two1nine"))).unwrap(),
            29
        );
        assert_eq!(
            get_calibration_number(alpha_to_numeric(String::from("eightwothree"))).unwrap(),
            83
        );
        assert_eq!(
            get_calibration_number(alpha_to_numeric(String::from("abcone2threexyz"))).unwrap(),
            13
        );
        assert_eq!(
            get_calibration_number(alpha_to_numeric(String::from("xtwone3four"))).unwrap(),
            24
        );
        assert_eq!(
            get_calibration_number(alpha_to_numeric(String::from("4nineeightseven2"))).unwrap(),
            42
        );
        assert_eq!(
            get_calibration_number(alpha_to_numeric(String::from("zoneight234"))).unwrap(),
            14
        );
        assert_eq!(
            get_calibration_number(alpha_to_numeric(String::from("7pqrstsixteen"))).unwrap(),
            76
        );
    }
}

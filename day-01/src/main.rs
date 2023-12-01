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
        let numbers = lines
            .into_iter()
            .flatten()
            .map(|line| get_calibration_number(alpha_to_numeric(line)));

        // Print the sum of all the numbers
        println!("{:?}", numbers.flatten().sum::<usize>());
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
    for c in s.chars() {
        if numbers.0.is_none() {
            if c.is_ascii_digit() {
                numbers.0.replace(c);
            }
        } else {
            break;
        }
    }

    for c in s.chars().rev() {
        if numbers.1.is_none() {
            if c.is_ascii_digit() {
                numbers.1.replace(c);
            }
        } else {
            break;
        }
    }

    match numbers {
        (Some(first), Some(second)) => {
            let combined = format!("{}{}", first, second);
            match combined.parse::<usize>() {
                Ok(num) => Ok(num),
                Err(_) => Err(ParseCalibrationError::InvalidNumber(combined)),
            }
        }
        _ => Err(ParseCalibrationError::MissingDigit),
    }
}

/// In a string, converts any alpha numbers (one, two, three)
/// into their numerical equivalent (1, 2, 3)
///
/// NOTE: This isn't technically true, I need to refactor (if it's worth it)
///
/// Examples:
/// - one2three -> 123
/// - abtwocdeeightfg -> ab2cde8fg
///
/// Arguments:
/// - s: The string to convert
fn alpha_to_numeric(s: String) -> String {
    s.replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
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
            "o1et2o3f4rpickle"
        );

        assert_eq!(
            alpha_to_numeric(String::from("twone3eightwo")),
            "t2o1e3e8t2o"
        );
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

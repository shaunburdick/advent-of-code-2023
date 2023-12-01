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
        if numbers.0.is_none() && c.is_ascii_digit() {
            numbers.0.replace(c);
            break;
        }
    }

    for c in s.chars().rev() {
        if numbers.1.is_none() && c.is_ascii_digit() {
            numbers.1.replace(c);
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
/// Examples:
/// - one2three -> 123
/// - abtwocdeeightfg -> ab2cde8fg
///
/// Arguments:
/// - s: The string to convert
fn alpha_to_numeric(s: String) -> String {
    let mut final_string = String::with_capacity(s.len());
    let mut index = 0;
    let mut include_char = true;

    while index < s.len() {
        // take a slice from index forward
        let slice = &s[index..];

        // Check if the slice starts with a number
        if slice.starts_with("one") {
            // if it does, push the digit onto the final string
            final_string.push('1');
            // advance the index the length of the number string minus 2
            // -1 because we increase the index at the end of the loop
            // -1 so we include the last letter in case its part of the next number
            index += 1;
            // indicate that we can skip including the next char, we are just checking
            // to see if its part of a new number
            // but its value has already been captured
            include_char = false;
        } else if slice.starts_with("two") {
            final_string.push('2');
            index += 1;
            include_char = false;
        } else if slice.starts_with("three") {
            final_string.push('3');
            index += 3;
            include_char = false;
        } else if slice.starts_with("four") {
            final_string.push('4');
            index += 2;
            include_char = false;
        } else if slice.starts_with("five") {
            final_string.push('5');
            index += 2;
            include_char = false;
        } else if slice.starts_with("six") {
            final_string.push('6');
            index += 1;
            include_char = false;
        } else if slice.starts_with("seven") {
            final_string.push('7');
            index += 3;
            include_char = false;
        } else if slice.starts_with("eight") {
            final_string.push('8');
            index += 3;
            include_char = false;
        } else if slice.starts_with("nine") {
            final_string.push('9');
            index += 2;
            include_char = false;
        } else if include_char {
            final_string.push(slice.chars().next().unwrap());
        } else {
            // if we skipped the current char, we can start including the next ones
            include_char = true;
        }

        index += 1;
    }

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

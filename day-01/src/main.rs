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
    let args = Cli::parse();

    // Read file from CLI arg
    if let Ok(lines) = read_lines(&args.input_file) {
        let numbers = lines
            .into_iter()
            .flatten()
            .map(|line| get_calibration_number(alpha_to_numeric(line)));
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

fn get_calibration_number(s: String) -> Result<usize, impl Error> {
    // let num = s.chars().filter(|c| c.is_ascii_digit()).collect::<String>();
    // println!("{}", num);

    // num.parse::<usize>()
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
    // // create a new final string
    // let mut final_string = String::new();

    // // create a buffer with max size of s
    // let mut buffer = String::with_capacity(s.capacity());

    // // iterate over s, adding char one at a time to buffer
    // for char in s.chars() {
    //     buffer.push(char);
    //     // if next char is a digit, push buffer into final string, empty buffer
    //     if char.is_ascii_digit() {
    //         final_string.push_str(buffer.as_str());
    //         buffer.clear();
    //     } else {
    //         // else check buffer for an alpha version of a number
    //         let current_size = buffer.len();
    //         buffer = buffer
    //             .replace("one", "1")
    //             .replace("two", "2")
    //             .replace("three", "3")
    //             .replace("four", "4")
    //             .replace("five", "5")
    //             .replace("six", "6")
    //             .replace("seven", "7")
    //             .replace("eight", "8")
    //             .replace("nine", "9");

    //         // if alpha version found, replace with number and push buffer into final string, empty buffer
    //         if buffer.len() != current_size {
    //             final_string.push_str(buffer.as_str());
    //             buffer.clear();
    //         }
    //     }
    // }

    // // push any remaining values on the string
    // if !buffer.is_empty() {
    //     final_string.push_str(buffer.as_str());
    // }

    // println!(
    //     "{},{},{}",
    //     s,
    //     final_string,
    //     get_calibration_number(final_string.clone()).unwrap()
    // );

    // final_string
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
    use crate::alpha_to_numeric;

    use super::get_calibration_number;

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

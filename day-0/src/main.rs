use clap::Parser;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::{Path, PathBuf},
    process,
};

#[derive(Parser)]
struct Cli {
    input_file: PathBuf,
}

fn main() {
    let args = Cli::parse();

    // Read file from CLI arg
    if let Ok(lines) = read_lines(&args.input_file) {
        let mut elf_count = get_calories_by_elf(lines.into_iter().flatten());
        // sort the list
        elf_count.sort();
        let top_3 = &elf_count[elf_count.len() - 3..elf_count.len()];
        println!(
            "Top 3 total: {}\nTop 3: {:?}",
            top_3.iter().sum::<usize>(),
            top_3
        );
    } else {
        eprintln!("Could not read file: {}", args.input_file.display());
        process::exit(1);
    }
}

/// Get the calorie count by elf
fn get_calories_by_elf<I: Iterator<Item = String>>(calorie_list: I) -> Vec<usize> {
    let mut elves: Vec<usize> = Vec::new();
    let mut current_elf: usize = 0;

    for calorie_count in calorie_list {
        if let Ok(cc) = calorie_count.parse::<usize>() {
            current_elf += cc;
        } else {
            elves.push(current_elf);
            current_elf = 0;
        }
    }

    // capture last value
    elves.push(current_elf);

    elves
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
mod tests {
    use super::get_calories_by_elf;

    #[test]
    fn test_get_calories_by_elf() {
        let example_data = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

        let lines = example_data.split('\n').map(String::from);

        let result = get_calories_by_elf(lines);
        assert_eq!(result, vec![6000, 4000, 11000, 24000, 10000]);
    }
}

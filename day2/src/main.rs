use std::{env, error::Error, fs::read_to_string};

fn is_invalid_one(number: u64) -> bool {
    let log10_num = (number as f64).log10().ceil() as u32;

    (number / 10u64.pow(log10_num / 2)) == (number % 10u64.pow(log10_num / 2))
}

fn is_invalid_two(number: u64) -> bool {
    let log10_num = (number as f64).log10().ceil() as u32;

    (1..=(log10_num / 2))
        .filter(|s| {
            if log10_num.is_multiple_of(*s) {
                let mut number = number;
                let pattern = number % 10u64.pow(*s);
                while number > 0 {
                    if pattern != number % 10u64.pow(*s) {
                        return false;
                    }
                    number /= 10u64.pow(*s);
                }
                true
            } else {
                false
            }
        })
        .peekable()
        .peek()
        .is_some()
}

fn file_string() -> Result<String, Box<dyn Error>> {
    let mut args = env::args();
    args.next(); // throw away program name

    let filename = args.next().ok_or("File name not provided!")?;
    let file = read_to_string(filename)?;

    Ok(file)
}

fn main() -> Result<(), Box<dyn Error>> {
    let invalid_id_sum_one = file_string()?
        .trim()
        .split(',')
        .map(|range| {
            let mut range = range.split('-');
            let start: u64 = range.next().ok_or("Start value invalid")?.parse()?;
            let end: u64 = range.next().ok_or("End value invalid")?.parse()?;

            Ok::<u64, Box<dyn Error>>((start..=end).filter(|n| is_invalid_one(*n)).sum())
        })
        .sum::<Result<u64, _>>()?;

    println!("Invalid ID sum (Part one): {invalid_id_sum_one}");

    let invalid_id_sum_two = file_string()?
        .trim()
        .split(',')
        .map(|range| {
            let mut range = range.split('-');
            let start: u64 = range.next().ok_or("Start value invalid")?.parse()?;
            let end: u64 = range.next().ok_or("End value invalid")?.parse()?;

            Ok::<u64, Box<dyn Error>>((start..=end).filter(|n| is_invalid_two(*n)).sum())
        })
        .sum::<Result<u64, _>>()?;

    println!("Invalid ID sum (Part two): {invalid_id_sum_two}");

    Ok(())
}

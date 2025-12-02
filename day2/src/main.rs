use std::{env, error::Error, fs::read_to_string};

fn is_invalid(number: u64) -> bool {
    let log10_num = (number as f64).log10().ceil() as u32;

    (number / 10u64.pow(log10_num / 2)) == (number % 10u64.pow(log10_num / 2))
}

fn file_string() -> Result<String, Box<dyn Error>> {
    let mut args = env::args();
    args.next(); // throw away program name

    let filename = args.next().ok_or("File name not provided!")?;
    let file = read_to_string(filename)?;

    Ok(file)
}

fn main() -> Result<(), Box<dyn Error>> {
    let invalid_id_sum = file_string()?
        .trim()
        .split(',')
        .map(|range| {
            let mut range = range.split('-');
            let start: u64 = range.next().ok_or("Start value invalid")?.parse()?;
            let end: u64 = range.next().ok_or("End value invalid")?.parse()?;

            Ok::<u64, Box<dyn Error>>((start..=end).filter(|n| is_invalid(*n)).sum())
        })
        .sum::<Result<u64, _>>()?;

    println!("Invalid ID sum: {invalid_id_sum}");

    Ok(())
}

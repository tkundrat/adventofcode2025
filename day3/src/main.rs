use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn file() -> Result<File, Box<dyn Error>> {
    let mut args = env::args();
    args.next(); // throw away program name

    let filename = args.next().ok_or("File name not provided!")?;
    let file = File::open(filename)?;

    Ok(file)
}

fn max_first(vec: &[u8]) -> Option<(usize, &u8)> {
    if vec.is_empty() {
        None
    } else {
        // implement own maximum logic, because we need the FIRST largest
        // element, NOT the LAST!
        let mut max_i: usize = 0;
        let mut max_n: &u8 = &vec[0];
        let vec = &vec[1..]; // cut off 1st element
        for (i, n) in vec.iter().enumerate() {
            if n > max_n {
                max_n = n;
                max_i = i + 1; // add one to fix offset from cutting off
            }
        }
        Some((max_i, max_n))
    }
}

fn biggest_digit(vec: &[u8], last_digit_idx: isize, number_len: usize) -> Option<(usize, &u8)> {
    let left_offset = (last_digit_idx + 1) as usize;
    let vec = &vec[left_offset..=vec.len() - number_len]; // cut off elements until last_digit_idx and last number_len-1 elements

    max_first(vec).map(|(index, value)| (index + left_offset, value))
}

fn str_to_numbers(string: &str) -> Vec<u8> {
    // parsing char numbers by subtracting '0'
    string.trim().chars().map(|c| c as u8 - b'0').collect()
}

fn biggest_joltage_in_list(numbers: &[u8], number_len: usize) -> Option<u64> {
    let mut joltage = 0u64;
    let mut number_len = number_len; // copy as mut
    let mut last_digit_idx = -1;
    while number_len > 0 {
        match biggest_digit(numbers, last_digit_idx, number_len) {
            Some((digit_idx, digit)) => {
                joltage += (*digit as u64) * 10u64.pow(number_len as u32 - 1);
                last_digit_idx = digit_idx as isize;
                number_len -= 1;
            }
            None => return None,
        }
    }

    Some(joltage)
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = file()?;
    let buf_reader = BufReader::new(file).lines();

    let mut total_two_joltage = 0u64;
    let mut total_twelve_joltage = 0u64;
    for line in buf_reader {
        let numbers = str_to_numbers(&line?);
        let two_joltage =
            biggest_joltage_in_list(&numbers, 2).ok_or("Joltage of length 2 not found!")?;
        let twelve_joltage =
            biggest_joltage_in_list(&numbers, 12).ok_or("Joltage of length 12 not found!")?;

        total_two_joltage += two_joltage;
        total_twelve_joltage += twelve_joltage;
    }

    println!("Total length two joltage: {total_two_joltage}");
    println!("Total length twelve joltage: {total_twelve_joltage}");

    Ok(())
}

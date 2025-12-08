use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

struct Ingredients {
    fresh_ranges: Vec<RangeInclusive<u64>>,
    available: Vec<u64>,
}

impl Ingredients {
    fn from(file: File) -> Result<Self, Box<dyn Error>> {
        let mut ingredients = Ingredients {
            fresh_ranges: Vec::new(),
            available: Vec::new(),
        };
        let mut fresh_done = false;
        let buf_reader = BufReader::new(file);
        for line in buf_reader.lines() {
            let line = line?;
            if line.is_empty() {
                // empty line as divider
                fresh_done = true;
            } else if fresh_done {
                // 2nd half - available ingredients
                let ingredient: u64 = line.parse()?;
                ingredients.available.push(ingredient);
            } else {
                // 1st half - fresh ingredients
                let mut iter = line.split('-');

                let first: u64 = iter
                    .next()
                    .ok_or("Start of fresh range could not be read")?
                    .parse()?;
                let last: u64 = iter
                    .next()
                    .ok_or("End of fresh range could not be read")?
                    .parse()?;

                let range = first..=last;

                ingredients.fresh_ranges.push(range);
            }
        }

        Ok(ingredients)
    }

    fn is_fresh(&self, ingredient: &u64) -> bool {
        self.fresh_ranges.iter().any(|r| r.contains(ingredient))
    }

    fn fresh_ingredients(&self) -> Vec<u64> {
        self.available
            .iter()
            .cloned()
            .filter(|a| self.is_fresh(a))
            .collect()
    }
}

fn file() -> Result<File, Box<dyn Error>> {
    let mut args = env::args();
    args.next(); // throw away program name

    let filename = args.next().ok_or("File name not provided!")?;
    let file = File::open(filename)?;

    Ok(file)
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = file()?;

    let ingredients = Ingredients::from(file)?;

    let fresh_number = ingredients.fresh_ingredients().len();

    println!("{fresh_number} ingredients are fresh.");

    Ok(())
}

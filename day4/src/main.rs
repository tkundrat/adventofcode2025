use std::env;
use std::error::Error;
use std::fs::read_to_string;

struct Diagram(Vec<Vec<char>>);

impl Diagram {
    fn from_string(string: String) -> Self {
        Diagram(string.lines().map(|l| l.chars().collect()).collect())
    }

    fn get_char_at(&self, x: usize, y: usize) -> Result<char, Box<dyn Error>> {
        Ok(*self
            .0
            .get(y)
            .ok_or("diagram invalid")?
            .get(x)
            .ok_or("diagram invalid")?)
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn width(&self) -> usize {
        self.0.first().map_or_else(|| 0, |l| l.len())
    }

    fn get_surrounding_rolls(&self, x: usize, y: usize) -> Option<u32> {
        if self.get_char_at(x, y).expect("Diagram malformed") != '@' {
            // we are not a roll
            None
        } else {
            let mut count = 0u32;
            let relative_positions: [(isize, isize); 8] = [
                (-1, -1),
                (0, -1),
                (1, -1),
                (-1, 0),
                (1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
            ];
            let mut already_checked: Vec<(usize, usize)> = Vec::new();
            for relative_position in relative_positions {
                let (x_offset, y_offset) = relative_position;

                let x_with_offset = x as isize + x_offset;
                let x_with_offset = if x_with_offset < 0 {
                    0
                } else if x_with_offset >= self.width() as isize {
                    self.width() - 1
                } else {
                    x_with_offset as usize
                };

                let y_with_offset = y as isize + y_offset;
                let y_with_offset = if y_with_offset < 0 {
                    0
                } else if y_with_offset >= self.width() as isize {
                    self.width() - 1
                } else {
                    y_with_offset as usize
                };

                if x_with_offset == x && y_with_offset == y
                    || already_checked
                        .iter()
                        .any(|(x_a, y_a)| *x_a == x_with_offset && *y_a == y_with_offset)
                {
                    continue;
                }

                if self
                    .get_char_at(x_with_offset, y_with_offset)
                    .expect("Diagram malformed")
                    == '@'
                {
                    count += 1;
                }
                already_checked.push((x_with_offset, y_with_offset));
            }

            Some(count)
        }
    }
}

fn file_string() -> Result<String, Box<dyn Error>> {
    let mut args = env::args();
    args.next(); // throw away program name

    let filename = args.next().ok_or("File name not provided!")?;
    let string = read_to_string(filename)?;

    Ok(string)
}

fn main() -> Result<(), Box<dyn Error>> {
    let diagram = Diagram::from_string(file_string()?);

    let mut count = 0u32;
    for x in 0..diagram.width() {
        for y in 0..diagram.height() {
            if let Some(surrounding_rolls) = diagram.get_surrounding_rolls(x, y)
                && surrounding_rolls < 4
            {
                count += 1;
            }
        }
    }

    println!("{count} rolls of paper can be accessed.");

    Ok(())
}

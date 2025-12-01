use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

struct Dial(u32);

impl Dial {
    pub fn new() -> Self {
        Dial(50)
    }

    fn turn_right(&mut self, amount: u32) {
        self.0 = (self.0 + amount) % 100;
    }

    fn turn_left(&mut self, amount: u32) {
        let mut result = self.0 as i32 - amount as i32;
        while result < 0 {
            result += 100
        }
        self.0 = result as u32;
    }

    pub fn turn(&mut self, instruction: DialInstruction) {
        match instruction.direction {
            TurnDirection::Left => self.turn_left(instruction.amount),
            TurnDirection::Right => self.turn_right(instruction.amount),
        };
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    pub fn passes_zero_amount(&self, instruction: &DialInstruction) -> u32 {
        match instruction.direction {
            TurnDirection::Right => (self.0 + instruction.amount) / 100,
            TurnDirection::Left => {
                let start = if self.is_zero() { 0 } else { 100 - self.0 };
                (start + instruction.amount) / 100
            } // view left rotation the same way as right rotation
        }
    }
}

enum TurnDirection {
    Left,
    Right,
}

struct DialInstruction {
    direction: TurnDirection,
    amount: u32,
}

impl DialInstruction {
    pub fn from_line(line: String) -> Result<Self, Box<dyn Error>> {
        let direction = match &line[..1] {
            "L" => TurnDirection::Left,
            "R" => TurnDirection::Right,
            dir => return Err(format!("Invalid direction `{dir}`!").into()),
        };
        let amount = u32::from_str(&line[1..])?;

        Ok(DialInstruction { direction, amount })
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
    let buf_reader = BufReader::new(file).lines();

    let mut dial = Dial::new();

    let mut password_part_one: u32 = 0;
    let mut password_part_two: u32 = 0;

    for line in buf_reader {
        let instruction = DialInstruction::from_line(line?)?;

        password_part_two += dial.passes_zero_amount(&instruction);

        dial.turn(instruction);

        if dial.is_zero() {
            password_part_one += 1;
        }
    }

    println!("Password (Part one) is: {password_part_one}");

    println!("Password (Part two) is: {password_part_two}");

    Ok(())
}

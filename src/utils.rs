use std::{io, fs::read_to_string};

const INPUT_FOLDER: &'static str = "./input";

pub enum Part {
    Example,
    Part1,
    Part2,
}

impl Part {
    fn extension(self) -> &'static str {
        match self {
            Part::Example => "example",
            Part::Part1 => "01",
            Part::Part2 => "02",
        }
    }
}

pub fn read_input(day: u8, part: Part) -> io::Result<String> {
    read_to_string(format!(
        "{}/day{:0>2}_{}.txt",
        INPUT_FOLDER,
        day,
        part.extension()
    ))
}

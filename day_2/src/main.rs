use std::fs;

fn main() {
    let data = load_data().unwrap();
    let points = parser_points(data).unwrap();

    println!("Total points: {:?}", points.iter().sum::<i32>());
}

fn load_data() -> Result<String, Error> {
    let data_path = format!("{}/data/rock_paper_scissor.txt", env!("CARGO_MANIFEST_DIR"));
    let file_contents = fs::read_to_string(data_path);

    match file_contents {
        Ok(c) => Ok(c),
        Err(e) => Err(Error::from(e)),
    }
}

fn parser_points(data: String) -> Result<Vec<i32>, Error> {
    let mut acc: Vec<i32> = Vec::new();

    for line in data.lines() {
        let values: Vec<char> = line.chars().filter(|c| !c.is_whitespace()).collect();

        let challenger = RockPaperScissor::from_challenger(values[0]);
        let condition = WinCondition::from_condition(values[1]);

        let points = condition.points(&challenger);

        acc.push(points);
    }

    Ok(acc)
}

#[derive(PartialEq, Copy, Clone)]
enum RockPaperScissor {
    Rock,
    Paper,
    Scissor,
}

impl RockPaperScissor {
    fn is_victory(&self, other: &Self) -> bool {
        match self {
            RockPaperScissor::Rock => match other {
                RockPaperScissor::Scissor => true,
                _ => false,
            },
            RockPaperScissor::Paper => match other {
                RockPaperScissor::Rock => true,
                _ => false,
            },
            RockPaperScissor::Scissor => match other {
                RockPaperScissor::Paper => true,
                _ => false,
            },
        }
    }

    fn points(&self, other: &Self) -> i32 {
        let mut points = match &self {
            RockPaperScissor::Rock => 1,
            RockPaperScissor::Paper => 2,
            RockPaperScissor::Scissor => 3,
        };

        if self.is_victory(other) {
            points += 6;
        } else if self == other {
            points += 3;
        }

        points
    }

    fn from_challenger(c: char) -> Self {
        match c {
            'A' => RockPaperScissor::Rock,
            'B' => RockPaperScissor::Paper,
            'C' => RockPaperScissor::Scissor,
            _ => panic!("Invalid character"),
        }
    }

    fn from_answer(c: char) -> Self {
        match c {
            'X' => RockPaperScissor::Rock,
            'Y' => RockPaperScissor::Paper,
            'Z' => RockPaperScissor::Scissor,
            _ => panic!("Invalid character"),
        }
    }
}

#[derive(PartialEq)]
enum WinCondition {
    Win,
    Loose,
    Draw,
}

impl WinCondition {
    fn points(&self, challenger: &RockPaperScissor) -> i32 {
        let answer = self.needed_answer(&challenger);

        answer.points(&challenger)
    }

    fn needed_answer(&self, challenger: &RockPaperScissor) -> RockPaperScissor {
        match self {
            WinCondition::Win => match challenger {
                RockPaperScissor::Rock => RockPaperScissor::Paper,
                RockPaperScissor::Paper => RockPaperScissor::Scissor,
                RockPaperScissor::Scissor => RockPaperScissor::Rock,
            },
            WinCondition::Loose => match challenger {
                RockPaperScissor::Rock => RockPaperScissor::Scissor,
                RockPaperScissor::Paper => RockPaperScissor::Rock,
                RockPaperScissor::Scissor => RockPaperScissor::Paper,
            },
            WinCondition::Draw => *challenger,
        }
    }

    fn from_condition(c: char) -> Self {
        match c {
            'X' => WinCondition::Loose,
            'Y' => WinCondition::Draw,
            'Z' => WinCondition::Win,
            _ => panic!("Invalid character"),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    pub message: String,
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error {
            message: error.to_string(),
        }
    }
}

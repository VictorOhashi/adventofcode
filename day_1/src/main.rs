use std::fs;

fn main() {
    let mut calories = load_calories_data().unwrap();
    calories.sort_by(|a, b| b.cmp(a));

    println!("Top most calories: {:?}", calories.first());

    let sum = calories.iter().take(3).sum::<i32>();
    println!("Sum of first three elements: {:?}", sum);
}

fn load_calories_data() -> Result<Vec<i32>, Error> {
    let data_path = format!("{}/data/calories_data.csv", env!("CARGO_MANIFEST_DIR"));
    let file_contents = fs::read_to_string(data_path);

    match file_contents {
        Ok(c) => return parse_calories_data(c),
        Err(e) => return Err(Error::from(e)),
    }
}

fn parse_calories_data(data: String) -> Result<Vec<i32>, Error> {
    let mut carriers: Vec<Vec<i32>> = vec![Vec::new()];

    for line in data.lines() {
        if line.is_empty() {
            carriers.push(Vec::new());
            continue;
        }

        let calories = line.parse::<i32>().unwrap();
        carriers.last_mut().unwrap().push(calories);
    }

    let summed_calories = carriers
        .iter()
        .map(|c| c.iter().sum())
        .collect::<Vec<i32>>();

    Ok(summed_calories)
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

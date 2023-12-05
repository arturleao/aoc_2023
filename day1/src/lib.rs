use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

struct Trebuchet {
    calibration_path: String,
    data: Option<String>,
    calibrated_data: Vec<u32>,
    digits: HashMap<String, u32>,
}

impl Trebuchet {
    fn new(calibration_path: String) -> Self {
        Self {
            calibration_path,
            data: None,
            calibrated_data: Vec::new(),
            digits: HashMap::from([
                ("one".to_string(), 1),
                ("two".to_string(), 2),
                ("three".to_string(), 3),
                ("four".to_string(), 4),
                ("five".to_string(), 5),
                ("six".to_string(), 6),
                ("seven".to_string(), 7),
                ("eight".to_string(), 8),
                ("nine".to_string(), 9),
            ]),
        }
    }

    fn parse_data(&mut self) -> io::Result<()> {
        let current_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let file_path = current_dir.join(&self.calibration_path);

        println!("Parsing file: {:?}", file_path);

        let mut file = File::open(file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        self.data = Some(content);

        Ok(())
    }

    fn format_line(&self, line: String) -> String {
        let mut new_line = line.clone();

        'outer: for (index, _ch) in new_line.char_indices() {
            for (word, number) in self.digits.iter() {
                let sub = &line[index..];

                if sub.starts_with(word) {
                    new_line.replace_range(index..index + 1, &number.to_string());
                    break 'outer;
                }
            }
        }

        'outer: for (index, _ch) in new_line.char_indices().rev() {
            for (word, number) in self.digits.iter() {
                let sub = &line[index..];

                if sub.starts_with(word) {
                    new_line = new_line.replace(word, &number.to_string());
                    break 'outer;
                }
            }
        }

        new_line
    }

    fn calibrate_value(&self, line: &str) -> String {
        let mut first = char::default();
        let mut last = char::default();

        for c in line.chars() {
            match c.to_digit(10) {
                Some(_parsed_number) => {
                    if first == Default::default() {
                        first = c;
                    }

                    last = c;
                }
                None => (),
            }
        }

        let res_string = format!("{}{}", first, last);

        res_string
    }

    fn get_calibrated_result(&self) {
        let sum: u32 = self.calibrated_data.iter().sum();

        println!("Calibrated result: {}", sum);
    }

    fn calibrate_data_per_line(&mut self) {
        match &self.data {
            Some(data) => {
                for line in data.lines() {
                    println!("Line: {}", line);
                    let formatted_line = self.format_line(line.to_owned());
                    println!("Formatted line: {}", formatted_line);
                    let calibrated_line = self.calibrate_value(&formatted_line);
                    println!("Calibrated line: {}", calibrated_line);

                    let val = calibrated_line.parse::<u32>().unwrap_or(0);
                    self.calibrated_data.push(val)
                }
            }
            None => {
                println!("No calibration data available");
            }
        }
    }

    fn run_calibration(&mut self) {
        if let Err(err) = self.parse_data() {
            eprintln!("Error parsing data: {}", err);
            return;
        } else {
            match &self.data {
                Some(_data) => println!("Parsed data."),
                None => println!("No calibration data available."),
            }
        }

        self.calibrate_data_per_line();

        self.get_calibrated_result();
    }
}

pub fn main() {
    let mut trebuchet = Trebuchet::new(r"data\input.txt".to_string());

    trebuchet.run_calibration();
}

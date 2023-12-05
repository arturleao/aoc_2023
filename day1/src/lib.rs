use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;
use std::ops::Add;
use std::path::Path;
use std::{env, string};

struct Trebuchet {
    calibration_path: String,
    data: Option<String>,
    calibrated_data: Vec<u32>,
    words: HashMap<String, u32>,
}

impl Trebuchet {
    fn new(calibration_path: String) -> Self {
        Self {
            calibration_path,
            data: None,
            calibrated_data: Vec::new(),
            words: HashMap::from([
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
        println!("Line: {}", line);
        let mut new_line = String::new();
        let mut slice_start = 0;

        for (index, ch) in line.char_indices() {
            let mut found = false;
            'outer: for (word, number) in self.words.iter() {                
                let sub = &line[slice_start..];
                //println!("Searching word {} in {}", word, sub);

                if sub.starts_with(word) {
                    new_line.push_str(&number.to_string());
                    slice_start += word.len();
                    found = true;
                    break 'outer;
                }
            }

            if slice_start <= index {
                if !found {
                    new_line.push(line.chars().nth(slice_start).unwrap());
                }
                slice_start += 1;                
            }                      
        }

        new_line.push_str("\n");
        //println!("Final line: {}", new_line);

        new_line
    }

    fn format_data(&mut self) {
        let mut new_data = String::new();

        match &self.data {
            Some(data) => {
                let new_line = data
                    .lines()
                    .map(|line| self.format_line(line.to_string()))
                    .collect();
                new_data = new_line;
            }
            None => {
                println!("No calibration data available");
            }
        }

        self.data = Some(new_data);
    }

    fn calibrate_data(&mut self) {
        match &self.data {
            Some(data) => {
                for line in data.lines() {
                    match self.calibrate_value(line) {
                        Ok(val) => self.calibrated_data.push(val),
                        Err(_) => (),
                    }
                }
            }
            None => {
                println!("No calibration data available");
            }
        }
    }

    fn calibrate_value(&self, line: &str) -> Result<u32, ParseIntError> {
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

        let result = match res_string.parse::<u32>() {
            Ok(val) => Ok(val),
            Err(err) => Err(err),
        };
        println!("Word result: {}", res_string);
        result
    }

    fn get_calibrated_result(&self) {
        let sum: u32 = self.calibrated_data.iter().sum();

        println!("Calibrated result: {}", sum);
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

        self.format_data();

        self.calibrate_data();

        self.get_calibrated_result();
    }
}

pub fn main() {
    let mut trebuchet = Trebuchet::new(r"data\test.txt".to_string());

    trebuchet.run_calibration();
}

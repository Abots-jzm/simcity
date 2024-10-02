use std::{
    fs,
    io::{self, BufRead, ErrorKind},
};

use crate::config::Config;

pub struct Map {
    grid: Vec<Vec<char>>,
}

impl Map {
    pub fn from_config(config: &mut Config) -> Self {
        let file = match fs::File::open(&config.region_layout_filename) {
            Ok(file) => file,
            Err(e) => {
                if let ErrorKind::NotFound = e.kind() {
                    println!(
                        "Error: couldn't find \"{}\". Please ensure the Region Layout file exists",
                        config.region_layout_filename
                    )
                } else {
                    println!("Error: An unenexpected error occured")
                }

                *config = Config::from_user_input();
                return Self::from_config(config);
            }
        };

        let reader = io::BufReader::new(file);
        let mut grid = vec![];

        for line in reader.lines() {
            let line = line.expect("Error. Unexpected error while reading line");
            let split = line.split(",");
            let mut row = vec![];

            for region in split {
                row.push(region.chars().next().unwrap());
            }

            grid.push(row);
        }

        return Self { grid };
    }

    pub fn print(&self) {
        for row in &self.grid {
            for region in row {
                print!("{}", region);
            }
            println!();
        }
    }
}

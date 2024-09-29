use std::fs::File;
use std::io::{stdin, BufRead, BufReader, ErrorKind};

pub struct Config {
    pub region_layout_filename: String,
    pub time_limit: u32,
    pub refresh_rate: u32,
    config_filename: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            config_filename: String::new(),
            region_layout_filename: String::new(),
            time_limit: 0,
            refresh_rate: 0,
        }
    }

    pub fn init(&mut self) {
        self.request_config_filename();
        self.read_config_file();
    }

    fn request_config_filename(&mut self) {
        println!("Please input a valid config file(.txt): ");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        self.config_filename = input.trim().to_string();

        while !self.config_filename.ends_with(".txt") {
            println!("Error: Invalid file format.\nPlease input a valid config file(.txt): ");
            input.clear();
            stdin().read_line(&mut input).unwrap();
            self.config_filename = input.trim().to_string();
        }
    }

    fn read_config_file(&mut self) {
        let file = match File::open(&self.config_filename.trim()) {
            Ok(file) => file,
            Err(ref error) if error.kind() == ErrorKind::NotFound => {
                println!("Error: File not found. Please input a valid config file(.txt): ");
                self.request_config_filename();
                return self.read_config_file();
            }
            Err(error) => {
                panic!("Error opening file: {:?}", error);
            }
        };
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            let mut split = line.split(":");
            let key = split.next().unwrap();
            let value = split.next().unwrap();

            match key {
                "Region Layout" => self.region_layout_filename = value.to_string(),
                "Time Limit" => self.time_limit = value.trim().parse().unwrap(),
                "Refresh Rate" => self.refresh_rate = value.trim().parse().unwrap(),
                _ => (),
            }
        }

        if self.region_layout_filename.is_empty() {
            println!("Error: Couldn't read 'Region Layout' from config file. Please ensure config file line is in the format 'Region Layout: <Region_Layout_File>.csv'");
            self.init();
        } else if self.time_limit == 0 {
            println!("Error: Couldn't read 'Time Limit' from config file. Please ensure config file line is in the format 'Time Limit: <Time_Limit>'");
            self.init();
        } else if self.refresh_rate == 0 {
            println!("Error: Couldn't read 'Refresh Rate' from config file. Please ensure config file line is in the format 'Refresh Rate: <Refresh_Rate>'");
            self.init();
        }
    }
}

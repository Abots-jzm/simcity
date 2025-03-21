use std::{error::Error, fs, io};

#[derive(Debug)]
pub struct Config {
    pub region_layout_filename: String,
    pub time_limit: u32,
    pub refresh_rate: u32,
}

impl Config {
    pub fn from_user_input() -> Self {
        let config_filename = Self::request_config_filename();

        let (region_layout_filename, time_limit, refresh_rate) =
            match Self::read_config_file(&config_filename) {
                Ok(data) => data,
                Err(e) => {
                    println!("Error: {}", e);
                    return Self::from_user_input();
                }
            };

        Config {
            region_layout_filename,
            time_limit,
            refresh_rate,
        }
    }

    pub fn reinitialize(&mut self) {
        let new_config = Self::from_user_input();
        self.region_layout_filename = new_config.region_layout_filename;
        self.time_limit = new_config.time_limit;
        self.refresh_rate = new_config.refresh_rate;
    }

    fn request_config_filename() -> String {
        println!("Please input a valid config file(.txt): ");
        let mut filename = String::new();
        io::stdin()
            .read_line(&mut filename)
            .expect("Failed to read line");

        while !Self::validate_file_extension(&filename, ".txt") {
            println!("Error. Invalid File Format.");
            println!("Please input a valid config file(.txt)");
            filename.clear();
            io::stdin()
                .read_line(&mut filename)
                .expect("Failed to read line");
        }

        filename.trim().to_string()
    }

    fn read_config_file(config_filename: &str) -> Result<(String, u32, u32), Box<dyn Error>> {
        let contents = fs::read_to_string(config_filename)
            .map_err(|_| format!("Couldn't open \"{}\"", config_filename))?;

        let mut result: (String, u32, u32) = (String::new(), 0, 0);
        contents
            .lines()
            .map(|line| line.split_once(':').unwrap())
            .for_each(|(key, value)| {
                if key == "Region Layout" {
                    result.0 = value.trim().to_string();
                } else if key == "Time Limit" {
                    result.1 = value.trim().parse().unwrap();
                } else if key == "Refresh Rate" {
                    result.2 = value.trim().parse().unwrap();
                }
            });

        if result.0.is_empty() || !Self::validate_file_extension(&result.0, ".csv") {
            return Err(format!("Missing 'Region Layout' in \"{}\"", config_filename).into());
        }
        if result.1 == 0 {
            return Err(
                format!("Missing or invalid 'Time Limit' in \"{}\"", config_filename).into(),
            );
        }
        if result.2 == 0 {
            return Err(format!(
                "Missing or invalid 'Refresh Rate' in \"{}\"",
                config_filename
            )
            .into());
        }

        Ok(result)
    }

    fn validate_file_extension(filename: &str, extension: &str) -> bool {
        if filename.trim().len() < extension.len() {
            return false;
        }

        filename.trim().ends_with(extension)
    }
}

use std::{
    fs,
    io::{self, BufRead, Write},
};

pub struct Config {
    pub region_layout_filename: String,
    pub time_limit: u32,
    pub refresh_rate: u32,
    config_filename: String,
}

impl Config {
    pub fn from_user_input() -> Self {
        let config_filename = Self::request_config_filename();
        let config = Self::read_config_file(&config_filename);

        match config {
            Ok(config) => config,
            Err(message) => {
                println!("{}", message);
                return Self::from_user_input();
            }
        }
    }

    fn request_config_filename() -> String {
        let mut input = String::new();
        print!("Please enter a valid config file(.txt): ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Error: Failed to read user input");
        input = input.trim().to_string();

        while !input.ends_with(".txt") {
            println!("Error: Invalid file format");
            input.clear();
            print!("Please enter a valid config file(.txt): ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut input)
                .expect("Error: Failed to read user input");
            input = input.trim().to_string();
        }

        return input;
    }

    fn read_config_file(config_filename: &str) -> Result<Self, String> {
        let file = fs::File::open(config_filename)
            .map_err(|_| format!("Error: Couldn't find \"{}\"", config_filename))?;

        let reader = io::BufReader::new(file);
        let mut region_layout_filename = String::new();
        let mut time_limit = 0;
        let mut refresh_rate = 0;

        for line in reader.lines() {
            let line = line.map_err(|_| "Error: Unexpected error while reading line")?;
            let mut split = line.split(":");
            let key = split.next().unwrap();
            let value = split.next().unwrap();

            match key {
                "Region Layout" => region_layout_filename = value.to_string(),
                "Time Limit" => time_limit = value.trim().parse().unwrap(),
                "Refresh Rate" => refresh_rate = value.trim().parse().unwrap(),
                _ => (),
            }
        }

        if region_layout_filename.is_empty() {
            return Err("Error: Couldn't read 'Region Layout' from config file. Please ensure config file line is in the format 'Region Layout: <Region_Layout_File>.csv'".to_string());
        } else if time_limit == 0 {
            return Err("Error: Couldn't read 'Time Limit' from config file. Please ensure config file line is in the format 'Time Limit: <Time_Limit>'".to_string());
        } else if refresh_rate == 0 {
            return Err("Error: Couldn't read 'Refresh Rate' from config file. Please ensure config file line is in the format 'Refresh Rate: <Refresh_Rate>'".to_string());
        }

        return Ok(Self {
            config_filename: String::from(config_filename),
            refresh_rate,
            time_limit,
            region_layout_filename,
        });
    }
}

// impl Config {
//     fn new(config_filename: &str) -> Config {
//         let file = match File::open(config_filename) {
//             Ok(file) => file,
//             Err(ref error) if error.kind() == ErrorKind::NotFound => {
//                 println!("Error: File not found. Please input a valid config file(.txt): ");
//                 let config_filename = request_config_filename();
//                 return Config::new(&config_filename);
//             }
//             Err(error) => {
//                 panic!("Error opening file: {:?}", error);
//             }
//         };

//         let reader = BufReader::new(file);
//         let mut region_layout_filename = String::new();
//         let mut time_limit = 0;
//         let mut refresh_rate = 0;

//         for line in reader.lines() {
//             let line = line.unwrap();
//             let mut split = line.split(":");
//             let key = split.next().unwrap();
//             let value = split.next().unwrap();

//             match key {
//                 "Region Layout" => region_layout_filename = value.to_string(),
//                 "Time Limit" => time_limit = value.trim().parse().unwrap(),
//                 "Refresh Rate" => refresh_rate = value.trim().parse().unwrap(),
//                 _ => (),
//             }
//         }

//         if region_layout_filename.is_empty() {
//             println!("Error: Couldn't read 'Region Layout' from config file. Please ensure config file line is in the format 'Region Layout: <Region_Layout_File>.csv'");
//             return Config::new(&request_config_filename());
//         } else if time_limit == 0 {
//             println!("Error: Couldn't read 'Time Limit' from config file. Please ensure config file line is in the format 'Time Limit: <Time_Limit>'");
//             return Config::new(&request_config_filename());
//         } else if refresh_rate == 0 {
//             println!("Error: Couldn't read 'Refresh Rate' from config file. Please ensure config file line is in the format 'Refresh Rate: <Refresh_Rate>'");
//             return Config::new(&request_config_filename());
//         }

//         return Config {
//             region_layout_filename,
//             time_limit,
//             refresh_rate,
//             config_filename: config_filename.to_string(),
//         };
//     }
// }

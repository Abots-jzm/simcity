use std::{
    fs::File,
    io::{BufRead, BufReader, ErrorKind},
};

use crate::{
    config::Config,
    map_cell::{CellType, MapCell},
};

pub struct Map {
    grid: Vec<Vec<MapCell>>,
}

impl Map {
    pub fn new(config: &Config) -> Map {
        let layout_file = match File::open(config.region_layout_filename.clone()) {
            Ok(file) => file,
            Err(ref error) if error.kind() == ErrorKind::NotFound => {
                panic!("Error: File not found. Please input a valid region layout file(.csv)");
            }
            Err(error) => {
                panic!("Error opening file: {:?}", error);
            }
        };

        let reader = BufReader::new(layout_file);
        let mut grid = Vec::new();

        for (i, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            let mut row = Vec::new();

            for (j, cell) in line.chars().enumerate() {
                let cell_type = match cell {
                    'R' => CellType::Residential(cell),
                    'C' => CellType::Commercial(cell),
                    'I' => CellType::Industrial(cell),
                    _ => CellType::Other(cell),
                };

                let map_cell = MapCell::new((i as u32, j as u32), cell_type);

                row.push(map_cell);
            }

            grid.push(row);
        }

        return Map { grid };
    }
}

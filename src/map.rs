use std::{
    cell::RefCell,
    fs,
    io::{self, BufRead, ErrorKind},
    rc::Rc,
};

use crate::{
    config::Config,
    map_cell::{CellType, MapCell},
};

pub struct Map {
    grid: Vec<Vec<Rc<RefCell<MapCell>>>>,
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

        for (y, line) in reader.lines().enumerate() {
            let line = line.expect("Error. Unexpected error while reading line");
            let split = line.split(",");
            let mut row = vec![];
            for (x, region) in split.enumerate() {
                row.push(Rc::new(RefCell::new(MapCell::new(
                    x,
                    y,
                    region.chars().next().unwrap(),
                ))));
            }

            grid.push(row);
        }

        let new_map = Self { grid };
        new_map.trackAdjacentCells();

        return new_map;
    }

    fn trackAdjacentCells(&self) {
        for row in &self.grid {
            for cell in row {
                let (x, y) = cell.borrow().position;
                // let mut adjacentCells = vec![];
                if let CellType::Other(_) = cell.borrow().type_ {
                    continue;
                }

                if x > 0 {
                    cell.borrow_mut()
                        .adjacentCells
                        .push(Rc::clone(&self.grid[y][x - 1]));

                    if y > 0 {
                        cell.borrow_mut()
                            .adjacentCells
                            .push(Rc::clone(&self.grid[y - 1][x - 1]));
                    }
                    if y < self.grid.len() - 1 {
                        cell.borrow_mut()
                            .adjacentCells
                            .push(Rc::clone(&self.grid[y + 1][x - 1]));
                    }
                }
                if x < self.grid[y].len() - 1 {
                    cell.borrow_mut()
                        .adjacentCells
                        .push(Rc::clone(&self.grid[y][x + 1]));

                    if y > 0 {
                        cell.borrow_mut()
                            .adjacentCells
                            .push(Rc::clone(&self.grid[y - 1][x + 1]));
                    }
                    if y < self.grid.len() - 1 {
                        cell.borrow_mut()
                            .adjacentCells
                            .push(Rc::clone(&self.grid[y + 1][x + 1]));
                    }
                }
                if y > 0 {
                    cell.borrow_mut()
                        .adjacentCells
                        .push(Rc::clone(&self.grid[y - 1][x]));
                }
                if y < self.grid.len() - 1 {
                    cell.borrow_mut()
                        .adjacentCells
                        .push(Rc::clone(&self.grid[y + 1][x]));
                }
            }
        }
    }

    pub fn print(&self) {
        for row in &self.grid {
            for region in row {
                print!("{}", region.borrow());
            }
            println!();
        }
    }
}

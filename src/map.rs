use std::{
    cell::RefCell,
    collections::BinaryHeap,
    fs,
    io::{self, BufRead, ErrorKind},
    rc::Rc,
};

use crate::{
    config::Config,
    map_cell::{CellType, MapCell},
};

pub struct Map {
    pub current: Vec<Vec<Rc<RefCell<MapCell>>>>,
    pub previous: Option<Vec<Vec<Rc<RefCell<MapCell>>>>>,
}

enum Grid {
    Current,
    Previous,
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
        let mut current = vec![];

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

            current.push(row);
        }

        let new_map = Self {
            current,
            previous: None,
        };
        new_map.track_adjacent_cells(Grid::Current);

        return new_map;
    }

    pub fn step(&mut self) {
        self.previous = Some(self.current.clone());
        self.track_adjacent_cells(Grid::Previous);

        // Add every element in current grid to max heap priority queue
        // let mut queue: BinaryHeap<_> = BinaryHeap::new();
        // for row in &self.current {
        //     for cell in row {
        //         queue.push(Rc::clone(cell));
        //     }
        // }
    }

    fn track_adjacent_cells(&self, grid: Grid) {
        let grid = match grid {
            Grid::Current => &self.current,
            Grid::Previous => self.previous.as_ref().unwrap(),
        };

        for row in grid {
            for cell in row {
                let (x, y) = cell.borrow().position;
                if let CellType::Other(_) = cell.borrow().type_ {
                    continue;
                }

                if x > 0 {
                    cell.borrow_mut()
                        .adjacent_cells
                        .push(Rc::clone(&grid[y][x - 1]));

                    if y > 0 {
                        cell.borrow_mut()
                            .adjacent_cells
                            .push(Rc::clone(&grid[y - 1][x - 1]));
                    }
                    if y < grid.len() - 1 {
                        cell.borrow_mut()
                            .adjacent_cells
                            .push(Rc::clone(&grid[y + 1][x - 1]));
                    }
                }
                if x < grid[y].len() - 1 {
                    cell.borrow_mut()
                        .adjacent_cells
                        .push(Rc::clone(&grid[y][x + 1]));

                    if y > 0 {
                        cell.borrow_mut()
                            .adjacent_cells
                            .push(Rc::clone(&grid[y - 1][x + 1]));
                    }
                    if y < grid.len() - 1 {
                        cell.borrow_mut()
                            .adjacent_cells
                            .push(Rc::clone(&grid[y + 1][x + 1]));
                    }
                }
                if y > 0 {
                    cell.borrow_mut()
                        .adjacent_cells
                        .push(Rc::clone(&grid[y - 1][x]));
                }
                if y < grid.len() - 1 {
                    cell.borrow_mut()
                        .adjacent_cells
                        .push(Rc::clone(&grid[y + 1][x]));
                }
            }
        }
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let width = self.current.first().map_or(0, |row| row.len());
        let border = "+".to_string() + &"-".repeat(width * 5) + "+\n";

        write!(f, "{}", border)?;

        for row in &self.current {
            write!(f, "|")?;
            for region in row {
                write!(f, "{}", region.borrow())?;
            }
            write!(f, "|\n")?;
        }

        write!(f, "{}", border)?;

        Ok(())
    }
}

impl PartialEq for Map {
    fn eq(&self, other: &Self) -> bool {
        for (row1, row2) in self.current.iter().zip(other.current.iter()) {
            for (cell1, cell2) in row1.iter().zip(row2.iter()) {
                if *cell1.borrow() != *cell2.borrow() {
                    return false;
                }
            }
        }

        true
    }
}

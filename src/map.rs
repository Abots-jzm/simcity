use std::{
    cell::RefCell,
    collections::BinaryHeap,
    fs,
    io::{self, BufRead, ErrorKind},
    rc::Rc,
};

use crate::grid::GridHelper;
use crate::{config::Config, grid::Grid, map_cell::MapCell};

pub struct Map {
    pub current: Grid,
    pub previous: Option<Grid>,
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
        let mut current: Grid = vec![];

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
        current.track_adjacent_cells();

        Self {
            current,
            previous: None,
        }
    }

    pub fn step(&mut self) {
        self.previous = Some(self.current.deep_clone());
        self.previous.as_ref().unwrap().track_adjacent_cells();

        let mut queue: BinaryHeap<_> = BinaryHeap::new();
        for row in &self.current {
            for cell in row {
                queue.push(Rc::clone(cell));
            }
        }

        let mut available_workers = self.previous.as_ref().unwrap().get_available_workers();
        let mut available_goods = self.previous.as_ref().unwrap().get_available_goods();

        while let Some(cell) = queue.pop() {
            let (x, y) = cell.borrow().position;

            let prev_cell = self.previous.as_ref().unwrap()[y][x].borrow();

            cell.borrow_mut()
                .step(&prev_cell, &mut available_workers, &mut available_goods);
        }
    }

    pub fn spread_pollution(&mut self) {
        let mut queue: BinaryHeap<_> = BinaryHeap::new();
        for row in &self.current {
            for cell in row {
                queue.push(Rc::clone(cell));
            }
        }

        while let Some(cell) = queue.pop() {
            if cell.borrow().pollution < 2 {
                continue;
            }

            for adjacent_cell in &cell.borrow().adjacent_cells {
                adjacent_cell.borrow_mut().pollution = cell.borrow().pollution - 1;
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

use crate::{
    config::Config,
    map_cell::{CellType, MapCell},
};
use std::{cell::RefCell, fmt, fs, rc::Rc};

pub type MapGrid = Vec<Vec<Rc<RefCell<MapCell>>>>;

pub struct Map {
    pub current: MapGrid,
    pub previous: Option<MapGrid>,
}

impl Map {
    pub fn from_config(config: &mut Config) -> Self {
        let contents = match fs::read_to_string(&config.region_layout_filename) {
            Ok(content) => content,
            Err(_) => {
                println!(
                    "Error: Couldn't find \"{}\". Please make sure the Region Layout file exists",
                    config.region_layout_filename
                );
                config.reinitialize();
                return Self::from_config(config);
            }
        };

        let grid: MapGrid = contents
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.split(',')
                    .enumerate()
                    .map(|(x, cell_char)| {
                        Rc::new(RefCell::new(MapCell {
                            position: (x as u32, y as u32),
                            cell_type: match cell_char {
                                "R" => CellType::Residential('R'),
                                "C" => CellType::Commercial('C'),
                                "I" => CellType::Industrial('I'),
                                _ => CellType::Other(cell_char.chars().next().unwrap()),
                            },
                            pollution: 0,
                            population: 0,
                            is_powerline_adjacent: false,
                            neighbors: vec![],
                        }))
                    })
                    .collect()
            })
            .collect();

        let map = Map {
            current: grid,
            previous: None,
        };
        Self::track_adjacency(&map.current);

        map
    }

    pub fn spread_pollution(&mut self) {
        // Collect all cells into a vector
        let mut cells: Vec<Rc<RefCell<MapCell>>> = self
            .current
            .iter()
            .flatten()
            .map(|cell| Rc::clone(cell))
            .collect();

        // Sort cells by pollution level in descending order
        cells.sort_by(|a, b| b.borrow().pollution.cmp(&a.borrow().pollution));

        // Process cells with pollution >= 2
        for cell in cells {
            let pollution_level = cell.borrow().pollution;
            if pollution_level < 2 {
                continue;
            }

            // Get weak references to neighbors and process them
            let neighbors = cell.borrow().neighbors.clone();
            for neighbor_weak in &neighbors {
                if let Some(neighbor) = neighbor_weak.upgrade() {
                    let neighbor_pollution = neighbor.borrow().pollution;
                    if neighbor_pollution < pollution_level - 1 {
                        neighbor.borrow_mut().pollution = pollution_level - 1;
                    }
                }
            }
        }
    }

    pub fn step(&mut self) {
        let mut cells: Vec<Rc<RefCell<MapCell>>> = self
            .current
            .iter()
            .flatten()
            .map(|cell| Rc::clone(cell))
            .collect();

        cells.sort_by(|a, b| a.borrow().cmp(&b.borrow()));

        let previous = &self.previous.as_ref().unwrap();

        let available_workers = Self::get_available_workers(previous);
        let available_goods = Self::get_available_goods(previous);

        let mut remaining_workers = available_workers;
        let mut remaining_goods = available_goods;

        for cell in cells {
            let (x, y) = cell.borrow().position;
            let remainders = cell.borrow_mut().grow(
                &previous[y as usize][x as usize].borrow(),
                remaining_workers,
                remaining_goods,
            );

            remaining_workers = remainders.0;
            remaining_goods = remainders.1;
        }
    }

    pub fn update_previous(&mut self) {
        let grid = &self.current;
        // Create deep clone with new MapCell instances
        let deep_clone: MapGrid = grid
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| {
                        let cell_ref = cell.borrow();
                        Rc::new(RefCell::new(MapCell {
                            position: cell_ref.position,
                            cell_type: cell_ref.cell_type.clone(),
                            pollution: cell_ref.pollution,
                            population: cell_ref.population,
                            is_powerline_adjacent: cell_ref.is_powerline_adjacent,
                            neighbors: vec![], // We'll rebuild neighbor references below
                        }))
                    })
                    .collect()
            })
            .collect();

        self.previous = Some(deep_clone);

        // Rebuild neighbor references in the cloned map
        if let Some(prev_grid) = &self.previous {
            Self::track_adjacency(prev_grid);
        }
    }

    pub fn track_adjacency(grid: &MapGrid) {
        let height = grid.len();

        for y in 0..height {
            let width = grid[y].len();
            for x in 0..width {
                let cell = &grid[y][x];
                let mut neighbors = Vec::new();

                // Check all 8 adjacent cells
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        } // Skip self

                        let nx = x as isize + dx;
                        let ny = y as isize + dy;

                        // Check bounds
                        if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                            // Add weak reference to neighbor
                            let neighbor = &grid[ny as usize][nx as usize];
                            let neighbor_cell = neighbor.borrow();
                            let is_powerline = neighbor_cell.cell_type == CellType::Other('T')
                                || neighbor_cell.cell_type == CellType::Other('#');

                            if is_powerline && !cell.borrow().is_powerline_adjacent {
                                cell.borrow_mut().is_powerline_adjacent = true;
                            }
                            neighbors.push(Rc::downgrade(neighbor));
                        }
                    }
                }

                cell.borrow_mut().neighbors = neighbors;
            }
        }
    }

    pub fn get_population(grid: &MapGrid, cell_type: Option<&CellType>) -> i32 {
        let mut total_population = 0;

        for row in grid {
            for cell_rc in row {
                let cell = cell_rc.borrow();
                if let Some(cell_type) = &cell_type {
                    if &cell.cell_type != *cell_type {
                        continue;
                    }
                }
                total_population += cell.population as i32;
            }
        }

        total_population
    }

    pub fn get_available_workers(grid: &MapGrid) -> i32 {
        let mut total_workers = 0;
        let mut taken_workers = 0;

        for row in grid {
            for cell_rc in row {
                let cell = cell_rc.borrow();
                match &cell.cell_type {
                    CellType::Residential(_) => {
                        total_workers += cell.population as i32;
                    }
                    CellType::Industrial(_) => {
                        taken_workers += (cell.population * 2) as i32;
                    }
                    CellType::Commercial(_) => {
                        taken_workers += cell.population as i32;
                    }
                    _ => {}
                }
            }
        }

        total_workers - taken_workers
    }

    pub fn get_available_goods(grid: &MapGrid) -> i32 {
        let mut total_goods = 0;
        let mut sold_goods = 0;

        for row in grid {
            for cell_rc in row {
                let cell = cell_rc.borrow();
                match &cell.cell_type {
                    CellType::Industrial(_) => {
                        total_goods += cell.population as i32;
                    }
                    CellType::Commercial(_) => {
                        sold_goods += cell.population as i32;
                    }
                    _ => {}
                }
            }
        }

        total_goods - sold_goods
    }

    pub fn total_pollution(grid: &MapGrid) -> i32 {
        let mut total_pollution = 0;

        for row in grid {
            for cell_rc in row {
                let cell = cell_rc.borrow();
                total_pollution += cell.pollution as i32;
            }
        }

        total_pollution
    }

    pub fn print_pollution(grid: &MapGrid) {
        // Print top border
        let width = grid[0].len();
        println!();
        for _ in 0..width {
            print!("----");
        }
        println!("--");

        // Print each row with pollution values
        for row in grid {
            print!("|"); // Left border
            for cell in row {
                let poll = cell.borrow().pollution;
                // Format pollution as fixed width of 3 characters
                print!(" {:<3}", poll);
            }
            println!("|"); // Right border
        }

        // Print bottom border
        for _ in 0..width {
            print!("----");
        }
        println!("--");
        println!();
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let grid = &self.current;
        let width = grid[0].len();

        // Print top border
        for _ in 0..width {
            write!(f, "----")?;
        }
        writeln!(f, "--")?;

        // Print each row with cells
        for row in grid {
            write!(f, "|")?; // Left border
            for cell in row {
                // Use left-padded fixed width of 4 characters
                let cell_str = format!("{}", cell.borrow());
                write!(f, " {:<3}", cell_str)?;
            }
            writeln!(f, "|")?; // Right border without extra space
        }

        // Print bottom border
        for _ in 0..width {
            write!(f, "----")?;
        }
        write!(f, "--")
    }
}

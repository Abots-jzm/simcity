use std::{cell::RefCell, cmp::Ordering, fmt, rc::Rc};

use crate::cell_type::CellType;

#[derive(Clone)]
pub struct MapCell {
    pub position: (usize, usize),
    pub type_: CellType,
    pub population: u32,
    pub pollution: u32,
    pub is_powerline_adjacent: bool,
    pub adjacent_cells: Vec<Rc<RefCell<MapCell>>>,
}

impl MapCell {
    pub fn new(x: usize, y: usize, symbol: char) -> Self {
        Self {
            position: (x, y),
            type_: match symbol {
                'R' => CellType::Residential(symbol),
                'I' => CellType::Industrial(symbol),
                'C' => CellType::Commercial(symbol),
                _ => CellType::Other(symbol),
            },
            population: 0,
            pollution: 0,
            is_powerline_adjacent: false,
            adjacent_cells: vec![],
        }
    }

    pub fn add_adjacent_cell(&mut self, cell: Rc<RefCell<MapCell>>) {
        if cell.borrow().type_ == CellType::Other('T')
            || cell.borrow().type_ == CellType::Other('#')
        {
            self.is_powerline_adjacent = true;
        }

        self.adjacent_cells.push(cell);
    }

    pub fn step(&mut self, previous: &MapCell, workers: &mut u32, goods: &mut u32) {
        match &self.type_ {
            CellType::Residential(_) => self.residential_grow(previous),
            CellType::Industrial(_) => self.industrial_grow(previous, workers),
            CellType::Commercial(_) => self.commercial_grow(previous, workers, goods),
            CellType::Other(_) => {}
        }
    }

    fn residential_grow(&mut self, previous: &MapCell) {
        if previous.population == 0 {
            if previous.is_powerline_adjacent || previous.count_adjacent_population() >= 1 {
                self.population = 1;
            }
        } else {
            let (target_adjacents, target_population) = match previous.population {
                1 => (2, 1),
                2 => (4, 2),
                3 => (6, 3),
                4 => (8, 4),
                _ => return,
            };

            let mut target_adjacents = target_adjacents;

            for cell in &previous.adjacent_cells {
                if cell.borrow().population >= target_population {
                    target_adjacents -= 1;
                }

                if target_adjacents == 0 {
                    self.population = previous.population + 1;
                    break;
                }
            }
        }
    }

    fn industrial_grow(&mut self, previous: &MapCell, workers: &mut u32) {
        if *workers < 2 {
            return;
        }

        if previous.population == 0 && previous.is_powerline_adjacent {
            self.population = 1;
            self.pollution = 1;
            *workers -= 2;
        } else {
            let (target_adjacents, target_population) = match previous.population {
                0 => (1, 1),
                1 => (2, 1),
                2 => (4, 2),
                _ => return,
            };

            let mut target_adjacents = target_adjacents;

            for cell in &previous.adjacent_cells {
                if cell.borrow().population >= target_population {
                    target_adjacents -= 1;
                }

                if target_adjacents == 0 {
                    self.population = previous.population + 1;
                    self.pollution = previous.pollution + 1;
                    *workers -= 2;
                    break;
                }
            }
        }
    }

    fn commercial_grow(&mut self, previous: &MapCell, workers: &mut u32, goods: &mut u32) {
        if *workers < 1 || *goods < 1 {
            return;
        }

        if previous.population == 0 && previous.is_powerline_adjacent {
            self.population = 1;
            *workers -= 1;
            *goods -= 1;
        } else {
            let (target_adjacents, target_population) = match previous.population {
                0 => (1, 1),
                1 => (2, 1),
                _ => return,
            };

            let mut target_adjacents = target_adjacents;

            for cell in &previous.adjacent_cells {
                if cell.borrow().population >= target_population {
                    target_adjacents -= 1;
                }

                if target_adjacents == 0 {
                    self.population = previous.population + 1;
                    *workers -= 1;
                    *goods -= 1;
                    break;
                }
            }
        }
    }

    pub fn count_adjacent_population(&self) -> u32 {
        self.adjacent_cells
            .iter()
            .map(|cell| cell.borrow().population)
            .sum()
    }
}

impl fmt::Display for MapCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (self.population, self.pollution) {
            (0, 0) => write!(f, "  {}  ", self.type_),
            (0, _) => write!(f, "  {}  ", self.pollution),
            (_, 0) => write!(f, "  {}  ", self.population),
            (_, _) => write!(f, "  {}  ", self.population),
        }
    }
}

impl PartialEq for MapCell {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
            && self.type_ == other.type_
            && self.population == other.population
            && self.pollution == other.pollution
            && self.is_powerline_adjacent == other.is_powerline_adjacent
    }
}

impl Eq for MapCell {}

impl PartialOrd for MapCell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MapCell {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.pollution != other.pollution {
            return self.pollution.cmp(&other.pollution);
        }

        match (&self.type_, &other.type_) {
            (CellType::Commercial(_), CellType::Commercial(_)) => {}
            (CellType::Commercial(_), _) => return Ordering::Greater,
            (_, CellType::Commercial(_)) => return Ordering::Less,
            (CellType::Industrial(_), CellType::Industrial(_)) => {}
            (CellType::Industrial(_), _) => return Ordering::Greater,
            (_, CellType::Industrial(_)) => return Ordering::Less,
            (CellType::Residential(_), CellType::Residential(_)) => {}
            (CellType::Residential(_), _) => return Ordering::Greater,
            (_, CellType::Residential(_)) => return Ordering::Less,
            (CellType::Other(_), CellType::Other(_)) => {}
        }

        self.population
            .cmp(&other.population)
            .then_with(|| {
                self.count_adjacent_population()
                    .cmp(&other.count_adjacent_population())
            })
            .then_with(|| other.position.1.cmp(&self.position.1))
            .then_with(|| self.position.0.cmp(&other.position.0))
    }
}

use std::{cell::RefCell, cmp::Ordering, fmt, rc::Rc};

use crate::cell_type::CellType;

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

    pub fn step(&mut self) {
        match &self.type_ {
            CellType::Residential(_) => {
                self.population += 1;
            }
            CellType::Industrial(_) => {
                self.pollution += 1;
            }
            CellType::Commercial(_) => {
                self.population -= 1;
            }
            CellType::Other(_) => {}
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
        write!(f, "  {}  ", self.type_)
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

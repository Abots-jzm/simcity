use std::{cell::RefCell, fmt, rc::Rc};

pub struct MapCell {
    pub position: (usize, usize),
    pub type_: CellType,
    pub population: u32,
    pub pollution: u32,
    pub is_powerline_adjacent: bool,
    pub adjacent_cells: Vec<Rc<RefCell<MapCell>>>,
}

pub enum CellType {
    Residential(char),
    Industrial(char),
    Commercial(char),
    Other(char),
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
}

impl fmt::Display for MapCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "  {}  ", self.type_)
    }
}

impl fmt::Display for CellType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CellType::Residential(symbol) => write!(f, "{}", symbol),
            CellType::Industrial(symbol) => write!(f, "{}", symbol),
            CellType::Commercial(symbol) => write!(f, "{}", symbol),
            CellType::Other(symbol) => write!(f, "{}", symbol),
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

impl PartialEq for CellType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CellType::Residential(_), CellType::Residential(_)) => true,
            (CellType::Industrial(_), CellType::Industrial(_)) => true,
            (CellType::Commercial(_), CellType::Commercial(_)) => true,
            (CellType::Other(_), CellType::Other(_)) => true,
            _ => false,
        }
    }
}

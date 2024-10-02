use std::{cell::RefCell, fmt, rc::Rc};

pub struct MapCell {
    pub position: (usize, usize),
    pub type_: CellType,
    population: u32,
    pollution: u32,
    isPowerlineAdjacent: bool,
    pub adjacentCells: Vec<Rc<RefCell<MapCell>>>,
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
            isPowerlineAdjacent: false,
            adjacentCells: vec![],
        }
    }
}

impl fmt::Display for MapCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.type_)
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

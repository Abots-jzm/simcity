use std::fmt;

pub enum CellType {
    Residential(char),
    Industrial(char),
    Commercial(char),
    Other(char),
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

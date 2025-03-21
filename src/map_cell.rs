use std::{cell::RefCell, rc::Weak};

#[derive(Debug)]
pub enum CellType {
    Residential(char),
    Industrial(char),
    Commercial(char),
    Other(char),
}

#[derive(Debug)]
pub struct MapCell {
    pub position: (u32, u32),
    pub cell_type: CellType,
    pub population: u32,
    pub pollution: u32,
    pub is_powerline_adjacent: bool,
    pub neighbors: Vec<Weak<RefCell<MapCell>>>,
}

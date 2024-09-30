use crate::map::Map;

pub enum CellType {
    Residential(char),
    Commercial(char),
    Industrial(char),
    Other(char),
}

pub struct MapCell {
    position: (u32, u32),
    cell_type: CellType,
    population: u32,
    pollution: u32,
    // is_powerline_adjacent: bool,
    // adjacent_cells: Vec<(u32, u32)>,
}

impl MapCell {
    pub fn new(position: (u32, u32), cell_type: CellType) -> MapCell {
        MapCell {
            position,
            cell_type,
            population: 0,
            pollution: 0,
        }
    }
}

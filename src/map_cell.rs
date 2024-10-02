pub struct CellData {
    posiiton: (u32, u32),
    symbol: char,
    population: u32,
    pollution: u32,
}

pub enum MapCell {
    Residential(CellData),
    Industrial(CellData),
    Commercial(CellData),
    Other(CellData),
}

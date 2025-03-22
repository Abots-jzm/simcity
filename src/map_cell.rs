use std::{
    cell::RefCell,
    cmp::Ordering,
    fmt::{Display, Formatter, Result},
    rc::Weak,
};

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl MapCell {
    pub fn grow(&mut self, previous: &MapCell, workers: i32, goods: i32) -> (i32, i32) {
        match self.cell_type {
            CellType::Residential(_) => Self::residential_grow(self, previous, workers, goods),
            CellType::Commercial(_) => Self::commercial_grow(self, previous, workers, goods),
            CellType::Industrial(_) => Self::industrial_grow(self, previous, workers, goods),
            CellType::Other(_) => (workers, goods),
        }
    }

    fn residential_grow(
        cell: &mut MapCell,
        previous: &MapCell,
        workers: i32,
        goods: i32,
    ) -> (i32, i32) {
        if previous.population == 0 {
            if previous.is_powerline_adjacent || previous.count_adjacent_population() >= 1 {
                cell.population = 1;
            }
        } else {
            let target_adjacents;

            match previous.population {
                1 => {
                    target_adjacents = 2;
                }
                2 => {
                    target_adjacents = 4;
                }
                3 => {
                    target_adjacents = 6;
                }
                4 => {
                    target_adjacents = 8;
                }
                _ => return (workers, goods),
            }

            let mut remaining_adjacents = target_adjacents;

            for cell_weak in &previous.neighbors {
                if let Some(neighbor) = cell_weak.upgrade() {
                    let neighbor = neighbor.borrow();

                    if neighbor.population >= previous.population {
                        remaining_adjacents -= 1;
                    }

                    if remaining_adjacents == 0 {
                        cell.population = previous.population + 1;
                        break;
                    }
                }
            }
        }

        (workers, goods)
    }

    fn commercial_grow(
        cell: &mut MapCell,
        previous: &MapCell,
        workers: i32,
        goods: i32,
    ) -> (i32, i32) {
        // Early return if not enough resources
        if workers < 1 || goods < 1 {
            return (workers, goods);
        }

        let mut workers_used = workers;
        let mut goods_used = goods;

        if previous.population == 0 && previous.is_powerline_adjacent {
            cell.population = 1;
            workers_used -= 1;
            goods_used -= 1;
        } else {
            let mut target_adjacents;
            let target_population;

            match previous.population {
                0 => {
                    target_adjacents = 1;
                    target_population = 1;
                }
                1 => {
                    target_adjacents = 2;
                    target_population = 1;
                }
                _ => return (workers, goods),
            }

            for cell_weak in &previous.neighbors {
                if let Some(neighbor) = cell_weak.upgrade() {
                    let neighbor = neighbor.borrow();

                    if neighbor.population >= target_population {
                        target_adjacents -= 1;
                    }

                    if target_adjacents == 0 {
                        cell.population = previous.population + 1;
                        workers_used -= 1;
                        goods_used -= 1;
                        break;
                    }
                }
            }
        }

        (workers_used, goods_used)
    }

    fn industrial_grow(
        cell: &mut MapCell,
        previous: &MapCell,
        workers: i32,
        goods: i32,
    ) -> (i32, i32) {
        // Early return if not enough workers
        if workers < 2 {
            return (workers, goods);
        }

        let mut workers_used = workers;

        if previous.population == 0 && previous.is_powerline_adjacent {
            cell.population = 1;
            cell.pollution = 1;
            workers_used -= 2;
        } else {
            let mut target_adjacents;
            let target_population;

            match previous.population {
                0 => {
                    target_adjacents = 1;
                    target_population = 1;
                }
                1 => {
                    target_adjacents = 2;
                    target_population = 1;
                }
                2 => {
                    target_adjacents = 4;
                    target_population = 2;
                }
                _ => return (workers, goods),
            }

            for cell_weak in &previous.neighbors {
                if let Some(neighbor) = cell_weak.upgrade() {
                    let neighbor = neighbor.borrow();

                    if neighbor.population >= target_population {
                        target_adjacents -= 1;
                    }

                    if target_adjacents == 0 {
                        cell.population = previous.population + 1;
                        cell.pollution = previous.pollution + 1;
                        workers_used -= 2;
                        break;
                    }
                }
            }
        }

        (workers_used, goods) // Note: goods is unchanged for industrial
    }

    pub fn count_adjacent_population(&self) -> u32 {
        let mut total = 0;
        for neighbor_weak in &self.neighbors {
            if let Some(neighbor) = neighbor_weak.upgrade() {
                let neighbor = neighbor.borrow();
                total += neighbor.population;
            }
        }
        total
    }
}

impl Display for MapCell {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.population == 0 {
            match &self.cell_type {
                CellType::Residential(c) => write!(f, "{}", c),
                CellType::Industrial(c) => write!(f, "{}", c),
                CellType::Commercial(c) => write!(f, "{}", c),
                CellType::Other(c) => write!(f, "{}", c),
            }
        } else {
            write!(f, "{}", self.population)
        }
    }
}

impl PartialEq for MapCell {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
            && self.cell_type == other.cell_type
            && self.population == other.population
            && self.pollution == other.pollution
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
        // Rule 1: Commercial zoned cells are prioritized over industrial zoned cells,
        //         and industrial cells over residential
        let self_type_char = match &self.cell_type {
            CellType::Commercial(_) => 'C',
            CellType::Industrial(_) => 'I',
            CellType::Residential(_) => 'R',
            CellType::Other(c) => *c,
        };

        let other_type_char = match &other.cell_type {
            CellType::Commercial(_) => 'C',
            CellType::Industrial(_) => 'I',
            CellType::Residential(_) => 'R',
            CellType::Other(c) => *c,
        };

        // Compare zone types with priority C > I > R
        match (self_type_char, other_type_char) {
            ('C', 'C') | ('I', 'I') | ('R', 'R') => {} // Same type, continue to next rule
            ('C', _) => return Ordering::Less,         // C has highest priority (should be first)
            (_, 'C') => return Ordering::Greater,      // Other has C, so it gets priority
            ('I', _) => return Ordering::Less,         // I has priority over R
            (_, 'I') => return Ordering::Greater,      // Other has I, so it gets priority
            _ => {}                                    // Continue for other cell types
        }

        // Rule 2: Larger population gets priority
        if self.population != other.population {
            return other.population.cmp(&self.population); // Reverse order: larger population first
        }

        // Rule 3: Greater adjacent population gets priority
        let self_adj_pop = self.count_adjacent_population();
        let other_adj_pop = other.count_adjacent_population();

        if self_adj_pop != other_adj_pop {
            return other_adj_pop.cmp(&self_adj_pop); // Reverse order: larger adjacent pop first
        }

        // Rules 4 & 5: Compare coordinates (smaller Y first, then smaller X)
        let (self_x, self_y) = self.position;
        let (other_x, other_y) = other.position;

        // Smaller Y first
        if self_y != other_y {
            return self_y.cmp(&other_y);
        }

        // Then smaller X
        self_x.cmp(&other_x)
    }
}

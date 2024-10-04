use std::{cell::RefCell, rc::Rc};

use crate::{cell_type::CellType, map_cell::MapCell};

pub type Grid = Vec<Vec<Rc<RefCell<MapCell>>>>;
pub trait GridHelper {
    fn deep_clone(&self) -> Self;
    fn track_adjacent_cells(&self);
    fn get_available_goods(&self) -> u32;
    fn get_available_workers(&self) -> u32;
}

impl GridHelper for Grid {
    fn deep_clone(&self) -> Self {
        self.iter()
            .map(|inner_vec| {
                inner_vec
                    .iter()
                    .map(|cell| {
                        let new_map_cell = MapCell {
                            position: cell.borrow().position,
                            type_: cell.borrow().type_,
                            population: cell.borrow().population,
                            pollution: cell.borrow().pollution,
                            is_powerline_adjacent: cell.borrow().is_powerline_adjacent,
                            adjacent_cells: vec![],
                        };

                        Rc::new(RefCell::new(new_map_cell))
                    })
                    .collect()
            })
            .collect()
    }

    fn track_adjacent_cells(&self) {
        for row in self {
            for cell in row {
                let (x, y) = cell.borrow().position;
                if let CellType::Other(_) = cell.borrow().type_ {
                    continue;
                }

                if x > 0 {
                    cell.borrow_mut()
                        .add_adjacent_cell(Rc::clone(&self[y][x - 1]));

                    if y > 0 {
                        cell.borrow_mut()
                            .add_adjacent_cell(Rc::clone(&self[y - 1][x - 1]));
                    }
                    if y < self.len() - 1 {
                        cell.borrow_mut()
                            .add_adjacent_cell(Rc::clone(&self[y + 1][x - 1]));
                    }
                }
                if x < self[y].len() - 1 {
                    cell.borrow_mut()
                        .add_adjacent_cell(Rc::clone(&self[y][x + 1]));

                    if y > 0 {
                        cell.borrow_mut()
                            .add_adjacent_cell(Rc::clone(&self[y - 1][x + 1]));
                    }
                    if y < self.len() - 1 {
                        cell.borrow_mut()
                            .add_adjacent_cell(Rc::clone(&self[y + 1][x + 1]));
                    }
                }
                if y > 0 {
                    cell.borrow_mut()
                        .add_adjacent_cell(Rc::clone(&self[y - 1][x]));
                }
                if y < self.len() - 1 {
                    cell.borrow_mut()
                        .add_adjacent_cell(Rc::clone(&self[y + 1][x]));
                }
            }
        }
    }

    fn get_available_goods(&self) -> u32 {
        let mut total_goods = 0;
        let mut sold_goods = 0;

        for row in self {
            for cell in row {
                match cell.borrow().type_ {
                    CellType::Industrial(_) => total_goods += cell.borrow().population,
                    CellType::Commercial(_) => sold_goods += cell.borrow().population,
                    _ => (),
                }
            }
        }

        total_goods - sold_goods
    }

    fn get_available_workers(&self) -> u32 {
        let mut total_workers = 0;
        let mut used_workers = 0;

        for row in self {
            for cell in row {
                match cell.borrow().type_ {
                    CellType::Residential(_) => total_workers += cell.borrow().population,
                    CellType::Industrial(_) | CellType::Commercial(_) => {
                        used_workers += cell.borrow().population
                    }
                    _ => (),
                }
            }
        }

        total_workers - used_workers
    }
}

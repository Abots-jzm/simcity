use crate::{
    config::Config,
    map_cell::{CellType, MapCell},
};
use std::{cell::RefCell, fs, rc::Rc};

pub type MapGrid = Vec<Vec<Rc<RefCell<MapCell>>>>;

pub struct Map {
    current: MapGrid,
    previous: Option<MapGrid>,
}

impl Map {
    pub fn from_config(config: &mut Config) -> Self {
        let contents = match fs::read_to_string(&config.region_layout_filename) {
            Ok(content) => content,
            Err(_) => {
                println!(
                    "Error: Couldn't find \"{}\". Please make sure the Region Layout file exists",
                    config.region_layout_filename
                );
                config.reinitialize();
                return Self::from_config(config);
            }
        };

        let grid: MapGrid = contents
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.split(',')
                    .enumerate()
                    .map(|(x, cell_char)| {
                        Rc::new(RefCell::new(MapCell {
                            position: (x.try_into().unwrap(), y.try_into().unwrap()),
                            cell_type: match cell_char {
                                "R" => CellType::Residential('R'),
                                "C" => CellType::Commercial('C'),
                                "T" => CellType::Industrial('I'),
                                _ => CellType::Other(cell_char.chars().next().unwrap()),
                            },
                            pollution: 0,
                            population: 0,
                            is_powerline_adjacent: false,
                            neighbors: vec![],
                        }))
                    })
                    .collect()
            })
            .collect();

        println!("{:#?}", grid);

        Map {
            current: grid,
            previous: None,
        }
    }
}

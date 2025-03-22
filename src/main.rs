use crate::config::Config;
use crate::map::Map;
use crate::simulation::Simulation;

mod config;
mod map;
mod map_cell;
mod simulation;

fn main() {
    let mut config = Config::from_user_input();
    let mut map = Map::from_config(&mut config);

    let mut simulation = Simulation::new(&config, &mut map);
    simulation.start();
}

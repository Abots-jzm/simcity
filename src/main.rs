use config::Config;
use map::Map;

mod config;
mod map;
mod map_cell;
mod simulation;

fn main() {
    let mut config = Config::from_user_input();
    let mut map = Map::from_config(&mut config);

    let mut simulation = simulation::Simulation::new(&mut config, &mut map);
    simulation.start();
}

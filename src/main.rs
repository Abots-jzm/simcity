use config::initialize_config;
use map::Map;

mod config;
mod map;
mod map_cell;

fn main() {
    let config = initialize_config();
    let map = Map::new(&config);
}

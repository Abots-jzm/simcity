use crate::config::Config;
use crate::map::Map;

mod config;
mod map;
mod map_cell;

fn main() {
    let mut config = Config::from_user_input();
    let map = Map::from_config(&mut config);
}

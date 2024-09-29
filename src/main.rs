use config::Config;

mod config;
mod map;
mod map_cell;

fn main() {
    let mut config: Config = Config::new();

    config.init();
}

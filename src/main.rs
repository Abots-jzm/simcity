use crate::config::Config;

mod config;

fn main() {
    let config = Config::from_user_input();
    println!("{:?}", config)
}

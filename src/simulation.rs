use crate::grid::GridHelper;
use crate::{config::Config, map::Map};

pub struct Simulation<'a> {
    time_step: u32,
    state: u32,
    config: &'a mut Config,
    map: &'a mut Map,
}

impl<'a> Simulation<'a> {
    pub fn new(config: &'a mut Config, map: &'a mut Map) -> Self {
        Self {
            time_step: 0,
            state: 0,
            config,
            map,
        }
    }

    pub fn start(&mut self) {
        println!("\nINITIAL REGION STATE");
        println!("{}\n", self.map);
        self.next();
    }

    fn next(&mut self) {
        self.time_step += 1;
        self.map.step();

        if (self.time_step % self.config.refresh_rate) == 0 {
            self.state += 1;
            self.print_current_state();
        }

        if self.should_simulation_end() {
            self.end();
        } else {
            self.next();
        }
    }

    fn end(&mut self) {
        println!("\nFINAL REGION STATE");
        println!("{}\n", self.map);
        println!(
            "Available Workers: {}",
            self.map.current.get_available_workers()
        );
        println!(
            "Available Goods: {}",
            self.map.current.get_available_goods()
        );
        self.map.spread_pollution();
        println!("\nFINAL POLLUTION SPREAD");
        println!("{}", self.map);
    }

    fn print_current_state(&self) {
        println!("STATE: {}", self.state);
        println!("Time Step: {}", self.time_step);
        println!("{}\n", self.map);
        println!(
            "Available Workers: {}",
            self.map.current.get_available_workers()
        );
        println!(
            "Available Goods: {}\n",
            self.map.current.get_available_goods()
        );
    }

    fn should_simulation_end(&self) -> bool {
        self.time_step >= self.config.time_limit
            || self
                .map
                .previous
                .as_ref()
                .map_or(false, |prev| &self.map.current == prev)
    }
}

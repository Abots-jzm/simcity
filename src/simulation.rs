use crate::{config::Config, map::Map};

pub struct Simulation<'a> {
    config: &'a Config,
    map: &'a mut Map,
    time_step: u32,
    state: u32,
}

impl<'a> Simulation<'a> {
    pub fn new(config: &'a Config, map: &'a mut Map) -> Self {
        Simulation {
            config,
            map,
            time_step: 0,
            state: 0,
        }
    }

    pub fn start(&mut self) {
        println!("\nINITIAL REGION STATE");
        println!("{}\n", self.map);
        self.next();
    }

    fn next(&mut self) {
        self.map.update_previous();
        Map::track_adjacency(&self.map.previous.as_ref().unwrap());
        self.time_step += 1;
        self.map.step();

        if self.time_step % self.config.refresh_rate == 0 {
            self.state += 1;
            self.print_current_state();
        }

        if self.time_step < 8 {
            self.next();
        }
        // if self.time_step < self.config.time_limit {
        //     self.next();
        // }
    }

    fn print_current_state(&self) {
        println!("State: {}", self.state);
        println!("Time Step: {}", self.time_step);
        println!("{}", self.map);
        println!(
            "Available Workers: {}",
            Map::get_available_workers(&self.map.current)
        );
        println!(
            "Available Goods: {}\n",
            Map::get_available_goods(&self.map.current)
        )
    }
}

use rand::Rng;
use serde::{Serialize};

#[derive(Serialize, Debug)]
pub struct Heartbeat {
    pub cpu_usage: f64,
    pub mem_usage: f64,
}

#[derive(Serialize, Debug)]
pub struct Temperature {
    pub temp: f64,
}

#[derive(Serialize, Debug)]
pub enum Event {
    Startup,
    Heartbeat(Heartbeat),
    Temperature(Temperature),
}

#[derive(Debug, Serialize)]
pub struct Device {
    pub id: u8,
    pub event: Event
}

impl Device {
    pub fn new(id: u8) -> Self {
        Device { id, event: Event::Startup }
    }

    pub fn event(&mut self, event: Event) {
        self.event = event;
    }

    pub fn generate_heartbeat() -> Event {
        let mut rng = rand::thread_rng();
        Event::Heartbeat(Heartbeat {
            cpu_usage: rng.gen_range(0.0..100.0),
            mem_usage: rng.gen_range(0.0..100.0),
        })
    }

    pub fn generate_temperature() -> Event {
        let mut rng = rand::thread_rng();
        Event::Temperature(Temperature {
            temp: rng.gen_range(-10.0..50.0),
        })
    }

    pub fn startup_message() -> Event {
        Event::Startup
    }
}

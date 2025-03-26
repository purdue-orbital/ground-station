use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GPS {
    pub x_pos: f64,
    pub y_pos: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Telemetry {
    pub altitude: f64,
    pub velocity: f64,
    pub temperture: f64,
}

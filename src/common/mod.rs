use flight_builder::prelude::*;
use telemetery::Telemetry;

pub mod telemetery;

pub fn CommonPlugin(s: &mut Scheduler) {
    s.add_resource(Telemetry {
        altitude: 0.0,
        velocity: 0.0,
        temperture: 0.0,
    });
}

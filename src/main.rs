use antenna_controller::AntennaControllerPlugin;
use common::CommonPlugin;
use flight_builder::prelude::*;
use general_compute::GeneralComputePlugin;
use radio::RadioPlugin;
use ui::UiPlugin;

mod antenna_controller;
mod common;
mod general_compute;
mod radio;
mod ui;

fn main() {
    let mut s = Scheduler::default();

    s.add_plugin(AntennaControllerPlugin);
    s.add_plugin(GeneralComputePlugin);
    s.add_plugin(RadioPlugin);
    s.add_plugin(UiPlugin);
    s.add_plugin(CommonPlugin);

    s.build().run();
}

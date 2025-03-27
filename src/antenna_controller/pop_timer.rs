use flight_builder::prelude::*;
use rand::random;

pub struct PopTimer {
    pub counter: u32,
}

fn pop_check(mut pop_timer: ResMut<PopTimer>) {
    pop_timer.counter += 1;

    if pop_timer.counter == 10 {
        pop();

        pop_timer.counter = 0;
    }
}

pub struct SensorReadings {
    pub bar: f32,
    pub temperature: f32,
    pub humidity: f32,
}
fn check_sensors(mut sensor_readings: ResMut<SensorReadings>) {
    sensor_readings.bar += (random::<f32>() * 10.0) - 5.0;
    sensor_readings.temperature += (random::<f32>() * 10.0) - 5.0;
    sensor_readings.humidity += (random::<f32>() * 10.0) - 5.0;
}

fn print_readings(readings: Res<SensorReadings>) {
    println!(
        "Bar: {} Temp: {} Humidity: {}",
        readings.bar, readings.temperature, readings.humidity
    );
}

fn pop() {
    println!("Pop!");
}

fn main() {
    let mut s = Scheduler::default();

    // Add pop timer resource
    s.add_resource(PopTimer { counter: 0 });

    // Add sensor reading resource
    s.add_resource(SensorReadings {
        bar: 0.0,
        temperature: 0.0,
        humidity: 0.0,
    });

    // Add a task that pops a balloon that runs at 1 hz (1/1)
    s.add_task(Schedule::Update(1.0), pop_check);

    // Add a task that updates sensor readings at 100hz
    s.add_task(Schedule::Update(1.0 / 100.0), check_sensors);

    // Print readings every 5 seconds
    s.add_task(Schedule::Update(5.0), print_readings);

    s.build().run();
}

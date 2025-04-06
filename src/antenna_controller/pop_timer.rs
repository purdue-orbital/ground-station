use flight_builder::prelude::*;
use rand::random;

// crates for ublox control
use chrono::prelude::*;
use serialport::SerialPort;
use std::convert::TryInto;
use std::thread;
use std::time::Duration;
use ublox::*;

// crates for pico control
use cortex_m::prelude::*;
use embedded_hal::pwm::SetDutyCycle;
use fugit::ExtU32;
use panic_halt as _;
use rp_pico::hal::pac;
use rp_pico::hal;

pub struct SensorReadings {
    pub bar: f32,
    pub temperature: f32,
    pub humidity: f32,
}

pub struct PositionReadings {
  pub long: f32,
  pub lat: f32,
  pub alt: f32,
}

fn check_sensors(mut sensor_readings: ResMut<SensorReadings>) {
    sensor_readings.bar += (random::<f32>() * 10.0) - 5.0;
    sensor_readings.temperature += (random::<f32>() * 10.0) - 5.0;
    sensor_readings.humidity += (random::<f32>() * 10.0) - 5.0;
}

fn check_position(mut position_readings: ResMut<PositionReadings>) {
  // device
  //     .on_data_available(|packet| match packet {
  //         PacketRef::MonVer(packet) => {
  //             println!(
  //                 "SW version: {} HW version: {}; Extensions: {:?}",
  //                 packet.software_version(),
  //                 packet.hardware_version(),
  //                 packet.extension().collect::<Vec<&str>>()
  //             );
  //         },
  //         PacketRef::NavPvt(pvt) => {
  //             let has_time = pvt.fix_type() == GpsFix::Fix3D
  //                 || pvt.fix_type() == GpsFix::GPSPlusDeadReckoning
  //                 || pvt.fix_type() == GpsFix::TimeOnlyFix;
  //             let has_posvel = pvt.fix_type() == GpsFix::Fix3D
  //                 || pvt.fix_type() == GpsFix::GPSPlusDeadReckoning;

  //             if has_posvel {
  //                 let pos: Position = (&pvt).into();
  //                 let vel: Velocity = (&pvt).into();
  //                 println!(
  //                     "Latitude: {:.5} Longitude: {:.5} Altitude: {:.2}m",
  //                     pos.lat, pos.lon, pos.alt
  //                 );
  //                 println!(
  //                     "Speed: {:.2} m/s Heading: {:.2} degrees",
  //                     vel.speed, vel.heading
  //                 );
  //                 println!("Sol: {:?}", pvt);
  //             }

  //             if has_time {
  //                 let time: DateTime<Utc> = (&pvt)
  //                     .try_into()
  //                     .expect("Could not parse NAV-PVT time field to UTC");
  //                 println!("Time: {:?}", time);
  //             }
  //         },
  //         PacketRef::EsfRaw(raw) => {
  //             println!("Got raw message: {:?}", raw);
  //         },
  //         _ => {
  //             println!("{:?}", packet);
  //         },
  //     })
  //     .expect("Failed to consume buffer");
  let pos: Position = (&pvt).into();
  position_readings.long = pos.long;
  position_readings.lat = pos.lat;
  position_readings.alt = pos.alt;
}

fn set_position(setting: i8) {
  let _ = channel.set_duty_cycle(setting);
}

fn print_readings(readings: Res<PositionReadings>) {
    println!(
        "Latitude: {} Longitude: {} Altitude: {}",
        readings.lat, readings.long, readings.alt;
    );
}

fn main() {

    let mut cli = ublox_device::cli::CommandBuilder::default().build();
    // cli = cli
    //     .about("uBlox multi-threaded CLI example program for ESF/ADR operation.")
    //     .name("Demonstrate usage of the uBlox package for ESF+ADR mode with one thread for receiving and one for sending UBX messages.")
    //     .author(clap::crate_authors!());

    let serialport = ublox_device::cli::Command::serialport(cli.clone());
    // Clone the port for the sending side
    let serialport_clone = serialport.try_clone().expect("Failed to clone serialport");

    let mut device = ublox_device::Device::new(serialport);

    let baud_rate = ublox_device::cli::Command::arg_boud(cli);
    sending_thread(baud_rate, serialport_clone);

    // Start reading data
    println!("Opened uBlox device, waiting for messages...");

    // PWM control setup

    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // Configure the Timer peripheral in count-down mode
    let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);
    let mut count_down = timer.count_down();

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Init PWMs
    let mut pwm_slices = hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);

    // Configure PWM0
    let pwm = &mut pwm_slices.pwm0;
    pwm.set_ph_correct();
    pwm.set_div_int(20u8); // 50 hz
    pwm.enable();

    // Output channel B on PWM0 to the GPIO1 pin
    let channel = &mut pwm.channel_b;
    channel.output_to(pins.gpio1);

    // end of device setup

    let mut s = Scheduler::default();

    // Add pop timer resource
    s.add_resource(PopTimer { counter: 0 });

    // Add sensor reading resource
    s.add_resource(SensorReadings {
      bar: 0.0,
      temperature: 0.0,
      humidity: 0.0,
    });

    s.add_resource(PositionReadings {
      long: 0.0,
      lat: 0.0,
      alt: 0.0
    });

    // Add a task that updates sensor readings at 100hz
    s.add_task(Schedule::Update(1.0 / 100.0), check_position);

    // Print readings every 5 seconds
    s.add_task(Schedule::Update(5.0), print_readings);

    s.build().run();
}

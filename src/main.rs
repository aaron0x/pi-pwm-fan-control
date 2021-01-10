use rppal::pwm::{Channel, Polarity, Pwm};
use signal_hook;

use std::sync::{atomic::AtomicBool, atomic::Ordering, Arc};
use std::{process::Command, str, thread, time};

#[derive(PartialEq, Debug)]
struct FanConfig {
    hz: f64,
    duty_cycle: f64,
}

fn main() {
    let high = FanConfig {
        hz: 800.0,
        duty_cycle: 0.99,
    };
    let median = FanConfig {
        hz: 600.0,
        duty_cycle: 0.99,
    };
    let low = FanConfig {
        hz: 400.0,
        duty_cycle: 0.99,
    };

    let mut speed = &high;
    let pwm = Pwm::with_frequency(
        Channel::Pwm0,
        speed.hz,
        speed.duty_cycle,
        Polarity::Normal,
        true,
    )
    .expect("failed to new pwm");

    let terminal = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&terminal))
        .expect("failed to register sig handler");
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&terminal))
        .expect("failed to register sig handler");

    while !terminal.load(Ordering::Relaxed) {
        thread::sleep(time::Duration::from_secs(3));

        let output = Command::new("sh")
            .arg("-c")
            .arg("vcgencmd measure_temp")
            .output()
            .expect("failed to measure temperature");
        let temperature = str::from_utf8(&output.stdout[5..7])
            .expect("failed to extract temperature str")
            .parse::<u32>()
            .expect("failed to convert temperature to u32");

        let new_speed;
        if temperature >= 55 {
            new_speed = &high;
        } else if temperature >= 45 {
            new_speed = &median;
        } else {
            new_speed = &low;
        }

        println!("temperature: {:?}, speed: {:?}", temperature, new_speed);

        if new_speed != speed {
            speed = new_speed;
            pwm.set_frequency(speed.hz, speed.duty_cycle)
                .expect("failed to change speed");
        }
    }
}

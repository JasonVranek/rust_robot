use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::pwm::{Channel, Polarity, Pwm};

pub mod motors {
    fn init() {

    }

    fn rw_forward() {

    }

    fn rw_back() {

    }

    fn lw_forward() {

    }

    fn lw_back() {
        
    }

    fn drive_right(speed: &u64) {

    }

    fn drive_left(speed: &u64) {

    }

    fn forward(speed: u64) {
        lw_forward();
        rw_forward();
        drive_left(&speed);
        drive_right(&speed);
    }
}




fn main() -> Result<(), Box<dyn Error>> {
    // Enable PWM channel 0 (BCM GPIO 18, physical pin 12) at 2 Hz with a 25% duty cycle.
    let pwm = Pwm::with_frequency(Channel::Pwm0, 2.0, 0.25, Polarity::Normal, true)?;

    // Sleep for 2 seconds while the LED blinks.
    thread::sleep(Duration::from_secs(2));

    // Reconfigure the PWM channel for an 8 Hz frequency, 50% duty cycle.
    pwm.set_frequency(8.0, 0.5)?;

    thread::sleep(Duration::from_secs(3));

    Ok(())

    // When the pwm variable goes out of scope, the PWM channel is automatically disabled.
    // You can manually disable the channel by calling the Pwm::disable() method.
}
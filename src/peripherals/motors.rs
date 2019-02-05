use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, OutputPin};
use rppal::pwm::{Channel, Polarity, Pwm};

// DC Motor - HDBRIDGE  
const ENA: u8 = 18;  //PWM0, Left Motor
const IN1: u8 = 5;
const IN2: u8 = 6;

const ENB: u8 = 19;  //PWM1, Right Motor
const IN3: u8 = 13;
const IN4: u8 = 26;

// DC Motor - Encoders
const L_ENC_YEL: u8 = 20;
const L_ENC_GRE: u8 = 21;

const R_ENC_YEL: u8 = 23;
const R_ENC_GRE: u8 = 24;

pub struct DriveSys {
	in1: OutputPin,
	in2: OutputPin,
	in3: OutputPin,
	in4: OutputPin,
	en_a: Pwm,
	en_b: Pwm,
}

impl DriveSys {
	fn new() -> Result<DriveSys, Box<dyn Error>> {
		// Set the directional pins as output and init Pwm channels
		let d = DriveSys {
			in1: Gpio::new()?.get(IN1)?.into_output(),
			in2: Gpio::new()?.get(IN2)?.into_output(),
			in3: Gpio::new()?.get(IN3)?.into_output(),
			in4: Gpio::new()?.get(IN4)?.into_output(),
			en_a: Pwm::new(Channel::Pwm0)?,
			en_b: Pwm::new(Channel::Pwm1)?,
		};
		Ok(d)
	}

	fn lw_forward(&self) {
		// Right motor forward
		// IN1 high, IN2 low
		
	}

	fn lw_back(&self) {
		// Left motor backward
		// IN1 low, IN2 high
	}

	fn rw_forward(&self) {
		// Right motor Forward
		// IN3 low, IN4 high
		
	}

	fn rw_back(&self,) {
		// Right motor backward
		// IN3 high, IN4 low
	}

	fn drive_right(&self, speed: &u64) {

	}

	fn drive_left(&self, speed: &u64) {

	}

	fn forward(&self, speed: u64) {
		self.lw_forward();
		self.rw_forward();
		self.drive_left(&speed);
		self.drive_right(&speed);
	}
}



#[cfg(test)]
mod tests {
	use super::*;

	//#[test]
	fn test_right_motor_forward() {
		let pwm = Pwm::with_frequency(Channel::Pwm0, 2.0, 0.25, Polarity::Normal, true)?;

		// Sleep for 2 seconds while the LED blinks.
		thread::sleep(Duration::from_secs(2));

		// Reconfigure the PWM channel for an 8 Hz frequency, 50% duty cycle.
		pwm.set_frequency(8.0, 0.5)?;

		thread::sleep(Duration::from_secs(3));

		Ok(())
	}
}















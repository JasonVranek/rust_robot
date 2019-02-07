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

const MAX_PWM: u64 = 1024;
const DEF_FREQUENCY: f64 = 60.0;		// Hz
const DEF_DUTY_CYCLE: f64 = 0.50;	// [0,1.0]

pub struct DriveSys {
	in1: OutputPin,
	in2: OutputPin,
	in3: OutputPin,
	in4: OutputPin,
	en_a: Pwm,
	en_b: Pwm,
}

impl DriveSys {
	pub fn new() -> Result<DriveSys, Box<dyn Error>> {
		// Set the directional pins as output and init Pwm channels
		let d = DriveSys {
			in1: Gpio::new()?.get(IN1)?.into_output(),
			in2: Gpio::new()?.get(IN2)?.into_output(),
			in3: Gpio::new()?.get(IN3)?.into_output(),
			in4: Gpio::new()?.get(IN4)?.into_output(),
			en_a: Pwm::with_frequency(Channel::Pwm0, DEF_FREQUENCY, DEF_DUTY_CYCLE, Polarity::Normal, false)?,
			en_b: Pwm::with_frequency(Channel::Pwm1, DEF_FREQUENCY, DEF_DUTY_CYCLE, Polarity::Normal, false)?,
			//en_a: Pwm::new(Channel::Pwm0)?,
			//en_b: Pwm::new(Channel::Pwm1)?,
		};
		Ok(d)
	}

	fn lw_forward(&mut self) {
		// Right motor forward
		// IN1 high, IN2 low
		self.in1.set_high();
		self.in2.set_low();
	}

	fn lw_back(&mut self) {
		// Left motor backward
		// IN1 low, IN2 high
		self.in1.set_low();
		self.in2.set_high();
	}

	fn rw_forward(&mut self) {
		// Right motor Forward
		// IN3 low, IN4 high
		self.in3.set_low();
		self.in4.set_high();
	}

	fn rw_back(&mut self) {
		// Right motor backward
		// IN3 high, IN4 low
		self.in3.set_high();
		self.in4.set_low();
	}

	fn drive_right(&mut self, speed: &u64) -> Result<(), Box<dyn Error>> {
		let dc = DriveSys::u64_to_duty_cycle(speed.clone());
		self.en_b.set_duty_cycle(dc)?;//.expect("Failed to set duty cycle en_b");
		self.en_b.enable()?;//.expect("Failed to enable PWM enb");
		println!("PWM en_b enabled!");
		Ok(())
	}

	fn drive_left(&mut self, speed: &u64) -> Result<(), Box<dyn Error>> {
		let dc: f64 = DriveSys::u64_to_duty_cycle(speed.clone());
		self.en_a.set_duty_cycle(dc).expect("Failed to set duty cycle en_a");
		self.en_a.enable().expect("Failed to enable PWM ena");
		println!("PWM en_a enabled!");
		Ok(())
	}
	
	pub fn stop(&mut self) -> Result<(), Box<dyn Error>> {
		self.en_a.disable()?;//.expect("Failed to disable PWM ena");
		self.en_b.disable()?;//.expect("Failed to disable PWM enb");
		Ok(())
	}

	pub fn forward(&mut self, speed: u64) {
		self.lw_forward();
		self.rw_forward();
		self.drive_left(&speed).expect("couldn't drive left motor");
                println!("Started left wheel");
		//self.drive_right(&speed).expect("couldn't drive right motor");
	}
	
	pub fn backward(&mut self, speed: u64) {
		self.lw_back();
		self.rw_back();
		self.drive_left(&speed).expect("couldn't drive left motor");
		self.drive_right(&speed).expect("couldn't drive right motor");
	}	
	
	pub fn tank_right(&mut self, speed: u64) {
		self.lw_forward();
		self.rw_back();
		self.drive_left(&speed).expect("couldn't drive left motor");
		self.drive_right(&speed).expect("couldn't drive right motor");
	}
	
	pub fn tank_left(&mut self, speed: u64) {
		self.lw_back();
		self.rw_forward();
		self.drive_left(&speed).expect("couldn't drive left motor");
		self.drive_right(&speed).expect("couldn't drive right motor");
	}
	
	// Drives wheels, negative vals for reversing
	pub fn drive(&mut self, left_vel: i64, right_vel: i64) {
		if left_vel < 0 {
			self.lw_back();
		}
		if right_vel < 0 {
			self.rw_back();
		}
		self.drive_left(&DriveSys::scaled_pwm(left_vel)).expect("couldn't drive left motor");
		self.drive_right(&DriveSys::scaled_pwm(right_vel)).expect("couldn't drive right motor");
	}
	
	fn scaled_pwm(x: i64) -> u64 {
		let x: u64 = x.abs() as u64;
		if x > MAX_PWM {
			return MAX_PWM;
		} 
		x
	}
	
	fn u64_to_duty_cycle(u: u64) -> f64 {
		let f: f64 = u as f64 / MAX_PWM as f64;
		f
	}
		
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_forward() {
		let mut motors = DriveSys::new().unwrap();
		//~ let mut motors = DriveSys {
			//~ in1: Gpio::new().expect("gpio err").get(IN1).expect("set gpio err").into_output(),
			//~ in2: Gpio::new().expect("gpio err").get(IN2).expect("set gpio err").into_output(),
			//~ in3: Gpio::new().expect("gpio err").get(IN3).expect("set gpio err").into_output(),
			//~ in4: Gpio::new().expect("gpio err").get(IN4).expect("set gpio err").into_output(),
			//~ en_a: Pwm::new(Channel::Pwm0).expect("pwm0 err"),
			//~ en_b: Pwm::new(Channel::Pwm1).expect("pwm1 err"),
		//~ };
		println!("Motor gpio's initialized");

		motors.forward(500);
		// Sleep for 2 seconds while the LED blinks.
		thread::sleep(Duration::from_secs(2));

		motors.stop();

		
	}
	
	#[test]
	fn test_gpio() {
		 let mut pin = Gpio::new().unwrap().get(16).unwrap().into_output();
		 pin.set_low();
		 thread::sleep(Duration::from_secs(2));
		 
		 pin.set_high();
		 
	}
}















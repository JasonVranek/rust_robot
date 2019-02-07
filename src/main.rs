extern crate robot;

use robot::io::tcp_server;
use robot::controller::Controller;
use robot::peripherals::motors::DriveSys;

use std::error::Error;
use std::thread;
use std::time::Duration;


fn main() -> Result<(), Box<dyn Error>> {

    //let tcp_server = tcp_server::new_tcp_listener(format!("0.0.0.0:5000"));
    //let mut controller = Controller::new();
    //controller.push(tcp_server);

    //controller.run();
    println!("initializing drive system");
    
    let mut motors = DriveSys::new()?;
    println!("initialized motors");

    motors.forward(500);
    // Sleep for 2 seconds while the LED blinks.
    thread::sleep(Duration::from_secs(2));

    motors.stop();
	
    Ok(())
}

extern crate robot;

use robot::io::tcp_server;
use robot::controller::Controller;
use robot::peripherals::motors;

fn main() {

    let tcp_server = tcp_server::new_tcp_listener(format!("0.0.0.0:5000"));
    let mut controller = Controller::new();
    controller.push(tcp_server);

    controller.run();
}

use crate::controller::Task;
use crate::io::tcp_json::JsonMsg;

use tokio::net::TcpListener;
use tokio::prelude::*;

pub fn new_tcp_listener(address: String) -> Task { 
     // Bind a TcpListener to a local port
    let addr = address.parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    println!("Running server on {}", addr);

    // start a tcp server that accepts JSON objects 
    let tcp_server = listener.incoming().for_each(move |socket| {

        // Deserialize the stream from the socket
        let deserialized = JsonMsg::deserialize(socket).map_err(|e| println!("ERR: {:?}", e));

        // Spawn a task that converts JSON to an Order and adds to queue
        tokio::spawn(deserialized.for_each(move |msg| {
            println!("Deserialzed: {:?}", msg);
            match JsonMsg::process_msg(msg) {
                Ok(json) => println!("{:?}", json),
                Err(e) => println!("{:?}", e),
            }
            
            Ok(())
        }));

        Ok(())
    })
    .map_err(|_| ());

    Task {
        task: Box::new(tcp_server),
    }
}



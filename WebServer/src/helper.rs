use zmq::Socket;
use zmq::Error;
use serde_cbor::to_vec;
use serde_cbor::from_slice;

use std::collections::HashMap;

pub fn connect() -> Socket {
    let ctx = zmq::Context::new();

    let socket = ctx.socket(zmq::REQ).unwrap();
    socket.connect("tcp://localhost:5555").unwrap();

    socket
}

pub fn send_message(socket: &Socket, data: &[u8]) {
    socket.send(data, 0).unwrap();
}

pub fn get_controller_state(socket: &Socket) -> Result<HashMap<String, String>, Error> {
    let mut data = HashMap::new();
    data.insert("type", "get");
    data.insert("value", "state");
    let encoded = to_vec(&data);
    send_message(&socket, &encoded.unwrap());
    
    match socket.recv_bytes(0) {
        Ok(value) => {
            let value: HashMap<String, String> = from_slice(&value).unwrap();
            Ok(value)
        },
        Err(e) => Err(e),
    }
}
use zmq::Socket;

pub fn connect() -> Socket {
    let ctx = zmq::Context::new();

    let socket = ctx.socket(zmq::REQ).unwrap();
    socket.connect("tcp://localhost:5555").unwrap();

    socket
}

pub fn send_message(socket: Socket, data: &[u8]) {
    socket.send(data, 0).unwrap();
}
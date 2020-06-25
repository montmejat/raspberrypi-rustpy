use zmq::Socket;
use zmq::Error;
use serde_cbor::to_vec;
use serde_cbor::from_slice;
use std::collections::HashMap;
use std::str::from_utf8;
use std::process::Command;

pub fn get_cpu_temp() -> String {
    let output = Command::new("sh").arg("-c").arg("/opt/vc/bin/vcgencmd measure_temp").output().expect("failed to execute process");
    let cpu_temp = output.stdout;
    let temp = from_utf8(&cpu_temp).unwrap().replace("temp=", "");
    temp
}

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

pub fn get_controller_settings(socket: &Socket) -> Result<HashMap<String, String>, Error> {
    let mut data = HashMap::new();
    data.insert("type", "get");
    data.insert("value", "settings");
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

pub fn pause_controller(socket: &Socket) {
    let mut data = HashMap::new();
    data.insert("type", "action");
    data.insert("value", "pause");
    let encoded = to_vec(&data);

    send_message(&socket, &encoded.unwrap());
}

pub fn unpause_controller(socket: &Socket) {
    let mut data = HashMap::new();
    data.insert("type", "action");
    data.insert("value", "unpause");
    let encoded = to_vec(&data);

    send_message(&socket, &encoded.unwrap());
}

pub fn to_html(variable: (String, String)) -> String {
    let (variable_name, value) = variable;

    if value.contains(":") {
        let values: Vec<&str> = value.split(":").collect();

        format!("<div class=\"col-4\">
                    <b>{}</b>
                </div>
                <div class=\"col-8 pl-0\" style=\"height: 25px; background: #e9ecef; display: flex; align-items: center;\">
                    <span class=\"badge badge-secondary\">{}</span>
                    <input type=\"range\" min=\"{}\" max=\"{}\" value=\"{}\" class=\"slider\">
                    <span class=\"badge badge-secondary\">{}</span>
                </div>", variable_name, values[0], values[0], values[2], values[1], values[2])
    } else {
        format!("<div class=\"col-4 col-form-label\">
                    <b>{}</b>
                </div>
                <div class=\"col-8 pl-0\">
                    <input class=\"form-control\" type=\"text\" value=\"{}\">
                </div>", variable_name, value)
    }
}
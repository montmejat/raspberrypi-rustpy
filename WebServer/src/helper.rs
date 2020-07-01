pub mod raspberry {
    use std::process::Command;
    use std::str::from_utf8;

    pub fn get_cpu_temp() -> String {
        let output = Command::new("sh").arg("-c").arg("/opt/vc/bin/vcgencmd measure_temp").output().expect("failed to execute process");
        let cpu_temp = output.stdout;
        let temp = from_utf8(&cpu_temp).unwrap().replace("temp=", "");
        temp
    }
}

pub mod script_controller {
    use zmq::Socket;
    use zmq::Error;
    use serde_cbor::to_vec;
    use serde_cbor::from_slice;
    use std::collections::HashMap;
    use std::process::Command;
    use std::str::from_utf8;

    pub struct Slider {
        pub name: String,
        pub min: u32,
        pub max: u32,
        pub value: u32,
    }

    pub struct Variable {
        pub name: String,
        pub value: String,
    }

    pub fn is_running() -> bool {
        let output = Command::new("sh").arg("-c").arg("ps -au | grep python3").output().expect("failed to execute process");
        from_utf8(&output.stdout).unwrap().contains("demo_controller_app.py")
    }

    pub fn connect() -> Socket {
        let ctx = zmq::Context::new();

        let socket = ctx.socket(zmq::REQ).unwrap();
        socket.connect("tcp://localhost:5555").unwrap();

        socket
    }

    pub fn send_message(socket: &Socket, data: HashMap<&str, &str>) {
        let encoded = to_vec(&data);
        socket.send(&encoded.unwrap(), 0).unwrap();
    }

    pub fn get_state(socket: &Socket) -> Result<HashMap<String, String>, Error> {
        let mut data = HashMap::new();
        data.insert("type", "get");
        data.insert("value", "state");
        send_message(&socket, data);
        
        match socket.recv_bytes(0) {
            Ok(value) => {
                let value: HashMap<String, String> = from_slice(&value).unwrap();
                Ok(value)
            },
            Err(e) => Err(e),
        }
    }

    pub fn get_settings(socket: &Socket) -> Result<(Vec<Slider>, Vec<Variable>), Error> {
        let mut data = HashMap::new();
        data.insert("type", "get");
        data.insert("value", "settings");
        send_message(&socket, data);
        
        match socket.recv_bytes(0) {
            Ok(value) => {
                let settings: HashMap<String, String> = from_slice(&value).unwrap();
                let mut settings_sliders = Vec::new();
                let mut settings_others = Vec::new();
                
                for (k, v) in settings {
                    if v.contains(":") {
                        let slider_values: Vec<&str> = v.split(":").collect();
                        settings_sliders.push(Slider { 
                            name: k,
                            min: slider_values[0].parse::<u32>().unwrap(),
                            value: slider_values[1].parse::<u32>().unwrap(),
                            max: slider_values[2].parse::<u32>().unwrap() 
                        });
                    } else {
                        settings_others.push(Variable { name: k, value: v});
                    }
                }

                Ok((settings_sliders, settings_others))
            },
            Err(e) => Err(e),
        }
    }

    pub fn pause(socket: &Socket) {
        let mut data = HashMap::new();
        data.insert("type", "action");
        data.insert("value", "pause");
        send_message(&socket, data);
    }

    pub fn unpause(socket: &Socket) {
        let mut data = HashMap::new();
        data.insert("type", "action");
        data.insert("value", "unpause");
        send_message(&socket, data);
    }

    pub mod web {
        use crate::helper::script_controller;

        pub fn get_navbar_info() -> (String, String) {
            let mut action = "";
            let mut icon_name = "x-circle";
            
            if script_controller::is_running() {
                let socket = script_controller::connect();

                match script_controller::get_state(&socket) {
                    Ok(value) => {
                        match value.get("paused") {
                            Some(paused) => {
                                if paused == "false" {
                                    action = "pause";
                                    icon_name = "pause";
                                } else {
                                    action = "unpause";
                                    icon_name = "play";
                                }
                            },
                            None => {}
                        }
                        
                    },
                    Err(_) => println!("Error retreiving state of controller...")
                }
            } else {
                action = "";
                icon_name = "x-circle";
            }

            (action.to_string(), icon_name.to_string())
        }
    }
}
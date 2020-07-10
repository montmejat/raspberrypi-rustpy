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
    use serde::{Deserialize, Serialize};
    use serde_cbor::to_vec;
    use serde_cbor::from_slice;
    use std::collections::HashMap;
    use std::process::Command;
    use std::str::from_utf8;

    #[derive(Serialize, Deserialize)]
    pub struct Slider {
        pub name: String,
        pub min: u32,
        pub max: u32,
        pub value: u32,
    }

    #[derive(Serialize, Deserialize)]
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

pub mod websocket {
    use crate::helper;

    use futures_util::future::{select, Either};
    use futures_util::{SinkExt, StreamExt};
    use std::net::SocketAddr;
    use std::time::Duration;
    use tokio::net::TcpStream;
    use tokio_tungstenite::{accept_async, tungstenite::Error};
    use tungstenite::{Message, Result};
    use serde_json::{json, Value};

    pub async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
        if let Err(e) = handle_connection(peer, stream).await {
            match e {
                Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
                err => println!("Error processing connection: {}", err),
            }
        }
    }
    
    /// Read incoming WebSocket messages and send a message periodically every two second.
    pub async fn handle_connection(peer: SocketAddr, stream: TcpStream) -> Result<()> {
        let ws_stream = accept_async(stream).await.expect("Failed to accept");
        println!("[WEBSOCKET] New WebSocket connection: {}", peer);
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();
        let mut interval = tokio::time::interval(Duration::from_millis(2000));

        let mut msg_fut = ws_receiver.next();
        let mut tick_fut = interval.next();
        
        let mut is_pyscript_running_old = false;
        let mut is_pyscript_running;
        let mut data;
        let mut page = None;

        loop {
            match select(msg_fut, tick_fut).await {
                Either::Left((msg, tick_fut_continue)) => {
                    match msg {
                        Some(msg) => {
                            let msg = msg?;
                            
                            match serde_json::from_str::<Value>(&msg.to_text().unwrap()) {
                                Ok(value) => {
                                    match value["command"].to_string().as_str() {
                                        "\"pause\"" => {
                                            let socket = helper::script_controller::connect();
                                            helper::script_controller::pause(&socket);
                                        },
                                        "\"unpause\"" => {
                                            let socket = helper::script_controller::connect();
                                            helper::script_controller::unpause(&socket);
                                        },
                                        _ => {}
                                    }
                                },
                                Err(_) => {
                                    page = Some(msg.to_text().unwrap().to_string());
                                    println!("[WEBSOCKET] Connection opened at '{}'", msg.to_text().unwrap());
                                },
                            }

                            tick_fut = tick_fut_continue; // Continue waiting for tick.
                            msg_fut = ws_receiver.next(); // Receive next WebSocket message.
                        }
                        None => break, // WebSocket stream terminated.
                    };
                }
                Either::Right((_, msg_fut_continue)) => {
                    is_pyscript_running = helper::script_controller::is_running();

                    if is_pyscript_running != is_pyscript_running_old {
                        if is_pyscript_running {
                            ws_sender.send(Message::Text("Python controller is online.".to_owned())).await?
                        } else {
                            ws_sender.send(Message::Text("Python controller is offline.".to_owned())).await?
                        }
                    }

                    match page {
                        Some(ref value) => {
                            let (action, icon_name) = helper::script_controller::web::get_navbar_info();
                            if value == "index" {
                                let cpu_temp = helper::raspberry::get_cpu_temp();
        
                                data = json!({
                                    "cpu_temp": cpu_temp,
                                    "is_pyscript_running": is_pyscript_running,
                                    "navbar": json!({
                                        "action": action,
                                        "icon_name": icon_name,
                                    }),
                                });
                            } else {
                                let mut settings_sliders = Vec::<helper::script_controller::Slider>::new();
                                let mut settings_others = Vec::<helper::script_controller::Variable>::new();

                                if is_pyscript_running {
                                    let socket = helper::script_controller::connect();
                            
                                    match helper::script_controller::get_settings(&socket) {
                                        Ok((sliders, others)) => {
                                            settings_sliders = sliders;
                                            settings_others = others;
                                        },
                                        Err(_) => println!("Error retreiving demo settings of controller...")
                                    }
                                }

                                data = json!({
                                    "is_pyscript_running": is_pyscript_running,
                                    "navbar": json!({
                                        "action": action,
                                        "icon_name": icon_name,
                                    }),
                                    "variables": json!({
                                        "sliders": settings_sliders,
                                        "others": settings_others,
                                    }),
                                });
                            }

                            match serde_json::to_string(&data) {
                                Ok(value) => ws_sender.send(Message::Text(value.to_owned())).await?,
                                Err(_) => {}
                            }
                        },
                        None => {}
                    }

                    is_pyscript_running_old = is_pyscript_running;
                    msg_fut = msg_fut_continue; // Continue receiving the WebSocket message.
                    tick_fut = interval.next(); // Wait for next tick.
                }
            }
        }
    
        Ok(())
    }
}

pub mod passwords {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use crypto::sha2::Sha256;
    use crypto::digest::Digest;

    #[derive(Debug, Clone)]
    pub struct PasswordError;

    pub fn check_password(user_in: &str, password_in: &str) -> Result<String, PasswordError> {
        let filename = "users.txt";
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
    
        for (_, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            let tokens: Vec<&str> = line.split(':').collect();
            
            if user_in == tokens[0] {
                let mut hasher = Sha256::new();
                hasher.input_str(password_in);
                let result = hasher.result_str();

                if result == tokens[1] {
                    return Ok(tokens[2].to_string());
                } else {
                    return Err(PasswordError);
                }
            }
        }

        Err(PasswordError)
    }
}
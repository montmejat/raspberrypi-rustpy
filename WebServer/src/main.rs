#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

mod helper;

use std::result::Result;
use std::collections::HashMap;
use std::sync::Mutex;
use std::{io::{Read, Write}, time};
use std::io;

use rocket::State;
use rocket::response::Redirect;
use rocket::request::{FromForm, FormItems, Form};
use rocket::http::{Cookie, Cookies};
use rocket_contrib::serve::StaticFiles;

use askama::Template;

use tokio::net::TcpListener;

use bluetooth_serial_port::{BtProtocol, BtSocket};

struct ServerState {
    logged_in_user: Mutex<Option<String>>,
}

#[derive(Template)]
#[template(path = "demo.html")]
struct DemoTemplate {
    logged_in: bool,
    icon_name: String,
    settings_sliders: Vec<helper::script_controller::Slider>,
    settings_others: Vec<helper::script_controller::Variable>,
    is_running: bool,
}

#[derive(Template)]
#[template(path = "index.html")]
struct HomeTemplate {
    logged_in: bool,
    icon_name: String,
    cpu_temp: String,
    is_running: bool,
}

#[derive(Template)]
#[template(path = "logout.html")]
struct LogoutTemplate {
    logged_in: bool,
    username: String,
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    any_user_online: bool,
    error: bool,
}

#[derive(Template)]
#[template(path = "maintenance.html")]
struct MaintenanceTemplate {
    admin: bool,
    logged_in: bool,
    icon_name: String,
    is_running: bool,
}

#[get("/")]
fn index(mut cookies: Cookies, state: State<ServerState>) -> HomeTemplate {
    let is_running = helper::script_controller::is_running();

    let (_, icon_name) = helper::script_controller::web::get_navbar_info();
    let cpu_temp = helper::raspberry::get_cpu_temp();

    match cookies.get_private("username") {
        Some(username) => {
            match &*state.logged_in_user.lock().unwrap() {
                Some(logged_user) => {
                    if logged_user == username.value() {
                        return HomeTemplate {
                            logged_in: true,
                            icon_name: icon_name.to_string(),
                            cpu_temp: cpu_temp.to_string(),
                            is_running: is_running,
                        }
                    }
                },
                None => {}
            }
        },
        None => {}
    }

    HomeTemplate {
        logged_in: false,
        icon_name: icon_name.to_string(),
        cpu_temp: cpu_temp.to_string(),
        is_running: is_running,
    }
}

#[get("/demo")]
fn demo(mut cookies: Cookies, state: State<ServerState>) -> DemoTemplate {
    let is_running = helper::script_controller::is_running();

    let (_, icon_name) = helper::script_controller::web::get_navbar_info();
    let mut settings_sliders = Vec::<helper::script_controller::Slider>::new();
    let mut settings_others = Vec::<helper::script_controller::Variable>::new();

    if is_running {
        let socket = helper::script_controller::connect();

        match helper::script_controller::get_settings(&socket) {
            Ok((sliders, others)) => {
                settings_sliders = sliders;
                settings_others = others;
            },
            Err(_) => println!("Error retreiving demo settings of controller...")
        }
    }

    match cookies.get_private("username") {
        Some(username) => {
            match &*state.logged_in_user.lock().unwrap() {
                Some(logged_user) => {
                    if logged_user == username.value() {
                        return DemoTemplate {
                            logged_in: true,
                            icon_name: icon_name.to_string(),
                            settings_sliders: settings_sliders,
                            settings_others: settings_others,
                            is_running: is_running,
                        }
                    }
                },
                None => {}
            }
        },
        None => {}
    }

    DemoTemplate {
        logged_in: false,
        icon_name: icon_name.to_string(),
        settings_sliders: settings_sliders,
        settings_others: settings_others,
        is_running: is_running,
    }
}

#[get("/maintenance")]
fn maintenance(mut cookies: Cookies, state: State<ServerState>) -> MaintenanceTemplate {
    let is_running = helper::script_controller::is_running();

    let (_, icon_name) = helper::script_controller::web::get_navbar_info();

    // TODO : get the maintenance settings

    match cookies.get_private("username") {
        Some(username) => {
            match &*state.logged_in_user.lock().unwrap() {
                Some(logged_user) => {
                    if logged_user == username.value() {
                        match cookies.get_private("usertype") {
                            Some(usertype) => {
                                if usertype.value() == "admin" {
                                    return MaintenanceTemplate {
                                        admin: true,
                                        logged_in: true,
                                        icon_name: icon_name.to_string(),
                                        is_running: is_running,
                                    }
                                } else {
                                    return MaintenanceTemplate {
                                        admin: false,
                                        logged_in: true,
                                        icon_name: icon_name.to_string(),
                                        is_running: is_running,
                                    }
                                }
                            },
                            None => {}
                        }
                    }
                },
                None => {}
            }
        },
        None => {}
    }

    MaintenanceTemplate {
        admin: true,
        logged_in: false,
        icon_name: icon_name.to_string(),
        is_running: is_running,
    }
}

#[get("/login?<error>")]
fn login_with_error(state: State<ServerState>, error: bool) -> LoginTemplate {
    match *state.logged_in_user.lock().unwrap() {
        Some(_) => {
            LoginTemplate {
                any_user_online: true,
                error: error,
            }
        },
        None => {
            LoginTemplate {
                any_user_online: false,
                error: error,
            }
        }
    }
}

#[get("/login")]
fn login(state: State<ServerState>) -> LoginTemplate {
    match *state.logged_in_user.lock().unwrap() {
        Some(_) => {
            LoginTemplate {
                any_user_online: true,
                error: false,
            }
        },
        None => {
            LoginTemplate {
                any_user_online: false,
                error: false,
            }
        }
    }
}

#[derive(FromForm)]
struct UserLogin {
    username: String,
    password: String,
}

#[post("/try_login", data = "<user_form>")]
fn login_form(mut cookies: Cookies, user_form: Form<UserLogin>, state: State<ServerState>) -> Redirect {
    match helper::passwords::check_password(&user_form.username, &user_form.password) {
        Ok(user_type) => {
            println!("usertype: {}", user_type);
            let mut user = state.logged_in_user.lock().unwrap();
            *user = Some(user_form.username.clone());
            cookies.add_private(Cookie::new("username", user_form.username.clone()));
            cookies.add_private(Cookie::new("usertype", user_type.clone()));
        },
        Err(_) => {
            return Redirect::to("/login?error=true");
        }
    }

    Redirect::to("/")
}

#[get("/logout")]
fn logout(mut cookies: Cookies, state: State<ServerState>) -> LogoutTemplate {
    let mut user = state.logged_in_user.lock().unwrap();
    *user = None;

    match cookies.get_private("username") {
        Some(username) => {
            cookies.remove_private(Cookie::named("username"));
            let mut name = username.to_string();
            name.replace_range(0..9, "");

            LogoutTemplate {
                logged_in: true,
                username: name,
            }
        },
        None => {
            LogoutTemplate {
                logged_in: false,
                username: "no name".to_string(),
            }
        }
    }
}

struct Item {
    fields: HashMap<String, String>,
}

impl<'f> FromForm<'f> for Item {
    type Error = ();

    fn from_form(items: &mut FormItems<'f>, _strict: bool) -> Result<Item, ()> {
        let mut fields = HashMap::new();

        for item in items {
            let decoded = item.value.url_decode().map_err(|_| ())?;
            fields.insert(item.key.as_str().to_string(), decoded);
        }

        Ok(Item { fields })
    }
}

#[post("/demo/send?", data="<form>")]
fn send(form: Form<Item>) -> Redirect{
    let socket = helper::script_controller::connect();

    for (variable_name, variable_value) in form.fields.iter() {
        let mut map = HashMap::new();
        map.insert("type", "set");
        map.insert("var", &variable_name);
        map.insert("value", &variable_value);
        helper::script_controller::send_message(&socket, map);

        match socket.recv_bytes(0) {
            Ok(value) => {
                println!("value: {:?}", value);
            },
            Err(_e) => {},
        }
    }

    Redirect::to("/demo")
}

fn rocket() -> rocket::Rocket {
    let server_state = ServerState {
        logged_in_user: Mutex::new(None),
    };

    rocket::ignite()
        .mount("/", StaticFiles::from("static"))
        .mount("/", routes![index, demo, maintenance, send, login, login_with_error, login_form, logout])
        .manage(server_state)
}

#[tokio::main]
async fn main() {
    // scan for devices
    let devices = bluetooth_serial_port::scan_devices(time::Duration::from_secs(5)).unwrap();
    if devices.len() == 0 {
        panic!("No devices found");
    }

    println!("Found bluetooth devices {:?}", devices);
    for device in devices {
        if device.name == "HC-05".to_string() {
            println!("Connecting to `{}` ({})", device.name, device.addr.to_string());

            let mut socket = BtSocket::new(BtProtocol::RFCOMM).unwrap();
            socket.connect(device.addr).unwrap();

            // BtSocket implements the `Read` and `Write` traits (they're blocking)
            let mut buffer = b"f";
            let mut buffer_read = [0 as u8; 5];

            let mut s = String::new();
            io::stdin().read_line(&mut s).expect("Couldn't read line");  

            let num_bytes_written = socket.write(buffer).unwrap();
            println!("wrote: {} bytes", num_bytes_written);
            let num_bytes_read = socket.read(&mut buffer_read).unwrap();
            println!("Read `{}` bytes", num_bytes_written);
        } else {
            println!("This is not the dongle you are looking for.");
        }
    }

    tokio::spawn(async move {
        let addr = "0.0.0.0:3012";
        let mut listener = TcpListener::bind(&addr).await.expect("Can't listen");

        while let Ok((stream, _)) = listener.accept().await {
            let peer = stream
                .peer_addr()
                .expect("connected streams should have a peer address");
            tokio::spawn(helper::websocket::accept_connection(peer, stream));
        }
    });
    
    rocket().launch();
}
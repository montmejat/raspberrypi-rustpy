#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

mod helper;

use rocket::response::Redirect;
use rocket::request::{FromForm, FormItems, Form};
use rocket_contrib::serve::StaticFiles;

use askama::Template;

use serde_json::json;

use tungstenite::Message::Text;
use tungstenite::server::accept;

use std::result::Result;
use std::collections::HashMap;
use std::net::TcpListener;
use std::thread::{spawn, sleep};
use std::time;

#[derive(Template)]
#[template(path = "demo.html")]
struct DemoTemplate {
    action: String,
    icon_name: String,
    settings_sliders: Vec<helper::script_controller::Slider>,
    settings_others: Vec<helper::script_controller::Variable>,
    is_running: bool,
}

#[derive(Template)]
#[template(path = "index.html")]
struct HomeTemplate {
    action: String,
    icon_name: String,
    cpu_temp: String,
    is_running: bool,
}

#[get("/pause")]
fn pause() -> Redirect {
    let socket = helper::script_controller::connect();
    helper::script_controller::pause(&socket);
    
    Redirect::to("/")
}

#[get("/demo/pause")]
fn demo_pause() -> Redirect {
    let socket = helper::script_controller::connect();
    helper::script_controller::pause(&socket);

    Redirect::to("/demo")
}

#[get("/unpause")]
fn unpause() -> Redirect {
    let socket = helper::script_controller::connect();
    helper::script_controller::unpause(&socket);
    
    Redirect::to("/")
}

#[get("/demo/unpause")]
fn demo_unpause() -> Redirect {
    let socket = helper::script_controller::connect();
    helper::script_controller::unpause(&socket);
    
    Redirect::to("/demo")
}

#[get("/")]
fn index() -> HomeTemplate {
    let is_running = helper::script_controller::is_running();

    let (action, icon_name) = helper::script_controller::web::get_navbar_info();
    let cpu_temp = helper::raspberry::get_cpu_temp();

    HomeTemplate {
        action: action.to_string(),
        icon_name: icon_name.to_string(),
        cpu_temp: cpu_temp.to_string(),
        is_running: is_running,
    }
}

#[get("/demo")]
fn demo() -> DemoTemplate {
    let is_running = helper::script_controller::is_running();

    let (action, icon_name) = helper::script_controller::web::get_navbar_info();
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

    DemoTemplate {
        action: action.to_string(),
        icon_name: icon_name.to_string(),
        settings_sliders: settings_sliders,
        settings_others: settings_others,
        is_running: is_running,
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
    spawn(move || {
        let server = TcpListener::bind("0.0.0.0:3012").unwrap();

        for stream in server.incoming() {
            spawn(move || {
                let mut websocket = accept(stream.unwrap()).unwrap();
                let page = websocket.read_message().unwrap();

                let mut is_pyscript_running_old = false;
                let mut is_pyscript_running;
                loop {
                    is_pyscript_running = helper::script_controller::is_running();
                    if is_pyscript_running != is_pyscript_running_old {
                        if is_pyscript_running {
                            websocket.write_message(Text("Python controller is online.".to_string())).unwrap()
                        } else {
                            websocket.write_message(Text("Python controller is offline.".to_string())).unwrap()
                        }
                    }

                    let (action, icon_name) = helper::script_controller::web::get_navbar_info();
                    let data;
                    if page.to_text().unwrap() == "index" {
                        data = json!({
                            "is_pyscript_running": is_pyscript_running,
                            "navbar": json!({
                                "action": action,
                                "icon_name": icon_name,
                            }),
                        });
                    } else {
                        data = json!({
                            "is_pyscript_running": is_pyscript_running,
                            "navbar": json!({
                                "action": action,
                                "icon_name": icon_name,
                            }),
                        });
                    }

                    match serde_json::to_string(&data) {
                        Ok(value) => websocket.write_message(Text(value)).unwrap(),
                        Err(_) => {}
                    }

                    sleep(time::Duration::from_millis(2000));
                    is_pyscript_running_old = is_pyscript_running;
                }
            });
        }
    });

    rocket::ignite()
        .mount("/", StaticFiles::from("static"))
        .mount("/", routes![index, demo, pause, demo_pause, unpause, demo_unpause, send])
}

fn main() {
    rocket().launch();
}
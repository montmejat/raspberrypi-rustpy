#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

mod helper;

use rocket::response::Redirect;
use rocket::request::{FromForm, FormItems, Form};
use rocket_contrib::serve::StaticFiles;
use askama::Template;
use std::result::Result;
use std::collections::HashMap;

#[derive(Template)]
#[template(path = "demo.html")]
struct DemoTemplate {
    action: String,
    icon_name: String,
    settings_sliders: Vec<helper::script_controller::Slider>,
    settings_others: Vec<helper::script_controller::Variable>,
}

#[derive(Template)]
#[template(path = "index.html")]
struct HomeTemplate {
    action: String,
    icon_name: String,
    cpu_temp: String,
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
    let socket = helper::script_controller::connect();
    let mut action = "";
    let mut icon_name = "";

    match helper::script_controller::get_state(&socket) {
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
    
    let cpu_temp = helper::raspberry::get_cpu_temp();

    HomeTemplate {
        action: action.to_string(),
        icon_name: icon_name.to_string(),
        cpu_temp: cpu_temp.to_string(),
    }
}

#[get("/demo")]
fn demo() -> DemoTemplate {
    let socket = helper::script_controller::connect();
    let mut action = "";
    let mut icon_name = "";

    match helper::script_controller::get_state(&socket) {
        Ok(value) => {
            match value.get("paused") {
                Some(paused) => {
                    if paused == "false" {
                        action = "demo/pause";
                        icon_name = "pause";
                    } else {
                        action = "demo/unpause";
                        icon_name = "play";
                    }
                },
                None => {}
            }
            
        },
        Err(_) => println!("Error retreiving state of controller...")
    }

    let mut settings_sliders = Vec::<helper::script_controller::Slider>::new();
    let mut settings_others = Vec::<helper::script_controller::Variable>::new();
    match helper::script_controller::get_settings(&socket) {
        Ok((sliders, others)) => {
            settings_sliders = sliders;
            settings_others = others;
        },
        Err(_) => println!("Error retreiving demo settings of controller...")
    }

    DemoTemplate {
        action: action.to_string(),
        icon_name: icon_name.to_string(),
        settings_sliders: settings_sliders,
        settings_others: settings_others,
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
    rocket::ignite()
        .mount("/", StaticFiles::from("static"))
        .mount("/", routes![index, demo, pause, demo_pause, unpause, demo_unpause, send])
}

fn main() {
    rocket().launch();
}
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

mod helper;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use rocket::response::Redirect;

use serde_cbor::to_vec;
use std::collections::HashMap;
use std::process::Command;
use std::str::from_utf8;

#[get("/pause")]
fn pause() -> Redirect {
    let socket = helper::connect();

    let mut data = HashMap::new();
    data.insert("type", "action");
    data.insert("value", "pause");
    let encoded = to_vec(&data);

    helper::send_message(&socket, &encoded.unwrap());

    Redirect::to("/")
}

#[get("/unpause")]
fn unpause() -> Redirect {
    let socket = helper::connect();

    let mut data = HashMap::new();
    data.insert("type", "action");
    data.insert("value", "unpause");
    let encoded = to_vec(&data);

    helper::send_message(&socket, &encoded.unwrap());
    
    Redirect::to("/")
}

#[get("/")]
fn index() -> Template {
    let socket = helper::connect();
    let mut context = HashMap::new();

    match helper::get_controller_state(&socket) {
        Ok(value) => {
            match value.get("paused") {
                Some(paused) => {
                    if paused == "false" {
                        context.insert("action", "pause");
                        context.insert("icon_name", "pause");
                    } else {
                        context.insert("action", "unpause");
                        context.insert("icon_name", "play");
                    }
                },
                None => {}
            }
            
        },
        Err(_) => println!("Error retreiving state of controller...")
    }
    
    let output = Command::new("sh").arg("-c").arg("/opt/vc/bin/vcgencmd measure_temp").output().expect("failed to execute process");
    let cpu_temp = output.stdout;
    let temp = from_utf8(&cpu_temp).unwrap().replace("temp=", "");
    context.insert("cpu_temp", &temp);

    Template::render("base", &context)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", StaticFiles::from("static")).mount("/", routes![index, pause, unpause]).attach(Template::fairing())
}

fn main() {
    rocket().launch();
}
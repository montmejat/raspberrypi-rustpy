#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

mod helper;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use rocket::response::Redirect;
use std::collections::HashMap;
use tera::Context;

#[get("/pause")]
fn pause() -> Redirect {
    let socket = helper::connect();
    helper::pause_controller(&socket);
    
    Redirect::to("/")
}

#[get("/demo/pause")]
fn demo_pause() -> Redirect {
    let socket = helper::connect();
    helper::pause_controller(&socket);

    Redirect::to("/demo")
}

#[get("/unpause")]
fn unpause() -> Redirect {
    let socket = helper::connect();
    helper::unpause_controller(&socket);
    
    Redirect::to("/")
}

#[get("/demo/unpause")]
fn demo_unpause() -> Redirect {
    let socket = helper::connect();
    helper::unpause_controller(&socket);
    
    Redirect::to("/demo")
}

#[get("/")]
fn index() -> Template {
    let socket = helper::connect();
    let mut context = HashMap::new();

    match helper::get_controller_state(&socket) {
        Ok(value) => {
            if value.get("paused").unwrap() == "false" {
                context.insert("action", "pause");
                context.insert("icon_name", "pause");
            } else {
                context.insert("action", "unpause");
                context.insert("icon_name", "play");
            }
        },
        Err(_) => println!("Error retreiving state of controller...")
    }
    
    let cpu_temp = helper::get_cpu_temp();
    context.insert("cpu_temp", &cpu_temp);
    Template::render("base", &context)
}

#[get("/demo")]
fn demo() -> Template {
    let socket = helper::connect();
    let mut context = Context::new();

    match helper::get_controller_state(&socket) {
        Ok(value) => {
            match value.get("paused") {
                Some(paused) => {
                    if paused == "false" {
                        context.insert("action", "demo/pause");
                        context.insert("icon_name", "pause");
                    } else {
                        context.insert("action", "demo/unpause");
                        context.insert("icon_name", "play");
                    }
                },
                None => {}
            }
            
        },
        Err(_) => println!("Error retreiving state of controller...")
    }

    let mut elements = Vec::new();
    match helper::get_controller_settings(&socket) {
        Ok(map) => {
            for (k, v) in map.iter() {
                elements.push(helper::to_html((k.to_string(), v.to_string())));
            }
        },
        Err(_) => println!("Error retreiving demo settings of controller...")
    }
    context.insert("elements", &elements);

    Template::render("demo", &context.into_json())
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", StaticFiles::from("static")).mount("/", routes![index, demo, pause, demo_pause, unpause, demo_unpause]).attach(Template::fairing())
}

fn main() {
    rocket().launch();
}
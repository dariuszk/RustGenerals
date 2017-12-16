extern crate piston_window;
#[macro_use] extern crate serde_derive;

use piston_window::*;
use std::fs::File;
use std::io::prelude::*;
mod config;


extern crate serde_json;

fn load_config() -> config::Config
{
    return  File::open("resources/config.json").ok().and_then(|mut f| {
        let mut s = String::new();
        f.read_to_string(&mut s).ok().and_then(|_| serde_json::from_str(&s).ok())
    }).unwrap_or_default();
}

fn main() {


    let config: config::Config = load_config();

    let mut window: PistonWindow = WindowSettings::new(config.title.clone(), (640, 480))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    while let Some(e) = window.next() {

        window.draw_2d(&e, |_c, g| {
            clear([0.5, 1.0, 0.5, 1.0], g);
        });
    }

}
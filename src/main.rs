
extern crate piston_window;
extern crate opengl_graphics;
#[macro_use] extern crate serde_derive;

use piston_window::*;
use std::fs::File;
use std::io::prelude::*;
use opengl_graphics::GlGraphics;

mod config;
mod view;
mod game_state;
mod elements;


extern crate serde_json;

fn load_config() -> config::Config
{
    return  File::open("resources/config.json").ok().and_then(|mut f| {
        let mut s = String::new();
        f.read_to_string(&mut s).ok().and_then(|_| serde_json::from_str(&s).ok())
    }).unwrap_or_default();
}

fn main() {

    let opengl = OpenGL::V3_2;

    let config: config::Config = load_config();

    let mut window: PistonWindow = WindowSettings::new(config.title.clone(), (640, 480))
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    let mut gl = GlGraphics::new(opengl);
    let mut state = game_state::State::new(config.board_size);
    while let Some(e) = window.next() {

        if let Some(ref args) = e.render_args() {

            gl.draw(args.viewport(), |c, g| view::render(c, g,  &state));
        }

    }

}
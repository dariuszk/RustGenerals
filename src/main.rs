extern crate piston_window;
extern crate opengl_graphics;
#[macro_use] extern crate serde_derive;
extern crate rand;
extern crate queue;

use piston_window::*;
use piston_window::Button::Keyboard;
use std::fs::File;
use std::io::prelude::*;
use opengl_graphics::GlGraphics;
use piston_window::ButtonState;

mod config;
mod view;
mod game_state;
mod elements;
mod controllers;
mod resources;

use controllers::{ InputController };
use controllers::StateInTimeController;
use resources::Resources;


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

    let mut window: PistonWindow = WindowSettings::new(config.title.clone(), (800, 600))
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    let mut gl = GlGraphics::new(opengl);
    let mut state = game_state::State::new(config.board_size);
    let mut input_controller = InputController::new();
    let mut state_controller = StateInTimeController::new();
    let mut resources = Resources::new();

    while let Some(e) = window.next() {

        match e {
            Event::Input(Input::Button(b_args)) =>
                {
                    use ButtonState::*;
                    match b_args.state {
                        Press => input_controller.key_press(b_args.button),
                        Release => input_controller.key_release(b_args.button)
                    }
                },
            Event::Loop( Loop::Update(update_args) ) =>
                {
                    state_controller.update(update_args.dt, &mut input_controller.actions(), &mut state );
                },
            _ => {}
        }

        if let Some(ref args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| view::render(c, g, &mut state, &mut resources));
        }



    }
}
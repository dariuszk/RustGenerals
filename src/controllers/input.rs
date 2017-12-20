

use piston_window::{ControllerButton, ControllerAxisArgs, Key, ButtonArgs, Button};
use elements::MoveDirection;
use std::vec;
use queue::Queue;

#[derive(Default)]
pub struct InputController {
    pub actions: Actions
}


pub struct Actions {
    pub go_up: bool,
    pub go_down: bool,
    pub go_left: bool,
    pub go_right: bool,
    pub actions_to_consume: Queue<MoveDirection>,
}


impl Default for Actions
{
    fn default() -> Actions
    {
        let mut q = Queue::new();
        Actions{ go_up:false
            , go_down:false
            , go_left:false
            , go_right:false
            , actions_to_consume:q }
    }
}


impl InputController {
    /// Create a new `InputController`
    pub fn new() -> InputController {
        InputController::default()
    }

    pub fn actions(&mut self) -> &mut Actions {
        &mut self.actions
    }

    pub fn key_press(&mut self, key: Button) {
        self.handle_key(key, true);
    }

    pub fn key_release(&mut self, key: Button) {
        self.handle_key(key, false);
    }

    pub fn handle_key(&mut self, key: Button, pressed : bool) {

        match key {
            Button::Keyboard(Key::Up)    => {
                self.actions.go_up = pressed;
                if pressed {
                    self.actions.actions_to_consume.queue(MoveDirection::Up).unwrap();
                }
            }
            Button::Keyboard(Key::Down)  => {
                self.actions.go_down = pressed;
                if pressed {
                    self.actions.actions_to_consume.queue(MoveDirection::Down).unwrap();
                }
            }
            Button::Keyboard(Key::Left)  => { 
                self.actions.go_left = pressed;
                if pressed {
                    self.actions.actions_to_consume.queue(MoveDirection::Left).unwrap();
                }
            }
            Button::Keyboard(Key::Right) => {
                self.actions.go_right = pressed;
                if pressed {
                    self.actions.actions_to_consume.queue(MoveDirection::Right).unwrap();
                }
            }
            _ => ()
        }
    }
}

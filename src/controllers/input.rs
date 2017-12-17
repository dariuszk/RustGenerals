use piston_window::{ControllerButton, ControllerAxisArgs, Key, ButtonArgs, Button};

#[derive(Default)]
pub struct InputController {
    actions: Actions
}


#[derive(Default)]
pub struct Actions {
    pub go_up: bool,
    pub go_down: bool,
    pub go_left: bool,
    pub gp_right: bool
}

impl InputController {
    /// Create a new `InputController`
    pub fn new() -> InputController {
        InputController::default()
    }

    pub fn actions(&self) -> &Actions {
        &self.actions
    }

    pub fn key_press(&mut self, key: Button) {
        self.handle_key(key, true);
    }


    pub fn key_release(&mut self, key: Button) {
        self.handle_key(key, false);
    }

    pub fn handle_key(&mut self, key: Button, pressed : bool) {

        match key {
            Button::Keyboard(Key::Left) => self.actions.go_up = pressed,
            Button::Keyboard(Key::Right) => self.actions.go_down = pressed,
            Button::Keyboard(Key::Up) => self.actions.go_left = pressed,
            Button::Keyboard(Key::Space) => self.actions.gp_right = pressed,
            _ => ()
        }
    }


}

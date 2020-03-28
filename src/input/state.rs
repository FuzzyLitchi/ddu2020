use std::hash::Hash;
use std::collections::HashMap;

use super::types::*;
use ggez_goodies::Point2;

#[derive(Debug, Copy, Clone, Default)]
struct ButtonState {
    // Current state
    down: bool,
    // Was pressed this frame
    pressed: bool,
    // Was released this frame
    released: bool,
}

#[derive(Debug, Clone)]
pub struct InputState<Buttons>
where
    Buttons: Hash + Eq + Clone,
{
    // Input states for buttons
    buttons: HashMap<Buttons, ButtonState>,
    mouse_position: Point2,
}

impl<Buttons> InputState<Buttons>
where
    Buttons: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        InputState {
            buttons: HashMap::new(),
            mouse_position: Point2::new(0.0, 0.0),
        }
    }

    /// Updates the logical input state based on the actual
    /// physical input state.  Should be called in the update()
    /// handler before the input dependent update.
    /// So, it will do things like move the axes and so on.
    pub fn update(&mut self) {
        for (_button, button_status) in self.buttons.iter_mut() {
            button_status.pressed = false;
            button_status.released = false;
        }
    }

    /// Takes an InputEffect and actually applies it.
    pub fn update_effect(&mut self, effect: InputEffect<Buttons>, started: bool) {
        match effect {
            InputEffect::Button(button) => {
                let f = || ButtonState::default();
                let button_status = self.buttons.entry(button).or_insert_with(f);
                button_status.down = started;

                match started {
                    true => button_status.pressed = true,
                    false => button_status.released = true,
                }
            }
        }
    }

    fn get_button(&self, button: Buttons) -> ButtonState {
        let d = ButtonState::default();
        let button_status = self.buttons.get(&button).unwrap_or(&d);
        *button_status
    }

    pub fn get_button_down(&self, axis: Buttons) -> bool {
        self.get_button(axis).down
    }

    pub fn get_button_up(&self, axis: Buttons) -> bool {
        !self.get_button(axis).down
    }

    /// Returns whether or not the button was pressed this frame,
    /// only returning true if the press happened this frame.
    ///
    /// Basically, `get_button_down()` and `get_button_up()` are level
    /// triggers, this and `get_button_released()` are edge triggered.
    pub fn get_button_pressed(&self, axis: Buttons) -> bool {
        self.get_button(axis).pressed
    }

    pub fn get_button_released(&self, axis: Buttons) -> bool {
        self.get_button(axis).released
    }

    pub fn update_mouse_position(&mut self, x: f32, y: f32) {
        self.mouse_position.x = x;
        self.mouse_position.y = y;
    }

    pub fn mouse_position(&self) -> Point2 {
        self.mouse_position
    }
}

// We implement Default so specs accepts this as a Read type in system data.
impl<Buttons> Default for InputState<Buttons>
where
    Buttons: Hash + Eq + Clone,
{
    fn default() -> Self {
        panic!("Input state is not supposed to be generated from default");
    }
}
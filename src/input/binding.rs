use ggez::event::*;
use std::collections::HashMap;
use std::hash::Hash;

use super::types::*;

pub struct InputBinding<Buttons>
where
    Buttons: Hash + Eq + Clone,
{
    bindings: HashMap<InputType, InputEffect<Buttons>>,
}

impl<Buttons> InputBinding<Buttons>
where
    Buttons: Hash + Eq + Clone,
{
    pub fn new() -> Self {
        InputBinding {
            bindings: HashMap::new(),
        }
    }

    /// Adds a key binding connecting the given keycode to the given
    /// logical button.
    pub fn bind_key_to_button(mut self, keycode: KeyCode, button: Buttons) -> Self {
        self.bindings.insert(
            InputType::KeyEvent(keycode),
            InputEffect::Button(button.clone()),
        );
        self
    }

    /// Adds a key binding connecting the given mouse button to the given
    /// logical button.
    pub fn bind_mouse_to_button(mut self, mouse_button: MouseButton, button: Buttons) -> Self {
        self.bindings.insert(
            InputType::MouseEvent(mouse_button),
            InputEffect::Button(button.clone()),
        );
        self
    }

    /// Takes an physical input type and turns it into a logical input type (keycode -> axis/button).
    pub fn resolve<T: Into<InputType>>(&self, input: T) -> Option<InputEffect<Buttons>> {
        self.bindings.get(&input.into()).cloned()
    }
}

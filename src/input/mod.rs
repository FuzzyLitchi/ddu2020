//! Typedefs for input shortcuts.
use ggez::event::*;
use ggez_goodies::Point2;

mod types;
mod binding;
mod state;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Button {
    Left,
    Right,
    Quit,
}

pub type Binding = binding::InputBinding<Button>;
pub type Event = types::InputEffect<Button>;
pub type State = state::InputState<Button>;

/// Create the default keybindings for our input state.
pub fn create_input_binding() -> binding::InputBinding<Button> {
    binding::InputBinding::new()
        .bind_mouse_to_button(MouseButton::Left, Button::Left)
        .bind_mouse_to_button(MouseButton::Right, Button::Right)
        .bind_key_to_button(KeyCode::Escape, Button::Quit)
}

pub struct MouseInput {
    pub pos: Point2,
    pub left: bool,
    pub right: bool,
}

impl Default for MouseInput {
    fn default() -> Self {
        MouseInput {
            pos: Point2::new(0.0, 0.0),
            left: false,
            right: false,
        }
    }
}
    
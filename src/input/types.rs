use ggez::event::*;
use std::hash::Hash;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum InputType {
    KeyEvent(KeyCode),
    MouseEvent(MouseButton)
}

impl From<KeyCode> for InputType {
    fn from(keycode: KeyCode) -> Self {
        InputType::KeyEvent(keycode)
    }
}

impl From<MouseButton> for InputType {
    fn from(mouse_button: MouseButton) -> Self {
        InputType::MouseEvent(mouse_button)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InputEffect<Buttons>
where
    Buttons: Hash + Eq + Clone,
{
    Button(Buttons),
}
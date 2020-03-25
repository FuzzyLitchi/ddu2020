use std::hash::Hash;
use std::collections::HashMap;

use super::types::*;

#[derive(Debug, Copy, Clone)]
enum AxisState {
    Positive,
    Neutral,
    Negative
}

impl Default for AxisState{
    fn default() -> Self {
        AxisState::Neutral
    }
}

impl AxisState {
    fn as_f32(self) -> f32 {
        match self {
            AxisState::Positive => 1.0,
            AxisState::Neutral => 0.0,
            AxisState::Negative => -1.0
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
struct ButtonState {
    pressed: bool,
    pressed_last_frame: bool,
}

#[derive(Debug)]
pub struct InputState<Axes, Buttons>
where
    Axes: Hash + Eq + Clone,
    Buttons: Hash + Eq + Clone,
{
    // Input state for axes
    axes: HashMap<Axes, AxisState>,
    // Input states for buttons
    buttons: HashMap<Buttons, ButtonState>,
}

impl<Axes, Buttons> InputState<Axes, Buttons>
where
    Axes: Eq + Hash + Clone,
    Buttons: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        InputState {
            axes: HashMap::new(),
            buttons: HashMap::new(),
        }
    }

    /// Updates the logical input state based on the actual
    /// physical input state.  Should be called in the update()
    /// handler before the input dependent update.
    /// So, it will do things like move the axes and so on.
    fn update(&mut self) {
        for (_button, button_status) in self.buttons.iter_mut() {
            button_status.pressed_last_frame = button_status.pressed;
        }
    }

    /// This method should get called by your key_down_event handler.
    pub fn update_button_down(&mut self, button: Buttons) {
        self.update_effect(InputEffect::Button(button), true);
    }

    /// This method should get called by your key_up_event handler.
    pub fn update_button_up(&mut self, button: Buttons) {
        self.update_effect(InputEffect::Button(button), false);
    }

    /// This method should get called by your key_up_event handler.
    pub fn update_axis_start(&mut self, axis: Axes, positive: bool) {
        self.update_effect(InputEffect::Axis(axis, positive), true);
    }

    pub fn update_axis_stop(&mut self, axis: Axes, positive: bool) {
        self.update_effect(InputEffect::Axis(axis, positive), false);
    }

    /// Takes an InputEffect and actually applies it.
    pub fn update_effect(&mut self, effect: InputEffect<Axes, Buttons>, started: bool) {
        match effect {
            InputEffect::Axis(axis, positive) => {
                unimplemented!();
            }
            InputEffect::Button(button) => {
                let f = || ButtonState::default();
                let button_status = self.buttons.entry(button).or_insert_with(f);
                button_status.pressed = started;
            }
        }
    }

    pub fn get_axis(&self, axis: Axes) -> f32 {
        let d = AxisState::default();
        let axis_status = self.axes.get(&axis).unwrap_or(&d);
        axis_status.as_f32()
    }

    fn get_button(&self, button: Buttons) -> ButtonState {
        let d = ButtonState::default();
        let button_status = self.buttons.get(&button).unwrap_or(&d);
        *button_status
    }

    pub fn get_button_down(&self, axis: Buttons) -> bool {
        self.get_button(axis).pressed
    }

    pub fn get_button_up(&self, axis: Buttons) -> bool {
        !self.get_button(axis).pressed
    }

    /// Returns whether or not the button was pressed this frame,
    /// only returning true if the press happened this frame.
    ///
    /// Basically, `get_button_down()` and `get_button_up()` are level
    /// triggers, this and `get_button_released()` are edge triggered.
    pub fn get_button_pressed(&self, axis: Buttons) -> bool {
        let b = self.get_button(axis);
        b.pressed && !b.pressed_last_frame
    }

    pub fn get_button_released(&self, axis: Buttons) -> bool {
        let b = self.get_button(axis);
        !b.pressed && b.pressed_last_frame
    }

    pub fn mouse_position() {
        unimplemented!()
    }

    pub fn reset_input_state(&mut self) {
        for (_axis, axis_status) in self.axes.iter_mut() {
            *axis_status = AxisState::Neutral;
        }

        for (_button, button_status) in self.buttons.iter_mut() {
            button_status.pressed = false;
            button_status.pressed_last_frame = false;
        }
    }
}
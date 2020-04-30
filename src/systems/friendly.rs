use crate::components::*;
use crate::resources::*;
use crate::input;

use specs::{self, Join};
use ggez_goodies::Vector2;

pub struct FriendlySystem;

impl<'a> specs::System<'a> for FriendlySystem {
    type SystemData = (
        specs::ReadStorage<'a, Position>,
        specs::WriteStorage<'a, Motion>,
        specs::WriteStorage<'a, Friendly>,
        specs::Read<'a, input::State>,
        specs::Write<'a, Option<SelectionBox>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            pos,
            mut motion,
            mut friendly,
            input,
            mut selection_box
        ) = data;

        // Selection box
        if input.get_button_pressed(input::Button::Left) {
            let selection_box = selection_box.get_or_insert(SelectionBox::new());
            selection_box.start = input.mouse_position()
        }

        if input.get_button_down(input::Button::Left) {
            let selection_box = selection_box.get_or_insert(SelectionBox::new());
            selection_box.stop = input.mouse_position()
        }

        if input.get_button_released(input::Button::Left) {
            if let Some(sel_box) = &*selection_box {   
                // Order x and y positions.
                let x1 = sel_box.start.x.min(sel_box.stop.x);
                let x2 = sel_box.start.x.max(sel_box.stop.x);
                let y1 = sel_box.start.y.min(sel_box.stop.y);
                let y2 = sel_box.start.y.max(sel_box.stop.y);
                
                for (pos, friendly) in (&pos, &mut friendly).join() {
                    // If in bounding box, select
                    friendly.selected = pos.0.x > x1 && pos.0.x < x2 && pos.0.y > y1 && pos.0.y < y2;
                }
            } else {
                // Maybe use log instead
                panic!("Button released but no selection box?!");
            }

            *selection_box = None;
        }
        
        // Make selected friendlies go to right click
        if input.get_button_pressed(input::Button::Right) {
            for friendly in (&mut friendly).join() {
                if friendly.selected {
                    friendly.action = Action::Goto(input.mouse_position());
                }
            }
        }

        // TODO: I could optimize this code to only run when Action::Goto changes or collisions and stuff
        for (pos, motion, friendly) in (&pos, &mut motion, &mut friendly).join() {
            match friendly.action {
                Action::Goto(target_pos) => {
                    let vector = target_pos - pos.0;

                    // If we're close enough, stop.
                    pub const DISTANCE_BEFORE_STOP: f32 = 4.0;
                    if vector.length() < DISTANCE_BEFORE_STOP {
                        friendly.action = Action::Standby;
                        motion.velocity = Vector2::zero();
                        continue;
                    }

                    let direction = vector.normalize();

                    motion.velocity = direction * WALK_SPEED;
                }
                _ => (),
            }
        }
    }
}
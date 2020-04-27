use specs::World;
use ggez_goodies::Point2;

pub struct SelectionBox {
    pub start: Point2,
    pub stop: Point2
}

impl SelectionBox {
    pub fn new() -> Self {
        SelectionBox {
            start: Point2::zero(),
            stop: Point2::zero(),
        }
    }
}

// Since Default for Option is None, this works without needing register_resourcesR
/*
pub fn register_resources(specs_world: &mut World) {
    specs_world.insert::<Option<SelectionBox>>(None);
}
*/

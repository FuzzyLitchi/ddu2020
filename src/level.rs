use std::path::{Path, PathBuf};
use std::fs::File;

use specs::{world::Builder, World, WorldExt};
use ggez_goodies::Point2;
use image;
use collider::geom::*;

use crate::physics::add_box_collider;
use crate::components::*;

pub fn load_level<P: AsRef<Path>>(level: &P, world: &mut World) {
    // Locate
    let mut path = PathBuf::from("resources/rooms");
    path.push(level);

    // Read png file
    let image = image::open(path).expect("No such level").to_rgba();
    
    // enumerat pixels
    for (x, y, pixel) in image.enumerate_pixels() {        
        match pixel.0 {
            // Wall
            [0, 0, 0, 255] => {
                let (x, y) = (x as f64 * 32.0, y as f64 * 32.0);

                let entity = world.create_entity()
                    .with(Position(Point2::new(x as f32, y as f32)))
                    .with(Renderable::Rectangle {
                        w: 32.0,
                        h: 32.0,
                        color: ggez::graphics::Color::new(0.25, 0.5, 0.25, 1.0)
                    })
                    .build();

                let hitbox = Shape::square(32.0).place(v2(x, y)).still();
                
                add_box_collider(entity, hitbox, true, world);
            }
            _ => (),
        }
    }
}



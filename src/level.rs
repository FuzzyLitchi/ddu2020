use std::path::{Path, PathBuf};
use std::fs::File;

use specs::{world::Builder, World, WorldExt};
use ggez_goodies::{Point2, Vector2};
use image;

use crate::{components};

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
                world.create_entity()
                    .with(components::Position(Point2::new(x as f32 * 32.0, y as f32 * 32.0)))
                    .with(components::Renderable::Rectangle {
                        w: 32.0,
                        h: 32.0,
                        color: ggez::graphics::Color::new(0.25, 0.5, 0.25, 1.0)
                    })
                    .build();
            }
            _ => (),
        }
    }
}



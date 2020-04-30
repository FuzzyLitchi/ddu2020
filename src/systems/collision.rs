use crate::components::*;
use crate::physics::*;

use specs::{self, Join, World};
use collider::{Collider, HbEvent, HbProfile, HbVel, geom::v2};

// Length of one tick
const TICK: f64 = 1.0/60.0;
pub struct CollisionSystem {
    tick: u64,
}

impl CollisionSystem {
    pub fn new() -> Self {
        CollisionSystem {
            tick: 0,
        }
    }
}

impl<'a> specs::System<'a> for CollisionSystem {
    type SystemData = (
        specs::Write<'a, Option<ColliderRes>>,
        specs::WriteStorage<'a, Position>,
        specs::WriteStorage<'a, Motion>,
        specs::ReadStorage<'a, BoxCollider>,
        specs::ReadStorage<'a, Friendly>,
    );

    fn run(&mut self, (mut collider, mut pos, mut motion, box_collider, _friendly): Self::SystemData) {
        let collider = collider.as_mut().unwrap();

        // Fix motion
        for (motion, box_collider) in (&motion, &box_collider).join() {
            let vel = HbVel::moving(v2(motion.velocity.x as f64, motion.velocity.y as f64));

            collider.set_hitbox_vel(box_collider.0, vel);
        }

        self.tick += 1;
        let end_time = self.tick as f64 * TICK;

        // Advance simulation to end of tick
        while collider.time() < end_time {
            let time = collider.next_time().min(end_time);
            collider.set_time(time);

            for (event, profile_1, profile_2) in collider.next() {
                println!("{:?} between {:?} and {:?} at time {}.",
                            event, profile_1, profile_2, collider.time());

                if event == HbEvent::Collide {
                    match (profile_1.wall, profile_2.wall) {
                        (false, false) => {
                            // Entity on entity collision
                        }
                        (true, false) | (false, true) => {
                            // Entity on wall collision
                            let entity;
                            let wall;

                            // Figure out profile is which
                            if profile_1.wall {
                                wall = profile_1;
                                entity = profile_2;
                            } else {
                                wall = profile_2;
                                entity = profile_1;
                            }

                            let wall_hb = collider.get_hitbox(wall.id());
                            let entity_hb = collider.get_hitbox(entity.id());

                            // The normal is applied to the entity to make it no longer colide with the wall.
                            let normal = entity_hb.value.normal_from(&wall_hb.value);

                            let new_pos = entity_hb.value.pos + normal.dir()*normal.len();
                            let new_vec = {
                                let a = entity_hb.vel.value;
                                let b = v2(normal.dir().y, -normal.dir().x);

                                let dot_product = a.x * b.x + a.y * b.y;
                                
                                dot_product * b
                            };
                            println!("{:?}", new_vec);

                            collider.remove_hitbox(entity.id());
                            collider.add_hitbox(entity, entity_hb.value.shape.place(new_pos).moving(new_vec));
                        }
                        _ => ()
                    }
                }
            }
        }

        // Update objects
        for (mut pos, mut motion, box_collider) in (&mut pos, (&mut motion).maybe(), &box_collider).join() {
            let hb = collider.get_hitbox(box_collider.0);

            // Update pos
            let v = hb.value.pos;
            pos.0.x = v.x as f32;
            pos.0.y = v.y as f32;

            if let Some(motion) = &mut motion {
                let v = hb.vel.value;
                motion.velocity.x = v.x as f32;
                motion.velocity.y = v.y as f32;
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        // Creates a new empty collider world
        let collider: ColliderRes = Collider::new();

        world.insert(Some(collider));
    }
}

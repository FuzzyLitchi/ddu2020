use std::collections::VecDeque;

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

        // Update motion from specs to collider world
        for (motion, box_collider) in (&motion, &box_collider).join() {
            let vel = HbVel::moving(v2(motion.velocity.x as f64, motion.velocity.y as f64));

            collider.set_hitbox_vel(box_collider.0, vel);
        }

        self.tick += 1;
        let end_time = self.tick as f64 * TICK;

        // Advance simulation to end of tick
        while collider.time() < end_time {
            // Advance simulation to next collision or end of tick
            let time = collider.next_time().min(end_time);
            collider.set_time(time);

            // Collect collisions if there are any.
            let mut collisions: VecDeque<(Profile, Profile)> = VecDeque::new();
            for (e, profile_1, profile_2) in collider.next() {
                if e == HbEvent::Collide {
                    collisions.push_back((profile_1, profile_2));
                }
            }

            // Handle all collisions
            while let Some((profile_1, profile_2)) = collisions.pop_front() {
                // Skip wall to wall collisions
                if profile_1.wall && profile_2.wall {
                    continue;
                }
                // Skip if collision is no longer relevant
                if !collider.is_overlapping(profile_1.id(), profile_2.id()) {
                    continue;
                }

                if profile_1.wall {
                    let new_collisions = handle_wall_collision(profile_2, profile_1, collider);

                    for other in new_collisions {
                        collisions.push_back((profile_2, other));
                    }
                } else if profile_2.wall {
                    let new_collisions = handle_wall_collision(profile_1, profile_2, collider);

                    for other in new_collisions {
                        collisions.push_back((profile_1, other));
                    }
                } else {
                    let hb_1 = collider.get_hitbox(profile_1.id());
                    let hb_2 = collider.get_hitbox(profile_2.id());

                    let _ = collider.remove_hitbox(profile_1.id());
                    let _ = collider.remove_hitbox(profile_2.id());

                    let (hb_1, hb_2) = resolve_entity_collision(hb_1, hb_2);

                    collider.add_hitbox(profile_1, hb_1);
                    collider.add_hitbox(profile_2, hb_2);
                    
                    let new_collisions = collider.get_overlaps(profile_1.id());
                    
                    for other in new_collisions {
                        collisions.push_back((profile_1, other));
                    }

                    let new_collisions = collider.get_overlaps(profile_2.id());

                    for other in new_collisions {
                        collisions.push_back((profile_2, other));
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

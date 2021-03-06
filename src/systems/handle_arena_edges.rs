use bbecs::components::point::Point;
use bbecs::world::World;

use crate::component_names::ComponentNames;
use crate::resource_names::ResourceNames;

pub fn handle_arena_edges_system(world: &World<ComponentNames, ResourceNames>) {
    let borrowed_arena_size = world.get_resource(&ResourceNames::ArenaSize).borrow();
    let arena_size = borrowed_arena_size.cast_point();
    let locations = world.query_one(&ComponentNames::Location).borrow();
    let velocities = world.query_one(&ComponentNames::Velocity).borrow();
    let mut accelerations = world.query_one(&ComponentNames::Acceleration).borrow_mut();
    let sight_range = world
        .get_resource(&ResourceNames::SightRange)
        .borrow()
        .cast_f32();

    locations.iter().enumerate().for_each(|(index, location)| {
        let location = location.cast_point();
        let velocity = velocities[index].cast_point();
        let acceleration = accelerations[index].cast_point_mut();

        let mut force = Point::default();
        let mut wall_dist = 0.0_f32;

        if location.x > arena_size.x - sight_range {
            if velocity.y >= 0.0 {
                // We are going to turn right
                force = velocity.to_perpendicular_right();
            } else {
                // We are going to turn left
                force = velocity.to_perpendicular_left();
            }
            wall_dist = arena_size.x - location.x;
        } else if location.x < sight_range {
            if velocity.y >= 0.0 {
                // We are going to turn left because it will be faster to avoid the wall
                force = velocity.to_perpendicular_left();
            } else {
                // We are going to turn right
                force = velocity.to_perpendicular_right();
            }
            wall_dist = location.x;
        }

        if location.y < sight_range {
            if velocity.x >= 0.0 {
                force = velocity.to_perpendicular_right();
            } else {
                force = velocity.to_perpendicular_left();
            }
            wall_dist = location.y;
        } else if location.y > arena_size.y - sight_range {
            if velocity.x >= 0.0 {
                force = velocity.to_perpendicular_left();
            } else {
                force = velocity.to_perpendicular_right();
            }
            wall_dist = arena_size.y - location.y;
        }
        force.normalize();
        force.multiply_scalar(3.0 * (1.0 - wall_dist/sight_range));
        acceleration.add(&force);
    });
}

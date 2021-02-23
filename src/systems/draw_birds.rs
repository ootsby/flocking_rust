use bbecs::world::World;
use ggez::{graphics, Context, GameResult};
use graphics::DrawParam;

use crate::component_names::ComponentNames;
use crate::resource_names::ResourceNames;

/// Query for the locations and then draw them out using GGEZ's draw method
pub fn draw_birds_system(
    context: &mut Context,
    world: &World<ComponentNames, ResourceNames>,
) -> GameResult {
    let borrowed_mesh = world.get_resource(&ResourceNames::BirdMesh).borrow();
    let mesh = borrowed_mesh.cast_mesh();
    let locations = world.query_one(&ComponentNames::Location).borrow();
    let velocities = world.query_one(&ComponentNames::Velocity).borrow();

    locations.iter().enumerate().try_for_each(|(index, component)| {
        let location = component.cast_point();
        let direction = velocities[index].cast_point();
        graphics::draw(
            context,
            mesh,
            DrawParam::default().rotation(direction.y.atan2(direction.x)).dest(location.to_array()),
        )
    })
}

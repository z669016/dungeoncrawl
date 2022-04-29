use crate::prelude::*;
use crate::systems::random_direction::random_direction;

#[system]
#[write_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_move(ecs: &mut SubWorld, #[resource] map: &Map) {
    let mut movers = <(&mut Point, &MovingRandomly)>::query();
    movers.iter_mut(ecs)
        .for_each(|(pos,_)| {
            let destinaton = random_direction() + *pos;
            if map.can_enter_tile(destinaton) {
                *pos = destinaton;
            }
        });
}
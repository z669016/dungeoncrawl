mod player_input;
mod map_render;
mod entity_render;
mod collitions;
mod random_move;
mod random_direction;

use crate::prelude::*;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(collitions::collitions_system())
        .flush()
        .add_system(random_move::random_move_system())
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .build()
}
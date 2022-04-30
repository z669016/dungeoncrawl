extern crate core;

mod map;
mod map_builder;
mod camera;
mod components;
mod spawner;
mod systems;
mod turn_state;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;

    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;

    pub const MAP_LAYER: usize = 0;
    pub const PLAYER_LAYER: usize = 1;

    pub const MAP_ORDER: usize = 0;
    pub const ENTITY_ORDER: usize = 5000;

    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::turn_state::*;
    pub use crate::systems::*;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let builder = MapBuilder::new(&mut rng);

        let mut camera = Camera::new(builder.player_start);
        camera.adjust();

        resources.insert(builder.map);
        resources.insert(camera);
        resources.insert(TurnState::AwaitingInput);

        spawn_player(&mut ecs, builder.player_start);

        // Spawn a monster in every room except for the first room
        // as the first room already contains the player (in the center)
        builder.rooms.iter()
            .skip(1)
            .map(|room| room.center())
            .for_each(|room_center| {
                spawn_enemies(&mut ecs, &mut rng, room_center)
            });

        Self {
            ecs,
            resources,
            input_systems: build_imput_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(MAP_LAYER);
        ctx.cls();
        ctx.set_active_console(PLAYER_LAYER);
        ctx.cls();

        self.resources.insert(ctx.key);

        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        match current_state {
            TurnState::AwaitingInput => {
                self.input_systems.execute(&mut self.ecs, &mut self.resources);
            },
            TurnState::PlayerTurn => {
                self.player_systems.execute(&mut self.ecs, &mut self.resources);
            },
            TurnState::MonsterTurn => {
                self.monster_systems.execute(&mut self.ecs, &mut self.resources);
            }
        }
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .build()?;

    main_loop(context, State::new())
}

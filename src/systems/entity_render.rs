use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &mut SubWorld, #[resource] camera: &mut Camera) {

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(PLAYER_LAYER);

    let offset = Point::new(camera.left_x, camera.top_y);

    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(pos, render)| {
            draw_batch.set(*pos - offset, render.color, render.glyph);
        });

    draw_batch.submit(ENTITY_ORDER).expect("Batch error entity_render");
}

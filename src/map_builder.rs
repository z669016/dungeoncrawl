use crate::prelude::*;
use std::cmp::{min, max};

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut builder = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            player_start: Point::zero(),
        };

        builder.fill();
        builder.build_random_rooms(rng);
        builder.player_start = builder.rooms[0].center();
        builder.build_corridors(rng);

        builder
    }

    fn fill(&mut self) {
        self.map.tiles.iter_mut().for_each(|t| *t = TileType::Wall);
    }

    fn clear(&mut self, room: Rect) {
        room.for_each(|point| {
            if Map::in_bounds(point) {
                self.map.tiles[Map::map_idx(point.x, point.y)] = TileType::Floor;
            }
        });
    }

    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            let new_room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );

            if let Some(_) = self.rooms.iter().find(|&room| room.intersect(&new_room)) {
                // Ignore if the new room has overlap with an existing room
            } else {
                self.clear(new_room);
                self.rooms.push(new_room);
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let point = Point::new(x, y);
            if let Some(idx) = Map::try_idx(point) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let point = Point::new(x, y);
            if let Some(idx) = Map::try_idx(point) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev_center = rooms[i - 1].center();
            let curr_center = room.center();

            if rng.range(0, 2) == 1 {
                self.apply_horizontal_tunnel(prev_center.x, curr_center.x, prev_center.y);
                self.apply_vertical_tunnel(prev_center.y, curr_center.y, curr_center.x);
            } else {
                self.apply_vertical_tunnel(prev_center.y, curr_center.y, prev_center.x);
                self.apply_horizontal_tunnel(prev_center.x, curr_center.x, curr_center.y);
            }
        }
    }
}
use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn map_idx(x: i32, y: i32) -> usize {
        ((y * SCREEN_WIDTH) + x) as usize
    }

    pub fn in_bounds(point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH &&
            point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn try_idx(point: Point) -> Option<usize> {
        if !Map::in_bounds(point) {
            None
        } else {
            Some(Map::map_idx(point.x, point.y))
        }
    }

    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = Map::map_idx(x, y);
                match self.tiles[idx] {
                    TileType::Floor => {
                        ctx.set(x, y, YELLOW, BLACK, to_cp437('.'));
                    }
                    TileType::Wall => {
                        ctx.set(x, y, GREEN, BLACK, to_cp437('#'));
                    }
                }

                // if y == 0 {
                //     ctx.set(x, y, NAVY, YELLOW, to_cp437( std::char::from_digit ((x % 10) as u32, 10).unwrap()));
                // }
                // if x == 0 {
                //     ctx.set(x, y, NAVY, YELLOW, to_cp437( std::char::from_digit ((y % 10) as u32, 10).unwrap()));
                // }
            }
        }
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        Map::in_bounds(point) &&
            self.tiles[Map::map_idx(point.x, point.y)] == TileType::Floor
    }
}


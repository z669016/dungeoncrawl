use crate::prelude::*;

pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

impl Camera {
    pub fn new(player_position: Point) -> Self {
        Self {
            left_x: player_position.x - DISPLAY_WIDTH / 2,
            right_x: player_position.x + DISPLAY_WIDTH / 2,
            top_y: player_position.y - DISPLAY_HEIGHT / 2,
            bottom_y: player_position.y + DISPLAY_HEIGHT / 2,
        }
    }

    pub fn adjust(&mut self) {
        // When too much to the left (right side of the screen is off the board
        while self.right_x >= SCREEN_WIDTH {
            self.left_x -= 1;
            self.right_x -= 1;
        }

        // When too much to the right (left side of the screen is off the board
        while self.left_x < 0 {
            self.left_x += 1;
            self.right_x += 1;
        }

        // When too much up (right side of the screen is off the board
        while self.bottom_y >= SCREEN_HEIGHT {
            self.top_y -= 1;
            self.bottom_y -= 1;
        }

        // When too much down (left side of the screen is off the board
        while self.top_y < 0 {
            self.top_y += 1;
            self.bottom_y += 1;
        }
    }

    pub fn on_player_move(&mut self, player_position: Point) {
        self.left_x = player_position.x - DISPLAY_WIDTH / 2;
        self.right_x = player_position.x + DISPLAY_WIDTH / 2;
        self.top_y = player_position.y - DISPLAY_HEIGHT / 2;
        self.bottom_y = player_position.y + DISPLAY_HEIGHT / 2;
    }
}
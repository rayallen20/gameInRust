use bracket_lib::color::{BLACK, YELLOW};
use bracket_lib::prelude::BTerm;
use crate::game::SCREEN_HEIGHT;

pub struct Player {
    // 玩家的世界坐标x
    pub world_x: i32,
    // 玩家的屏幕坐标y
    pub screen_y: i32,
    // 玩家的速度
    pub velocity: f32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            world_x: 0,
            screen_y: SCREEN_HEIGHT / 2,
            velocity: 0.0,
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set(0, self.screen_y, YELLOW, BLACK, '@');
    }

    pub fn gravity(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
    }

    pub fn flap(&mut self) {
        self.velocity = -2.0;
        if self.velocity < -2.0 {
            self.velocity = -2.0;
        }
    }

    pub fn shift(&mut self) {
        self.world_x += 1;
        self.screen_y += self.velocity as i32;
        if self.screen_y < 0 {
            self.screen_y = 0;
        }
    }
}
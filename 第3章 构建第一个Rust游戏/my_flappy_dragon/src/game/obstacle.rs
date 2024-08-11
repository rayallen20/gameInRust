use bracket_lib::color::{BLACK, RED};
use bracket_lib::prelude::{BTerm, RandomNumberGenerator};
use crate::game::{Player, SCREEN_WIDTH};

pub struct Obstacle {
    pub world_x: i32,
    pub screen_x: i32,
    pub gap_center_screen_y: i32,
    pub gap_size: i32,
}

impl Obstacle {
    pub fn new(world_x: i32, player_world_x: i32, score: i32) -> Obstacle {
        let mut random = RandomNumberGenerator::new();

        Obstacle {
            world_x,
            screen_x: world_x - player_world_x,
            gap_center_screen_y: random.range(10, 40),
            gap_size: i32::max(2, 20 - score),
        }
    }

    pub fn render(&mut self, ctx: &mut BTerm, player_world_x: i32) {
        let half_size = self.gap_size / 2;

        // 渲染障碍物前重新计算障碍物的屏幕坐标x
        self.screen_x = self.world_x - player_world_x;

        for y in 0 .. self.gap_center_screen_y - half_size {
            ctx.set(self.screen_x, y, RED, BLACK, '|');
        }

        for y in self.gap_center_screen_y + half_size .. SCREEN_WIDTH {
            ctx.set(self.screen_x, y, RED, BLACK, '|');
        }
    }

    pub fn hit(&self, player: &Player) -> bool {
        let half_size = self.gap_size/ 2;

        let does_x_match = player.world_x == self.world_x;
        let does_above_gap = player.screen_y < self.gap_center_screen_y - half_size;
        let does_bellow_gap = player.screen_y > self.gap_center_screen_y + half_size;

        does_x_match && (does_above_gap || does_bellow_gap)
    }
}
use bracket_lib::color::NAVY;
use bracket_lib::prelude::{BTerm, GameState, VirtualKeyCode};
use crate::game::{GameMode, Obstacle, Player};

pub const SCREEN_HEIGHT: i32 = 50;
pub const SCREEN_WIDTH: i32 = 80;
pub const FRAME_DURATION: f32 = 75.0;

pub struct State {
    game_mode: GameMode,
    player: Player,
    obstacle: Obstacle,
    frame_time: f32,
    score: i32,
}

impl State {
    pub fn new() -> Self {
        let player = Player::new();
        let player_world_x = player.world_x;

        State {
            game_mode: GameMode::Menu,
            player,
            obstacle: Obstacle::new(SCREEN_WIDTH, player_world_x, 0),
            frame_time: 0.0,
            score: 0,
        }
    }

    pub fn show_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.game_mode = GameMode::Playing,
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {},
            }
        }
    }

    pub fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);

        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.player.gravity();
            self.player.shift();
            self.frame_time = 0.0;
        }

        if let Some(key) = ctx.key {
            match key {
                // Tips: 这里只修改速度 不需要修改坐标 因为坐标计算只按每75ms计算一次即可
                // Tips: 否则感官上按空格向上移动的频次就太高了
                VirtualKeyCode::Space => self.player.flap(),
                _ => {},
            }
        }

        ctx.print(0, 0, format!("Velocity: {}", self.player.velocity));
        ctx.print(0, 2, format!("Score: {}", self.score));
        ctx.print(0, 3, format!("FPS: {}", ctx.fps));

        self.player.render(ctx);
        self.obstacle.render(ctx, self.player.world_x);

        // 玩家成功穿越障碍物 则得分+1 并生成新的障碍物
        if self.player.world_x > self.obstacle.world_x {
            self.score += 1;
            self.obstacle = Obstacle::new(SCREEN_WIDTH + self.player.world_x, self.player.world_x, self.score);
        }

        if self.player.screen_y > SCREEN_HEIGHT || self.obstacle.hit(&self.player) {
            self.game_mode = GameMode::Dead;
        }
    }

    pub fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");
        ctx.print_centered(10, format!("Score: {}", self.score));

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {},
            }
        }
    }

    pub fn restart(&mut self) {
        self.player = Player::new();
        self.obstacle = Obstacle::new(SCREEN_WIDTH, self.player.world_x, 0);
        self.frame_time = 0.0;
        self.score = 0;
        self.game_mode = GameMode::Playing;
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.game_mode {
            GameMode::Menu => self.show_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::Dead => self.dead(ctx),
        }
    }
}
use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Playing,
    End,
}

// 屏幕宽度
const SCREEN_WIDTH: i32 = 80;

// 屏幕高度
const SCREEN_HEIGHT: i32 = 50;

// 屏幕刷新率 单位: 毫秒 即:每75ms重新渲染一次屏幕
const FRAME_DURATION: f32 = 75.0;

struct State {
    player: Player,
    // 帧与帧之间的时间间隔 以毫秒为单位
    frame_time: f32,
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        State {
            player: Player::new(5, 25),
            frame_time: 0.0,
            // 初始状态为菜单
            mode: GameMode::Menu,
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        // 清空屏幕 打印欢迎菜单
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.mode = GameMode::Playing,
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        // 清空屏幕
        ctx.cls_bg(NAVY);

        // 计算玩家受重力影响在位置上的改变
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.player.gravity_and_move();
            self.frame_time = 0.0;
        }

        // 计算玩家主动扇动翅膀在位置上的改变
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        // 渲染玩家
        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap.");
        ctx.print(0, 1, &format!("Frame Time: {}", ctx.frame_time_ms));
        ctx.print(0, 2, &format!("FPS: {}", ctx.fps));
        ctx.print(0, 3, &format!("Speed: {}", self.player.velocity));

        // 判断游戏是否结束
        if self.player.y > SCREEN_HEIGHT {
            self.mode = GameMode::End;
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        // 清空屏幕 打印结束菜单
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.mode = GameMode::Playing,
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn restart(&mut self, ctx: &mut BTerm) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

struct Player {
    // 玩家当前的水平位置
    x: i32,
    // 玩家当前的垂直位置
    y: i32,
    // 玩家在垂直方向上的速度
    velocity: f32,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(
            0,
            self.y,
            YELLOW,
            BLACK,
            to_cp437('@')
        );
    }

    fn gravity_and_move(&mut self) {
        // 若当前下坠速度小于2.0 则增加0.2
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }

        // 向下移动
        self.y += self.velocity as i32;

        // 水平方向前进1px
        self.x += 1;

        // 若玩家在垂直方向上到达屏幕顶部 则将其固定在顶部
        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        // 玩家向上飞行
        self.velocity = -2.0;
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50().
        with_title("Flappy Dragon").build()?;
    main_loop(context, State::new())
}

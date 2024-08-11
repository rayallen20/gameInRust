# 3.3 添加游戏角色

```rust
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
}
```

- 使用整型表示位置是因为我们的游戏是一个像素级的游戏,而像素是整数
- 使用浮点数表示速度是因为速度是一个连续的值,而不是一个离散的值.使得速度的变化更加平滑

## 3.3.1 渲染游戏角色

```rust
impl Player {
    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(
            0,
            self.y,
            YELLOW,
            BLACK,
            to_cp437('@')
        );
    }
}
```

- `BTerm.set()`: 设置一个像素的颜色和字符
- `0, self.y`: 设置被渲染像素的位置
- `YELLOW, BLACK`: 设置被渲染像素的颜色.其中`YELLOW`表示前景色,即字符的颜色;`BLACK`表示字符的背景色,即这个像素中字符的背景颜色
- `to_cp437('@')`: 设置被渲染像素的字符.这里使用`to_cp437()`函数将字符`@`转换为`cp437`字符集中的字符

## 3.3.2 坠向不可避免的死亡

```rust
impl Player {
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
}
```

- 写游戏时,函数的调用都是以帧为单位的.每一帧都会调用该函数,因此玩家的下坠速度会在每一帧中增加0.2
- 在本函数中,下坠在垂直方向上对位置的影响也实现了

## 3.3.3 扇动翅膀

```rust
impl Player {
    fn flap(&mut self) {
        // 玩家向上飞行
        self.velocity = -2.0;
    }
}
```

## 3.3.4 实例化玩家

```rust
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
    
    fn restart(&mut self, ctx: &mut BTerm) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
    }
}
```

这里要解释`frame_time`这个成员属性.它的作用是控制帧与帧之间的时间间隔.之前在使用`GameState.tick()`方法时,也说过帧的概念.

以下是对帧的2种解释:

- 调用`GameState.tick()`方法:每调用一次`GameState.tick()`方法,就表示游戏进行了一帧.但是如果游戏状态没有发生变化,则下一次渲染的画面和上一次的是完全一样的.这意味着玩家感受不到变化
  - 这种理解是从程序执行的频次理解帧的概念
- 通过成员属性`frame_time`控制:我们在逻辑上定义每经过`frame_time`的时长,游戏的状态就会发生变化,因此需要重新渲染.这种概念将游戏状态的每次改变都称为一帧.可以认为这个成员属性定义了屏幕的刷新率
  - 这种是从玩家感官的角度上理解帧的概念

以上两种状态都是对的.但是在游戏开发中,我们更多的是使用第二种概念.因为第二种概念更加直观,更加符合我们的思维方式

## 3.3.5 常量

```rust
// 屏幕宽度
const SCREEN_WIDTH: i32 = 80;

// 屏幕高度
const SCREEN_HEIGHT: i32 = 50;

// 屏幕刷新率 单位: 毫秒 即:每75ms重新渲染一次屏幕
const FRAME_DURATION: f32 = 75.0;
```

## 3.3.6 完善游戏的`play()`函数

```rust
impl State {
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

        // 判断游戏是否结束
        if self.player.y > SCREEN_HEIGHT {
            self.mode = GameMode::End;
        }
    }
}
```

- `self.frame_time += ctx.frame_time_ms;`和`if self.frame_time > FRAME_DURATION{}`: 这两行代码实现了帧与帧之间的时间间隔控制.
  - 默认情况下,`GameState.tick()`方法会以尽可能高的频次调用(通常每秒60次以上).但是玩家没有这么快的反应速度.`BTerm.frame_time_ms`这个属性表示上一次调用`GameState.tick()`方法到这一次调用`GameState.tick()`方法之间的时间间隔.通过这个属性,我们可以控制帧与帧之间的时间间隔
  - 这里我们记录了程序上认为的2帧之间的时间间隔,只有当这个间隔大于`FRAME_DURATION`时,才重新计算玩家的位置
  - 但实际上无论我们是否重新计算玩家的位置,整个屏幕都会重新渲染.所以这里只是让玩家在感官上感受到帧与帧之间的时间间隔
  - 根据我在本机上的测算,`BTerm.frame_time_ms`的值为33ms,即每次调用`GameState.tick()`方法的时间间隔为33ms
- `if let Some(VirtualKeyCode::Space) = ctx.key {}`: 计算扇动翅膀对玩家在垂直位置上带来的影响.这部分计算是不受`frame_time`控制的.
  - 因为扇动翅膀是玩家主动操作的,如果这个操作也受`frame_time`的控制,那么在1次`frame_time`(本例中即为75ms)的时长内,玩家按空格确实会减小下坠的速度,但是速度变化对位置的影响是在第75ms时才重新计算的.这样就会导致玩家无论何时按空格,其位置都会在第75ms时才发生变化.这样就会导致玩家感受不到按空格的效果
- `if self.player.y > SCREEN_HEIGHT {}`: 玩家下坠到屏幕底部则游戏结束
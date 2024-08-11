use bracket_lib::prelude::*;
use my_flappy_dragon::State;

fn main() {
    let state = State::new();
    let ctx = BTermBuilder::simple80x50().with_title("Flappy Dragon").build().unwrap();
    main_loop(ctx, state).unwrap()
}

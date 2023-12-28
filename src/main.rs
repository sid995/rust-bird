mod obstacles;
mod constants;
mod player;
mod game_state;

use crate::game_state::State;
use bracket_lib::prelude::*;


fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("Flappy Dragon").build()?;

    main_loop(context, State::new())
}
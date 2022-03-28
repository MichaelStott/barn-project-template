mod start_state;

use barn::game::barn_context::BarnContext;

use barn::game::game::Game;
use barn::game::state::State;
use crate::start_state::StartState;

fn main() {
    let mut game = Game::new(&String::from("Hello World!"), 500, 500, false);

    let state: Box<dyn State<BarnContext>> = Box::new(StartState::new());
    let context = BarnContext::new(&mut game);

    game.run(context, state).unwrap();
}

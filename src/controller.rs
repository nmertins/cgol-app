use piston::input::GenericEvent;

use cgol::{GameOfLife, GameOfLifeSettings, GameState};

pub struct CgolController {
    game: GameOfLife
}

impl CgolController {
    pub fn new(game_settings: GameOfLifeSettings) -> CgolController {
        let game = GameOfLife::new(game_settings);

        CgolController { game }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {

    }

    pub fn get_state(&self) -> &GameState {
        self.game.get_state()
    }
}
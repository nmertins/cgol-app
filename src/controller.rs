use piston::input::UpdateArgs;

use cgol::{GameOfLife, GameOfLifeSettings, GameState};

pub struct CgolController {
    game: GameOfLife
}

impl CgolController {
    pub fn new(game_settings: GameOfLifeSettings) -> CgolController {
        let game = GameOfLife::new(game_settings);

        CgolController { game }
    }

    pub fn update(&mut self, _args: UpdateArgs) {
        self.game.update();
    }

    pub fn get_state(&self) -> &GameState {
        self.game.get_state()
    }
}
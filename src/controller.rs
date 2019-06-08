use piston::input::UpdateArgs;

use cgol::{GameOfLife, GameOfLifeSettings, GameState};

pub struct CgolController {
    game: GameOfLife,
    world_dimensions: (usize, usize),
}

impl CgolController {
    pub fn new() -> CgolController {
        let game_settings = GameOfLifeSettings::from_file("resources/initial.state").unwrap();
        let world_dimensions = game_settings.get_dimensions();
        let game = GameOfLife::new(game_settings);

        CgolController {
            game,
            world_dimensions,
        }
    }

    pub fn update(&mut self, _args: UpdateArgs) {
        self.game.update();
    }

    pub fn get_state(&self) -> &GameState {
        self.game.get_state()
    }

    pub fn get_world_dimensions(&self) -> (usize, usize) {
        self.world_dimensions
    }
}
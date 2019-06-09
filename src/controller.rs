use piston::input::UpdateArgs;

use cgol::{GameOfLife, GameOfLifeSettings, GameState};

pub struct CgolState {
    live_cells: Vec<(usize, usize)>
}

impl CgolState {
    fn new() -> CgolState {
        CgolState { live_cells: vec![] }
    }

    fn update(&mut self, game_state: &GameState) {
        let mut new_state: Vec<(usize, usize)> = vec![];
        let (world_x, world_y) = game_state.get_dimensions();
        for y in 0..world_y {
            for x in 0..world_x {
                let cell_alive= game_state.get_cell_state(x, y);
                if cell_alive {
                    new_state.push((x, y));
                }
            }
        }

        self.live_cells = new_state;
    }

    pub fn get_live_cells(&self) -> &Vec<(usize, usize)> {
        &self.live_cells
    }
}

pub struct CgolController {
    game: GameOfLife,
    world_dimensions: (usize, usize),
    state: CgolState,
}

impl CgolController {
    pub fn new() -> CgolController {
        let game_settings = GameOfLifeSettings::from_file("resources/initial.state").unwrap();
        let world_dimensions = game_settings.get_dimensions();
        let game = GameOfLife::new(game_settings);
        let state = CgolState::new();

        CgolController {
            game,
            world_dimensions,
            state,
        }
    }

    pub fn update(&mut self, _args: UpdateArgs) {
        self.game.update();

        let game_state = self.game.get_state();
        self.state.update(game_state);
    }

    pub fn get_state(&self) -> &CgolState {
        &self.state
    }

    pub fn get_world_dimensions(&self) -> (usize, usize) {
        self.world_dimensions
    }
}
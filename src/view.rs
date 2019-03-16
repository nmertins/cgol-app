use graphics::types::Color;
use graphics::{Context, Graphics};

use cgol::GameState;

#[derive(Debug)]
pub struct CgolViewSettings {
    position: [f64; 2],
    size: f64,
    background_color: Color,
    grid_line_color: Color,
    cell_color: Color,
}

impl CgolViewSettings {
    pub fn new() -> CgolViewSettings {
        CgolViewSettings {
            position: [10.0; 2],
            size: 400.0,
            background_color: [1.0; 4],
            grid_line_color: [0.0; 4],
            cell_color: [1.0; 4],
        }
    }
}

pub struct CgolView {
    settings: CgolViewSettings,
}

impl CgolView {
    pub fn new(settings: CgolViewSettings) -> CgolView {
        CgolView { settings }
    }

    pub fn draw<G: Graphics>(&self, game_state: &Option<GameState>, c: &Context, g: &mut G) {}
}

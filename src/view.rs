use graphics::types::Color;
use graphics::{Context, Graphics};

use cgol::GameState;

#[derive(Debug)]
pub struct CgolViewSettings {
    cells_per_row: usize,
    grid_thickness: f64,
    background_color: Color,
    grid_line_color: Color,
    cell_color: Color,
}

impl CgolViewSettings {
    pub fn new() -> CgolViewSettings {
        CgolViewSettings {
            cells_per_row: 100,
            grid_thickness: 0.5,
            background_color: [1.0; 4],
            grid_line_color: [0.9, 0.9, 0.9, 1.0],
            cell_color: [0.0, 0.0, 0.0, 1.0],
        }
    }

    pub fn cells_per_row(mut self, cells_per_row: usize) -> Self {
        self.cells_per_row = cells_per_row;
        self
    }
}

pub struct CgolView {
    settings: CgolViewSettings,
}

impl CgolView {
    pub fn new(settings: CgolViewSettings) -> CgolView {
        CgolView { settings }
    }

    pub fn draw<G: Graphics>(&self, game_state: &Option<GameState>, c: &Context, g: &mut G) {
        let settings = &self.settings;

        graphics::clear(settings.background_color, g);
        self.draw_grid_lines(settings, c, g);
        if let Some(state) = game_state {
            self.fill_live_cells(settings, state, c, g);
        }
    }

    fn draw_grid_lines<G: Graphics>(&self, settings: &CgolViewSettings, c: &Context, g: &mut G) {
        if let Some(v) = c.viewport {
            let cell_edge = graphics::Line::new(settings.grid_line_color, settings.grid_thickness);

            for i in 1..settings.cells_per_row {
                let y1 =
                    (i as f64 / settings.cells_per_row as f64 * v.window_size[1] as f64) as f64;
                let y2 =
                    (i as f64 / settings.cells_per_row as f64 * v.window_size[1] as f64) as f64;

                let h_line = [0.0, y1, v.window_size[0] as f64, y2];
                cell_edge.draw(h_line, &c.draw_state, c.transform, g);

                let x1 =
                    (i as f64 / settings.cells_per_row as f64 * v.window_size[0] as f64) as f64;
                let x2 =
                    (i as f64 / settings.cells_per_row as f64 * v.window_size[0] as f64) as f64;

                let v_line = [x1, 0.0, x2, v.window_size[1] as f64];
                cell_edge.draw(v_line, &c.draw_state, c.transform, g);
            }
        }
    }

    fn fill_live_cells<G: Graphics>(&self, settings: &CgolViewSettings, game_state: &GameState, c: &Context, g: &mut G) {

    }
}

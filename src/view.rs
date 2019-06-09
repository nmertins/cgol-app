use graphics::types::Color;
use graphics::{Context, Graphics, Rectangle};

use crate::controller::CgolState;

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

    pub fn draw<G: Graphics>(&self, state: &CgolState, c: &Context, g: &mut G) {
        let settings = &self.settings;

        graphics::clear(settings.background_color, g);
        self.draw_grid_lines(settings, c, g);
        self.fill_live_cells(settings, state, c, g);
    }

    fn draw_grid_lines<G: Graphics>(&self, settings: &CgolViewSettings, c: &Context, g: &mut G) {
        if let Some(v) = c.viewport {
            let cell_edge = graphics::Line::new(settings.grid_line_color, settings.grid_thickness);

            let window_width = v.window_size[0];
            let window_height = v.window_size[1];

            for i in 1..settings.cells_per_row {
                let y1 =
                    i as f64 / settings.cells_per_row as f64 * window_height;
                let y2 =
                    i as f64 / settings.cells_per_row as f64 * window_height;

                let horizontal_line = [0.0, y1, window_width, y2];
                cell_edge.draw(horizontal_line, &c.draw_state, c.transform, g);

                let x1 =
                    i as f64 / settings.cells_per_row as f64 * window_width;
                let x2 =
                    i as f64 / settings.cells_per_row as f64 * window_width;

                let vertical_line = [x1, 0.0, x2, window_height];
                cell_edge.draw(vertical_line, &c.draw_state, c.transform, g);
            }
        }
    }

    fn fill_live_cells<G: Graphics>(&self, settings: &CgolViewSettings, state: &CgolState, c: &Context, g: &mut G) {
        if let Some(viewport) = c.viewport {
            let window_width = viewport.window_size[0];
            let window_height = viewport.window_size[1];

            let cell_width = window_width/settings.cells_per_row as f64;
            let cell_height = window_height/settings.cells_per_row as f64;

            let live_cells = state.get_live_cells();
            for (x, y) in live_cells {
                let cell_x = *x as f64 * cell_width;
                let cell_y = *y as f64 * cell_height;

                let cell = [cell_x, cell_y, cell_width, cell_height];
                Rectangle::new(settings.cell_color).draw(cell, &c.draw_state, c.transform, g);
            }
        }
    }
}

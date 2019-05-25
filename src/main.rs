extern crate cgol;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod view;
mod controller;

use piston::event_loop::{EventSettings, Events, EventLoop};
use piston::window::WindowSettings;
use piston::input::{UpdateEvent, RenderEvent};

use glutin_window::GlutinWindow;

use opengl_graphics::{GlGraphics, OpenGL};

use view::{CgolView, CgolViewSettings};
use crate::controller::CgolController;
use cgol::GameOfLifeSettings;

fn main() {
    let opengl = OpenGL::V3_2;
    let window_settings = WindowSettings::new("Game of Life", [700, 700])
        .opengl(opengl)
        .exit_on_esc(true);
    let mut window: GlutinWindow = window_settings.build().expect("Could not create window");
    let event_settings = EventSettings::new()
        .ups(1);
    let mut events = Events::new(event_settings);
    let mut gl = GlGraphics::new(opengl);

    let game_settings = GameOfLifeSettings::from_file("resources/initial.state").unwrap();

    let (rows, _) = game_settings.get_dimensions();
    let view_settings = CgolViewSettings::new()
        .cells_per_row(rows);
    let view = CgolView::new(view_settings);
    let mut controller = CgolController::new(game_settings);

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.update_args() {
            controller.update(args);
        }

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                view.draw(controller.get_state(), &c, g);
            });
        }
    }
}

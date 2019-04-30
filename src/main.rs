extern crate cgol;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod view;
mod controller;

use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;

use glutin_window::GlutinWindow;

use opengl_graphics::{GlGraphics, OpenGL};

use view::{CgolView, CgolViewSettings};
use crate::controller::CgolController;
use cgol::{GameOfLife, GameError};

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Game of Life", [700, 700])
        .opengl(opengl)
        .exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");
    let mut events = Events::new(EventSettings::new());
    let mut gl = GlGraphics::new(opengl);

    let mut game = GameOfLife::new();
    if let Err(e) = game.set_state("resources/initial.state") {
        panic!(e)
    }
    let (rows, columns) = game.get_state().unwrap().get_dimensions();

    let view_settings = CgolViewSettings::new()
        .cells_per_row(rows);
    let view = CgolView::new(view_settings);
    let mut controller = CgolController{};

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            controller.event(&e);
            gl.draw(args.viewport(), |c, g| {
                view.draw(&None, &c, g);
            });
        }
    }
}

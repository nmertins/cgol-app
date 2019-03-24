extern crate cgol;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod view;

use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;

use glutin_window::GlutinWindow;

use opengl_graphics::{GlGraphics, OpenGL};

use view::{CgolView, CgolViewSettings};

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Game of Life", [700, 700])
        .opengl(opengl)
        .exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");
    let mut events = Events::new(EventSettings::new());
    let mut gl = GlGraphics::new(opengl);

    let view_settings = CgolViewSettings::new();
    let view = CgolView::new(view_settings);

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                view.draw(&None, &c, g);
            });
        }
    }
}

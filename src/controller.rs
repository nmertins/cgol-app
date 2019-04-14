use piston::input::GenericEvent;

use cgol::GameOfLife;

pub struct CgolController {}

impl CgolController {
    pub fn event<E: GenericEvent>(&mut self, e: &E) {

    }
}
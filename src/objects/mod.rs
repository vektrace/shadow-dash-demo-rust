use macroquad::prelude::*;

mod platform;
pub use platform::Platform;

pub trait Object {
    fn cbox(&self) -> &Rect;
    fn draw(&self);
    // map loading stuff
    fn z(&self) -> u32;
    fn scale(&self) -> Vec2;
}

pub fn draw_all(objects: &Vec<Box<dyn Object>>) {
    for object in objects {
        object.draw();
    }
}

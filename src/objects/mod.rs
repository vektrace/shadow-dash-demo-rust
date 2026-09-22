use macroquad::prelude::*;

mod platform;
pub use platform::Platform;

pub trait Object {
    fn cbox(&self) -> &Rect;
    // fn properties(&self) -> HashMap<&str, &str>;
    fn draw(&self);
}

pub fn draw_all(objects: &Vec<Box<dyn Object>>) {
    for object in objects {
        object.draw();
    }
}

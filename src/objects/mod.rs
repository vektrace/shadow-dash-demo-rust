use std::collections::HashMap;

use macroquad::prelude::*;

mod platform;
pub use platform::Platform;

pub trait Object {
    fn id(&self) -> &str;
    fn cbox(&self) -> &Rect;
    fn z(&self) -> u32;
    fn properties(&self) -> HashMap<&str, &str>;
    fn draw(&self);
}

pub fn draw_all(objects: &Vec<Box<dyn Object>>) {
    for object in objects {
        object.draw();
    }
}

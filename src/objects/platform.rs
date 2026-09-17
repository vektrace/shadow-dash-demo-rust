use super::{HashMap, Object, Rect, Texture2D, WHITE, draw_texture};
use macroquad::prelude::*;

pub struct Platform {
    cbox: Rect,
    texture: Texture2D,
}

impl Object for Platform {
    fn id(&self) -> &str {
        todo!("read id from map file");
    }

    fn cbox(&self) -> &Rect {
        &self.cbox
    }

    fn z(&self) -> u32 {
        todo!("read z index from map file");
    }

    fn properties(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }

    fn draw(&self) {
        draw_texture(&self.texture, self.cbox.x, self.cbox.y, WHITE);
    }
}

impl Platform {
    pub fn new(cbox: Rect, texture: Texture2D) -> Self {
        Self { cbox, texture }
    }
}

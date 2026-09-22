use super::{Object, Rect, Texture2D, WHITE, draw_texture};

pub struct Platform {
    cbox: Rect,
    texture: Texture2D,
}

impl Object for Platform {
    fn cbox(&self) -> &Rect {
        &self.cbox
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

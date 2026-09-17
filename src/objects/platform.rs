use super::{HashMap, Object, Rect, Texture2D, WHITE, draw_texture};

pub struct Platform {
    id: String,
    cbox: Rect,
    texture: Texture2D,
}

impl Object for Platform {
    fn id(&self) -> &str {
        &self.id
    }

    fn cbox(&self) -> &Rect {
        &self.cbox
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
        todo!("generate id (check for duplicates)");
        /*
                Self {
                    id,
                    cbox,
                    texture,
                }
        */
    }
}

use super::{
    DrawTextureParams, Object, Rect, Texture2D, Vec2, WHITE, draw_texture_ex, load_texture,
};

pub struct Trampoline {
    cbox: Rect,
    texture: Texture2D,
}

impl Object for Trampoline {
    fn cbox(&self) -> &Rect {
        &self.cbox
    }

    fn draw(&self) {
        let params = DrawTextureParams {
            dest_size: Some(Vec2::new(self.cbox.w, self.cbox.h)),
            ..Default::default()
        };
        draw_texture_ex(&self.texture, self.cbox.x, self.cbox.y, WHITE, params);
    }
}

impl Trampoline {
    pub async fn new(cbox: Rect) -> Self {
        Self {
            cbox,
            texture: load_texture("assets/sprites/trampoline/trampoline.png")
                .await
                .unwrap(),
        }
    }
}

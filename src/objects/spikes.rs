use super::{
    DrawTextureParams, Object, Rect, Texture2D, Vec2, WHITE, draw_texture_ex, load_texture,
};

pub struct Spikes {
    cbox: Rect,
    texture: Texture2D,
}

impl Object for Spikes {
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

impl Spikes {
    pub async fn new(cbox: Rect) -> Self {
        Self {
            cbox,
            texture: load_texture("assets/sprites/spikes/spikes.png")
                .await
                .unwrap(),
        }
    }
}

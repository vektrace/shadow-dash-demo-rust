use super::{
    DrawTextureParams, Object, Rect, Texture2D, Vec2, WHITE, draw_texture_ex, load_texture,
};

pub struct MovingPlatform {
    cbox: Rect,
    texture: Texture2D,
    start: Vec2,
    end: Vec2,
    speed: u32,
}

impl Object for MovingPlatform {
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

impl MovingPlatform {
    pub async fn new(cbox: Rect, start: Vec2, end: Vec2, speed: u32) -> Self {
        Self {
            cbox,
            texture: load_texture("assets/sprites/moving_platform/moving_platform.png")
                .await
                .unwrap(),
            start,
            end,
            speed,
        }
    }
}

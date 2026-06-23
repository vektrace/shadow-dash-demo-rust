use macroquad::prelude::*;

pub struct Player {
    pub speed: f32,
    pub jump: f32,
    pub dash: f32,
    pub x: f32,
    pub y: f32,
    pub texture: Texture2D,
}

impl Player {
    pub async fn new() -> Self {
        Self {
            speed: 250.0,
            jump: 100.0,
            dash: 100.0,
            x: 250.0,
            y: 250.0,
            texture: load_texture("assets/sprites/spr_player/spr_player.png")
                .await
                .unwrap(),
        }
    }
}

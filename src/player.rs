use macroquad::prelude::*;

pub struct Player {
    pub speed: f32,
    pub gravity: f32,
    pub jump: f32,
    pub dash: f32,
    
    pub x: f32,
    pub y: f32,
    pub xspeed: f32,
    pub yspeed: f32,
    pub direction: f32,
    pub texture: Texture2D,
}

impl Player {
    pub async fn new() -> Self {
        Self {
            speed: 250.0,
            gravity: 0.5 * 60.0 * 60.0,
            jump: 10000.0,
            dash: 100.0 * 60.0,
            x: 250.0,
            y: 0.0,
            xspeed: 0.0,
            yspeed: 0.0,
            direction: 0.,
            texture: load_texture("assets/sprites/spr_player/spr_player.png")
                .await
                .unwrap(),
        }
    }

    pub fn apply_gravity(&mut self, delta_time: f32) {
        self.yspeed += self.gravity * delta_time;

        // self.y += self.yspeed * delta_time;
    }

    pub fn apply_speed(&mut self, delta_time: f32) {
        if self.y < 400. {
            self.y += self.yspeed * delta_time;
        }
        self.x += self.xspeed;
        
    }
}

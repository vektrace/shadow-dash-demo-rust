use macroquad::prelude::*;
// use vector2::Vector2;

pub struct Player {
    pub cbox: Rect,

    pub speed: Vec2,

    pub on_ground: bool,

    pub can_jump: bool,
    pub can_double_jump: bool,

    pub direction: f32,
    pub texture: Texture2D,
}

impl Player {
    // units/sec
    pub const SPEED: f32 = 250.0;
    pub const GRAVITY: f32 = 0.5 * 60.0; // units/sec**2
    pub const JUMP: f32 = 400.0;
    pub const DASH: f32 = 100.0 * 60.0;
}

impl Player {
    pub async fn new() -> Self {
        Self {
            cbox: Rect::new(300., 0., 32., 32.),

            speed: vec2(0.0, 0.0),

            on_ground: false,

            can_jump: false,
            can_double_jump: false,

            direction: 0.0,
            texture: load_texture("assets/sprites/spr_player/spr_player.png")
                .await
                .unwrap(),
        }
    }

    pub fn apply_gravity(&mut self) {
        if !self.on_ground {
            self.speed.y += Self::GRAVITY;
        }

        // self.y += self.yspeed * delta_time;
    }

    pub fn apply_speed(&mut self, delta_time: f32) {
        /*
        self.pos.y += self.speed.y * delta_time;
        self.pos.x += self.speed.x * delta_time;
        */
        self.cbox = self.cbox.offset(self.speed * delta_time);
    }
}

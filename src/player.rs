use super::Object;
use macroquad::prelude::*;

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
    pub const GRAVITY: f32 = 0.5 * 60.0;
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
    }

    pub fn apply_speed(&mut self, delta_time: f32) {
        self.cbox = self.cbox.offset(self.speed * delta_time);
    }

    pub fn check_collision(&mut self, objects: &Vec<Box<dyn Object>>) {
        self.on_ground = false;
        for object in objects {
            let object_cbox = object.cbox();

            if !self.cbox.overlaps(object_cbox) {
                continue;
            }

            // calculate overlap (distance)
            let overlap_left = self.cbox.right() - object_cbox.x;
            let overlap_right = object_cbox.right() - self.cbox.x;
            let overlap_top = self.cbox.bottom() - object_cbox.y;
            let overlap_bottom = object_cbox.bottom() - self.cbox.y;

            // take overlap which is closest to not clipping
            let min = overlap_left
                .min(overlap_right)
                .min(overlap_top)
                .min(overlap_bottom);

            // check which one is closest to min (not clipping)
            if min >= overlap_top {
                self.cbox.y = object_cbox.y - self.cbox.h;
                self.speed.y = 0.0;
                self.on_ground = true;
                self.can_jump = true;
            } else if min >= overlap_bottom {
                self.cbox.y = object_cbox.bottom();
            } else if min >= overlap_left {
                self.cbox.x = object_cbox.x - self.cbox.w;
            } else {
                self.cbox.x = object_cbox.right();
            }
        }
    }
}

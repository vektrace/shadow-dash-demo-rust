use super::Object;
use macroquad::prelude::*;

enum CollisionSide {
    Top,
    Bottom,
    Left,
    Right,
}

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
            let player_cbox = self.cbox;
            if !player_cbox.overlaps(object_cbox) {
                continue;
            }
            match self.collision_side(object_cbox) {
                CollisionSide::Top => {
                    self.cbox.y = object_cbox.y - self.cbox.h;
                    self.speed.y = 0.0;
                    self.on_ground = true;
                    self.can_jump = true;
                }
                CollisionSide::Bottom => {
                    self.cbox.y = object_cbox.y + object_cbox.h;
                }
                CollisionSide::Left => {
                    self.cbox.x = object_cbox.x - self.cbox.w;
                }
                CollisionSide::Right => {
                    self.cbox.x = object_cbox.x + object_cbox.w;
                }
            }
        }
    }

    fn collision_side(&self, object: &Rect) -> CollisionSide {
        let overlap_left = (self.cbox.x + self.cbox.w) - object.x;
        let overlap_right = (object.x + object.w) - self.cbox.x;
        let overlap_top = (self.cbox.y + self.cbox.h) - object.y;
        let overlap_bottom = (object.y + object.h) - self.cbox.y;

        let min = overlap_left
            .min(overlap_right)
            .min(overlap_top)
            .min(overlap_bottom);

        if min >= overlap_top {
            CollisionSide::Top
        } else if min >= overlap_bottom {
            CollisionSide::Bottom
        } else if min >= overlap_left {
            CollisionSide::Left
        } else {
            CollisionSide::Right
        }
    }
}

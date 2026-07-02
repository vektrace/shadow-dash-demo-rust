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
            if self.cbox.overlaps(&object_cbox.offset(vec2(0., 0.))) {
                let inter_rect = self.cbox.intersect(*object_cbox).unwrap();
                if (inter_rect.h - object_cbox.h).abs() < 0.000_001 {
                    //inter_rect.h == ob.cbox.h {
                    // if the entire wall is inside the player

                    if self.speed.x > 0. && self.cbox.x < object_cbox.center().x {
                        // if going right and on the left of center
                        self.cbox.x = object_cbox.x - self.cbox.w - 0.;
                        self.on_ground = true;
                        println!("left");
                    } else if self.speed.x < 0. && object_cbox.center().x < self.cbox.x {
                        // if going left and on the right of center
                        self.cbox.x = object_cbox.right() + 0.;
                        self.on_ground = true;
                        println!("right");
                    }
                }

                if self.speed.y > 0. && self.cbox.bottom() < object_cbox.bottom() {
                    // if moving down and over ob

                    self.cbox.y = object_cbox.y - self.cbox.h;
                    self.on_ground = true;

                    println!("up");
                } else if self.speed.y < 0. && self.cbox.top() > object_cbox.top() {
                    // if moving up and under ob
                    self.cbox.y = object_cbox.bottom();
                    println!("down");
                }
            }
        }
        if self.on_ground {
            self.can_jump = true;
            self.speed.y = 0.;
        }
    }
}

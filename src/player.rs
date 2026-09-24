use super::Object;
use macroquad::prelude::*;

#[derive(PartialEq, Debug)]
pub enum PlayerJumpState {
    CanJump,
    CanDoubleJump,
    Used,
}

#[derive(PartialEq, Debug)]
pub enum PlayerDashState {
    CanDash,
    IsDashing,
    OnCooldown,
}

pub struct Player {
    pub cbox: Rect,

    pub accell: Vec2,
    pub velocity: Vec2,

    pub on_ground: bool,

    pub jump: PlayerJumpState,
    pub dash: PlayerDashState,

    pub direction: f32,
    pub texture: Texture2D,
}

impl Player {
    // units/sec
    pub const SPEED: f32 = 250.0;
    pub const GRAVITY: f32 = 0.5 * 60. * 60.; // og value but with delta time
    pub const JUMP: f32 = 400.0;
    //
    // not * 60 because it doesn't have delta time anymore
    pub const DASH: f32 = 100.;

    // myb for the future
    // pub const DAMPENING: f32 = 1.1;
}

impl Player {
    pub async fn new() -> Self {
        Self {
            cbox: Rect::new(300., 0., 32., 32.),

            accell: vec2(0., 0.),
            velocity: vec2(0., 0.),

            on_ground: false,

            jump: PlayerJumpState::CanJump,
            dash: PlayerDashState::CanDash,

            direction: 0.0,
            texture: load_texture("assets/sprites/player/player.png")
                .await
                .unwrap(),
        }
    }

    pub fn apply_gravity(&mut self) {
        if !self.on_ground && self.dash != PlayerDashState::IsDashing {
            self.accell.y += Self::GRAVITY;
        }
    }

    pub fn apply_speed(&mut self, delta_time: f32) {
        // myb for the future
        // self.accell += -self.velocity * Player::DAMPENING;

        self.velocity += self.accell * delta_time;
        self.cbox = self.cbox.offset(self.velocity * delta_time);

        // reset accell after every frame
        self.accell *= 0.;
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

                // reset on hitting ground
                // ( check_collision is called after input, gravity and
                //   apply_speed so it doesn't break jumping because the jump
                //   already happend at this point in time )
                self.accell.y = 0.;
                self.velocity.y = 0.;

                self.on_ground = true;
                self.jump = PlayerJumpState::CanJump;
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

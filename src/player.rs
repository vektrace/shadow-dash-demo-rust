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

    pub dash_dest: f32,
    dash_timer: f32,

    pub direction: f32,
    pub texture: Texture2D,
}

impl Player {
    // units/sec
    pub const SPEED: f32 = 250.0;
    pub const GRAVITY: f32 = 0.5 * 60. * 60.; // og value but with delta time
    pub const JUMP: f32 = 400.0;

    pub const DASH_DIST: f32 = 100.;
    const DASH_DURATION: f32 = 0.166_666_67;
    const DASH_COOLDOWN: f32 = 0.416_666_67;

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

            dash_dest: 0.,
            dash_timer: 0.,

            direction: 0.0,
            texture: load_texture("assets/sprites/player/player.png")
                .await
                .unwrap(),
        }
    }

    pub fn tick(&mut self, delta_time: f32, objects: &Vec<Box<dyn Object>>) {
        match self.dash {
            PlayerDashState::CanDash => {}
            PlayerDashState::IsDashing => {
                self.dash_tick(delta_time);
            }
            PlayerDashState::OnCooldown => {
                if self.dash_timer > 0. {
                    self.dash = PlayerDashState::CanDash;
                }
                self.dash_timer += delta_time;
            }
        }

        self.apply_gravity();
        self.apply_speed(delta_time);
        self.check_collision(objects);
    }

    fn dash_tick(&mut self, delta_time: f32) {
        let temp_dest =
            self.cbox.x + (Self::DASH_DIST / (Self::DASH_DURATION / delta_time)).clamp(0., Self::DASH_DIST) * self.direction;



        // possible exploit with delta_time = 10 (s because of intentional lag) eg. moving to right
        // self.cbox.x + (100 / (~0.16... / 10) ) * 1
        // => self.cbox.x + 100 / 0.016...
        // => self.cbox.x + 6000.
        //
        // more math:
        // self.cbox.x + ( Self::DASH_DIST / (Self::DASH_DURATION / delta_time))
        // self.cbox.x + ( 100 / (0.16... / 0.16...) ) = 100
        //
        // => all delta_time values smaller than the dash duration cause a
        // dash thats bigger than wanted
        //
        //
        // delta_time = 1 / fps
        //
        // 0.16... = 1 / fps | *fps; /0.16...
        // fps = 1 / 0.16... = 6.0
        //
        // => all fps values under [ 1 / dash_duration (currently 0.166..) = ]
        // 6 cause a bigger than wanted dash
        //
        // possible fix: capping the value with
        // self.cbox.x + min(Self::DASH_DIST, (...) ) * self.direction;
        // before the direction might make it negative
        //
        // min is slit with f32 so clamp it is
        // self.cbox.x + (...).clamp(0., Self::DASH_DIST) ...



        // if it the dest was passed
        // (moving to the right = if temp_dest is right of it)
        // or ...
        if (self.direction >= 1. && self.dash_dest <= temp_dest
            || self.direction <= -1. && temp_dest <= self.dash_dest)
            // ... the time ran out
            || self.dash_timer >= Self::DASH_DURATION
        {
            self.dash_timer = -Self::DASH_COOLDOWN;
            self.dash = PlayerDashState::OnCooldown;

            // move player to dest
            self.cbox.x = self.dash_dest;

            return;
        }

        self.dash_timer += delta_time;
        self.cbox.x = temp_dest;
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

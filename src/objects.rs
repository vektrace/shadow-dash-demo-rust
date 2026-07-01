use super::player::Player;
use macroquad::prelude::*;

pub struct Object {
    pub cbox: Rect,

    pub texture: Texture2D,
}

impl Player {
    pub fn objects_draw_coll(&mut self, objects: &Vec<Object>) {
        self.on_ground = false;
        for ob in objects {
            draw_texture(&ob.texture, ob.cbox.x, ob.cbox.y, WHITE);

            if self.cbox.overlaps(&ob.cbox.offset(vec2(0., 0.))) {
                let inter_rect = self.cbox.intersect(ob.cbox).unwrap();
                if (inter_rect.h - ob.cbox.h).abs() < 0.000_001 {
                    //inter_rect.h == ob.cbox.h {
                    // if the entire wall is inside the player

                    if self.speed.x > 0. && self.cbox.x < ob.cbox.center().x {
                        // if going right and on the left of center
                        self.cbox.x = ob.cbox.x - self.cbox.w - 0.;
                        self.on_ground = true;
                        println!("left");
                    } else if self.speed.x < 0. && ob.cbox.center().x < self.cbox.x {
                        // if going left and on the right of center
                        self.cbox.x = ob.cbox.right() + 0.;
                        self.on_ground = true;
                        println!("right");
                    }
                }

                if self.speed.y > 0. && self.cbox.bottom() < ob.cbox.bottom() {
                    // if moving down and over ob

                    self.cbox.y = ob.cbox.y - self.cbox.h;
                    self.on_ground = true;

                    println!("up");
                } else if self.speed.y < 0. && self.cbox.top() > ob.cbox.top() {
                    // if moving up and under ob
                    self.cbox.y = ob.cbox.bottom();
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

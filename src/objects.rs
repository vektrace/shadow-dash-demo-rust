use macroquad::prelude::*;

use super::player::Player;

pub struct Object {
    pub cbox: Rect,

    pub texture: Texture2D,
}

fn is_in_range(a: f32, b: f32, c: f32) -> bool {
    a < b && b < c
}

impl Player {
    pub fn do_collision(&mut self, objects: &Vec<Object>) {
        self.on_ground = false;
        for ob in objects {
            draw_texture(&ob.texture, ob.cbox.x, ob.cbox.y, WHITE);

            // checks if player x/y is inside the object
            // if is_in_range(ob.pos.y, self.pos.y, (ob.pos.y + ob.size.y){
            //     self.speed.x = 0.;
            // }
            // if ob.cbox.contains(vec2(ob.cbox.x, ob.cbox.bottom() + 4.)) {
            // self.speed.y = 0.;
            // self.on_ground = true;
            // }
            // if is_in_range(ob.cbox.y, self.cbox.y, (ob.cbox.y + ob.cbox.h)) && is_in_range(ob.cbox.x, self.cbox.x, ob.cbox.x + ob.cbox.w) { // old ver

            if self.cbox.overlaps(&ob.cbox.offset(vec2(0., 0.))) {
                /*
                let inter_rect = self.cbox.intersect(ob.cbox).unwrap();
                if self.speed.x > 0. && inter_rect.left() == ob.cbox.left() {
                    // going right and on the left
                    self.cbox.x = ob.cbox.x - self.cbox.w - 0.;
                    self.speed.y = 0.;
                    println!("left");
                } else if self.speed.x < 0. && inter_rect.right() == ob.cbox.right() {
                    // going left and on the right
                    self.cbox.x = ob.cbox.right() + 0.;
                    self.speed.y = 0.;
                    println!("right");
                }
                */

                if self.speed.y > 0. && self.cbox.bottom() < ob.cbox.bottom() {
                    // if moving down and over ob

                    self.cbox.y = ob.cbox.y - self.cbox.h;
                    self.speed.y = 0.;
                    self.on_ground = true;
                    self.is_jumping = false;
                    println!("up")
                } else if self.speed.y < 0. && self.cbox.y > ob.cbox.y {
                    // if moving up and under ob
                    self.cbox.y = ob.cbox.bottom();
                    println!("down")
                }
            }
        }
    }
}

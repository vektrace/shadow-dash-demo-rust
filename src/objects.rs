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
            // if is_in_range(ob.pos.x, self.pos.x, (ob.pos.x + ob.size.x){
            //     self.speed.x = 0.;
            // }
            if is_in_range(ob.cbox.y, self.cbox.y, ob.cbox.y + ob.cbox.h)
                && is_in_range(ob.cbox.x, self.cbox.x, ob.cbox.x + ob.cbox.w)
            {
                self.speed.y = 0.;
                self.on_ground = true;
            }
        }
    }
}

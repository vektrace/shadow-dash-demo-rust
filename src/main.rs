use macroquad::miniquad::conf::Icon;
use macroquad::prelude::*;
use std::sync::OnceLock;

mod keybinds;
mod map;
mod objects;
mod player;
mod tileset;

use keybinds::KeyBinds;
use objects::{Object, draw_all};
use player::Player;
use tileset::load_tileset;

static TILESET: OnceLock<Vec<tileset::Tile>> = OnceLock::new();
static FONT: OnceLock<Font> = OnceLock::new();

// TODO:
// - include_bytes for all assets
// - use cargo-bundle for... well bundling

fn window_conf() -> Conf {
    let icon = Icon {
        small: *include_bytes!("../assets/sprites/icon/icon_16.rgba"),
        medium: *include_bytes!("../assets/sprites/icon/icon_32.rgba"),
        big: *include_bytes!("../assets/sprites/icon/icon_64.rgba"),
    };
    Conf {
        window_title: String::from("Shadow Dash Demo"),
        fullscreen: true,
        window_resizable: false,
        icon: Some(icon),
        ..Default::default()
    }
}

struct Game {
    player: Player,
    delta_time: f32,
    debug: bool,
    // key_binds: KeyBinds,
    objects: Vec<Box<dyn Object>>,
}

impl Game {
    async fn new() -> Self {
        Self {
            player: Player::new(0., 0., 0., 0.).await,
            delta_time: 0.0,
            debug: false,
            // key_binds: KeyBinds::new()
            objects: vec![],
        }
    }

    fn draw(&self, scale: f32) {
        const DEBUG_FONT_SIZE: f32 = 48.;

        // clear_background instead of texture so it covers the entire screen
        clear_background(Color::from_hex(0x0000_AEF0));

        let debug_textparams = TextParams {
            font: Some(FONT.get().unwrap()),
            font_size: (DEBUG_FONT_SIZE * scale).round() as u16,
            ..Default::default()
        };

        draw_texture_ex(
            &self.player.texture,
            self.player.cbox.x,
            self.player.cbox.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.player.cbox.w, self.player.cbox.h)),
                ..Default::default()
            },
        );
        draw_all(&self.objects);

        if self.debug {
            // the starting place
            let mut place_cords: Vec2 = vec2(10., 200.);
            place_cords *= scale;

            const MARGIN: Vec2 = vec2(0., 30.);

            for text in [
                format!("fps: {}", get_fps()),
                format!("jump: {:#?}", self.player.jump),
                format!("dash: {:#?}", self.player.dash),
                format!("x/y: {:#?}/{:?}", self.player.cbox.x, self.player.cbox.y),
                format!("accell: {:#?}", self.player.accell),
                format!("vel: {:#?}", self.player.velocity),
                format!(
                    "screensize(x/y): {:#?}/{:#?}",
                    macroquad::window::screen_width(),
                    macroquad::window::screen_height()
                ),
            ] {
                draw_text_ex(text, place_cords.x, place_cords.y, debug_textparams.clone());

                place_cords += MARGIN * scale;
            }
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    FONT.set(load_ttf_font("assets/fonts/lubbartz.ttf").await.unwrap())
        .ok();

    let mut game = Game::new().await;
    let key_binds = KeyBinds::new();

    load_tileset().await;

    let map = map::load_map("level1").await;
    game.player = map.place_objects(&mut game.objects).await;

    let mut old_screen_width = 0.;
    let mut old_screen_height = 0.;

    loop {
        // delta time: makes for example speed dependent on seconds NOT on frames:
        // x += accell * delta_time

        if screen_width() != old_screen_width || screen_height() != old_screen_height {
            game.objects.clear();
            game.player = map.place_objects(&mut game.objects).await;

            old_screen_width = screen_width();
            old_screen_height = screen_height();

            game.player.consts.reload(map.get_scale());
        }

        // first apply all forces, then check collision, then draw frame
        game.delta_time = get_frame_time();

        key_binds.do_input(&mut game);

        game.player.apply_gravity();

        game.player.apply_speed(game.delta_time);

        game.player.check_collision(&game.objects);

        game.draw(map.get_scale());

        // reset accell after every frame
        game.player.accell *= 0.;

        next_frame().await;
    }
}

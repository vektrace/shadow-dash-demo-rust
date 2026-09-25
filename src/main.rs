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
            player: Player::new(0., 0.).await,
            delta_time: 0.0,
            debug: false,
            // key_binds: KeyBinds::new()
            objects: vec![],
        }
    }

    fn draw(&self) {
        // clear_background instead of texture so it covers the entire screen
        clear_background(Color::from_hex(0x0000_AEF0));

        let textparams = TextParams {
            font: Some(FONT.get().unwrap()),
            font_size: 64,
            ..Default::default()
        };
        draw_texture(
            &self.player.texture,
            self.player.cbox.x,
            self.player.cbox.y,
            WHITE,
        );
        draw_all(&self.objects);

        if self.debug {
            draw_text_ex(format!("fps: {}", get_fps()), 10., 160., textparams.clone());
            draw_text_ex(
                format!("jump: {:#?}", self.player.jump),
                10.,
                200.0,
                textparams.clone(),
            );
            draw_text_ex(
                format!("dash: {:#?}", self.player.dash),
                10.,
                240.0,
                textparams.clone(),
            );

            draw_text_ex(
                format!("x/y: {:#?}/{:?}", self.player.cbox.x, self.player.cbox.y),
                10.,
                280.0,
                textparams.clone(),
            );
            draw_text_ex(
                format!("accell: {:#?}", self.player.accell),
                10.,
                320.0,
                textparams.clone(),
            );
            draw_text_ex(
                format!("vel: {:#?}", self.player.velocity),
                10.,
                360.0,
                textparams.clone(),
            );
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

    loop {
        // delta time: makes for example speed dependent on seconds NOT on frames:
        // x += accell * delta_time

        // first apply all forces, then check collision, then draw frame
        game.delta_time = get_frame_time();

        key_binds.do_input(&mut game);

        game.player.apply_gravity();

        game.player.apply_speed(game.delta_time);

        game.player.check_collision(&game.objects);

        game.draw();

        // reset accell after every frame
        game.player.accell *= 0.;

        next_frame().await;
    }
}

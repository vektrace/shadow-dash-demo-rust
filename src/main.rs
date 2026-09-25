use macroquad::miniquad::conf::Icon;
use macroquad::prelude::*;

mod keybinds;
mod objects;
mod player;

use keybinds::KeyBinds;
use objects::{Object, Platform, draw_all};
use player::Player;

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
    font: Font,
    delta_time: f32,
    debug: bool,
    // key_binds: KeyBinds,
    objects: Vec<Box<dyn Object>>,
}

impl Game {
    async fn new() -> Self {
        Self {
            font: load_ttf_font("assets/fonts/lubbartz.ttf").await.unwrap(),
            player: Player::new(0., 0.).await,
            delta_time: 0.0,
            debug: false,
            // key_binds: KeyBinds::new()
            objects: vec![
                Box::new(Platform::new(
                    Rect::new(100., 100., 64., 16.),
                    load_texture("assets/sprites/platform/platform.png")
                        .await
                        .unwrap(),
                )),
                Box::new(Platform::new(
                    Rect::new(300., 200., 64., 16.),
                    load_texture("assets/sprites/platform/platform.png")
                        .await
                        .unwrap(),
                )),
                Box::new(Platform::new(
                    Rect::new(364., 216., 64., 16.),
                    load_texture("assets/sprites/platform/platform.png")
                        .await
                        .unwrap(),
                )),
                Box::new(Platform::new(
                    Rect::new(300., 135., 64., 16.),
                    load_texture("assets/sprites/platform/platform.png")
                        .await
                        .unwrap(),
                )),
                Box::new(Platform::new(
                    Rect::new(300., 151., 64., 16.),
                    load_texture("assets/sprites/platform/platform.png")
                        .await
                        .unwrap(),
                )),
                Box::new(Platform::new(
                    Rect::new(300., 167., 64., 16.),
                    load_texture("assets/sprites/platform/platform.png")
                        .await
                        .unwrap(),
                )),
                Box::new(Platform::new(
                    Rect::new(300., 183., 64., 16.),
                    load_texture("assets/sprites/platform/platform.png")
                        .await
                        .unwrap(),
                )),
                Box::new(Platform::new(
                    Rect::new(500., 950., 64., 16.),
                    load_texture("assets/sprites/platform/platform.png")
                        .await
                        .unwrap(),
                )),
                Box::new(Platform::new(
                    Rect::new(300., 50., 64., 16.),
                    load_texture("assets/sprites/platform/platform.png")
                        .await
                        .unwrap(),
                )),
            ],
        }
    }

    fn draw(&self) {
        // clear_background instead of texture so it covers the entire screen
        clear_background(Color::from_hex(0x0000_AEF0));

        let textparams = TextParams {
            font: Some(&self.font),
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
    let mut game = Game::new().await;
    let key_binds = KeyBinds::new();

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

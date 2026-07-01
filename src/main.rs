use macroquad::miniquad::conf::Icon;
use macroquad::prelude::*;

mod keybinds;
mod objects;
mod player;

use keybinds::KeyBinds;
use objects::Object;
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
    // key_binds: KeyBinds,
    objects: Vec<Object>,
}

impl Game {
    async fn new() -> Self {
        Self {
            player: Player::new().await,
            font: load_ttf_font("assets/fonts/lubbartz.ttf").await.unwrap(),
            delta_time: 0.0,
            // key_binds: KeyBinds::new()
            objects: vec![
                Object {
                    cbox: Rect::new(100., 100., 64., 16.),
                    /* // works, but dont know if properly
                    // had to read the docs for this lol
                    texture: Texture2D::from_file_with_format(
                    include_bytes!("../assets/sprites/spr_platform/spr_platform.png"),
                    None,
                    ),
                    */
                    texture: load_texture("assets/sprites/spr_platform/spr_platform.png")
                        .await
                        .unwrap(),
                },
                Object {
                    cbox: Rect::new(300., 200., 64., 16.),
                    /* // works, but dont know if properly
                    // had to read the docs for this lol
                    texture: Texture2D::from_file_with_format(
                    include_bytes!("../assets/sprites/spr_platform/spr_platform.png"),
                    None,
                    ),
                    */
                    texture: load_texture("assets/sprites/spr_platform/spr_platform.png")
                        .await
                        .unwrap(),
                },
                Object {
                    cbox: Rect::new(364., 216., 64., 16.),
                    /* // works, but dont know if properly
                    // had to read the docs for this lol
                    texture: Texture2D::from_file_with_format(
                    include_bytes!("../assets/sprites/spr_platform/spr_platform.png"),
                    None,
                    ),
                    */
                    texture: load_texture("assets/sprites/spr_platform/spr_platform.png")
                        .await
                        .unwrap(),
                },
                Object {
                    cbox: Rect::new(300., 135., 64., 16.),
                    /* // works, but dont know if properly
                    // had to read the docs for this lol
                    texture: Texture2D::from_file_with_format(
                    include_bytes!("../assets/sprites/spr_platform/spr_platform.png"),
                    None,
                    ),
                    */
                    texture: load_texture("assets/sprites/spr_platform/spr_platform.png")
                        .await
                        .unwrap(),
                },
            ],
        }
    }

    fn draw(&self) {
        // clear_background instead of texture so it covers the entire screen
        clear_background(Color::from_hex(0x0000_AEF0));

        // since the texture covers the entire screen, all texture x and y values are in the top
        // left corner (this will be fun later)
        // draw_texture(&bg, 0.0, 0.0, WHITE);

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
        draw_text_ex("HELLO", 200.0, 200.0, textparams);
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new().await;
    let key_binds = KeyBinds::new();

    loop {
        // delta time: makes for example speed dependent on seconds NOT on frames:
        // x += speed * delta_time
        game.delta_time = get_frame_time();

        game.draw();

        game.player.apply_gravity();

        key_binds.do_input(&mut game);

        game.player.do_collision(&game.objects);
        game.player.apply_speed(game.delta_time);
        next_frame().await;
    }
}

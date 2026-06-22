use macroquad::input::KeyCode;
use macroquad::miniquad::conf::Icon;
use macroquad::prelude::*;

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
    SPEED: f32,
    x: f32,
    y: f32,
    font: Font,
    player: Texture2D,
    delta_time: f32,
    // key_binds: KeyBinds,
}

impl Game {
    async fn new() -> Self {
        Self {
            SPEED: 250.0,
            x: 250.0,
            y: 250.0,
            font: load_ttf_font("assets/fonts/lubbartz.ttf").await.unwrap(),
            player: load_texture("assets/sprites/spr_player/spr_player.png")
                .await
                .unwrap(),
            delta_time: 0.0,
            // key_binds: KeyBinds::new()
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
        draw_texture(&self.player, self.x, self.y, WHITE);
        draw_text_ex("HELLO", 200.0, 200.0, textparams);
    }
}

/*
type Key = [Option<KeyCode>; 2];

fn keybind(a: KeyCode, b: KeyCode) -> Key {
    [Some(a), Some(b)]
}

struct KeyBinds {
    left: Key,
    right: Key,
    jump: Key,
    dash: Key,
}

impl KeyBinds {
    fn new() -> Self {
        Self {
            left: keybind(KeyCode::A, KeyCode::Left),
            right: keybind(KeyCode::D, KeyCode::Right),
            jump: keybind(KeyCode::W, KeyCode::Up),
            dash: [Some(KeyCode::Space), None],
        }
    }

    fn left(&self) {
        super::x -= super::SPEED * super::delta_time
    }
}
*/

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new().await;

    loop {
        // delta time: makes for example speed dependent on seconds NOT on frames:
        // x += speed * delta_time
        game.delta_time = get_frame_time();

        game.draw();

        // TODO:
        // - add keybinds with 2 options at the same time
        // - add helper fn for shorter code when doing 2 keys

        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            game.x -= game.SPEED * game.delta_time;
        } else if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            game.x += game.SPEED * game.delta_time;
        }

        next_frame().await;
    }
}

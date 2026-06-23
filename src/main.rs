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
    player: Player,
    font: Font,
    delta_time: f32,
    // key_binds: KeyBinds,
}

impl Game {
    async fn new() -> Self {
        Self {
            player: Player::new().await,
            font: load_ttf_font("assets/fonts/lubbartz.ttf").await.unwrap(),
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
        draw_texture(&self.player.texture, self.player.x, self.player.y, WHITE);
        draw_text_ex("HELLO", 200.0, 200.0, textparams);
    }
}

struct Player {
    speed: f32,
    jump: f32,
    dash: f32,
    x: f32,
    y: f32,
    texture: Texture2D,
}

impl Player {
    async fn new() -> Self {
        Self {
            speed: 250.0,
            jump: 100.0,
            dash: 100.0,
            x: 250.0,
            y: 250.0,
            texture: load_texture("assets/sprites/spr_player/spr_player.png")
                .await
                .unwrap(),
        }
    }
}

type Key = [Option<KeyCode>; 2];

fn keybind(a: KeyCode, b: KeyCode) -> Key {
    [Some(a), Some(b)]
}

struct KeyBind {
    keys: Key,
    action: fn(&mut Game),
}

struct KeyBinds {
    binds: [KeyBind; 4], // increase when more keys are added
}

impl KeyBinds {
    fn new() -> Self {
        Self {
            binds: [
                KeyBind {
                    keys: keybind(KeyCode::A, KeyCode::Left),
                    action: Self::action_left,
                },
                KeyBind {
                    keys: keybind(KeyCode::D, KeyCode::Right),
                    action: Self::action_right,
                },
                KeyBind {
                    keys: keybind(KeyCode::W, KeyCode::Up),
                    action: Self::action_jump,
                },
                KeyBind {
                    keys: [Some(KeyCode::Space), None],
                    action: Self::action_dash,
                },
            ],
        }
    }

    fn action_left(g: &mut Game) {
        g.player.x -= g.player.speed * g.delta_time;
    }

    fn action_right(g: &mut Game) {
        g.player.x += g.player.speed * g.delta_time;
    }

    fn action_jump(g: &mut Game) {
        g.player.y -= g.player.jump * g.delta_time;
    }

    fn action_dash(g: &mut Game) {
        g.player.x += g.player.dash * g.delta_time;
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

        // TODO:
        // - add keybinds with 2 options at the same time
        // - add helper fn for shorter code when doing 2 keys

        for kb in &key_binds.binds {
            let held = kb.keys.into_iter().any(|k| k.is_some_and(is_key_down));
            if held {
                (kb.action)(&mut game);
            }
        }
        next_frame().await;
    }
}

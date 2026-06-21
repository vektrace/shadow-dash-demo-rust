use macroquad::input::KeyCode;
use macroquad::miniquad::conf::Icon;
use macroquad::prelude::*;

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

// TODO:
// - include_bytes for all assets
// - use cargo-bundle for... well bundling

#[macroquad::main(window_conf)]
async fn main() {
    const SPEED: f32 = 250.0;

    let mut x = 250.0;
    let mut y = 250.0;

    let font = load_ttf_font("assets/fonts/lubbartz.ttf").await.unwrap();
    let player = load_texture("assets/sprites/spr_player/spr_player.png")
        .await
        .unwrap();

    loop {
        // delta time: makes for example speed dependent on seconds NOT on frames:
        // x += speed * delta_time
        let delta_time = get_frame_time();

        // clear_background instead of texture so it covers the entire screen
        clear_background(Color::from_hex(0x0000_AEF0));

        // since the texture covers the entire screen, all texture x and y values are in the top
        // left corner (this will be fun later)
        // draw_texture(&bg, 0.0, 0.0, WHITE);

        let textparams = TextParams {
            font: Some(&font),
            font_size: 64,
            ..Default::default()
        };
        draw_texture(&player, x, y, WHITE);
        draw_text_ex("HELLO", 200.0, 200.0, textparams);

        // TODO:
        // - add keybinds with 2 options at the same time
        // - add helper fn for shorter code when doing 2 keys

        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            x -= SPEED * delta_time;
        } else if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            x += SPEED * delta_time;
        }

        next_frame().await;
    }
}

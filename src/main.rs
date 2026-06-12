use macroquad::prelude::*;

#[macroquad::main("Shadow Dash")]
async fn main() {
    set_fullscreen(true);
    let font = load_ttf_font("assets/fonts/lubbartz.ttf").await.unwrap();
    let bg = load_texture("assets/sprites/bg1/bg1.png").await.unwrap();
    let player = load_texture("assets/sprites/spr_player/spr_player.png")
        .await
        .unwrap();
    loop {
        // since the texture covers the entire screen, all texture x and y values are in the top
        // left corner (this will be fun later)
        draw_texture(&bg, 0.0, 0.0, WHITE);

        let textparams = TextParams {
            font: Some(&font),
            font_size: 64,
            ..Default::default()
        };
        draw_texture(&player, 250.0, 250.0, WHITE);
        draw_text_ex("HELLO", 200.0, 200.0, textparams);

        next_frame().await
    }
}

use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let font = load_ttf_font("assets/Inter_18pt-Bold.ttf").await.unwrap();
    loop {
        clear_background(DARKGRAY);
        draw_text_ex(
            "симулятор",
            20.0,
            20.0,
            TextParams {
                font: Some(&font),
                font_size: 18,
                color: BLACK,
                ..Default::default()
            },
        );

        next_frame().await
    }
}
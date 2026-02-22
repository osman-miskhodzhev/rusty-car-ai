use macroquad::prelude::*;
mod car;
use car::Car;

#[macroquad::main("Симулятор")]
async fn main() {
    let font = load_ttf_font("assets/fonts/Inter_18pt-Bold.ttf").await.unwrap();
    let mut car = Car::new(400.0, 300.0).await;
    loop {
        clear_background(DARKGRAY);
        let dt = get_frame_time();

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
        car.update(dt);
        car.draw();

        next_frame().await
    }
}
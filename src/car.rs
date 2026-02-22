use macroquad::prelude::*;

pub struct Car{
    x: f32,
    y: f32,
    angle: f32,
    vx: f32,
    vy: f32,
    texture: Texture2D,
}

impl Car {

    pub async fn new(x: f32, y: f32) -> Self {
        let texture: Texture2D = load_texture("assets/img/car.png").await.unwrap();
        Self { 
            x,
            y,
            angle: 0.0,
            vx: 0.0,
            vy: 0.0,
            texture,
        }
    }
    
    pub fn draw(&self) {
        draw_texture_ex(
            &self.texture,
            self.x,
            self.y,
            WHITE,
            DrawTextureParams{
                rotation: self.angle,
                ..Default::default()
            }
        );
    }
}
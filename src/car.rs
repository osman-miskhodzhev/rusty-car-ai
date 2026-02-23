use macroquad::prelude::*;

pub struct Car{
    x: f32,
    y: f32,
    angle: f32,
    rotation_speed:f32,
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
            rotation_speed: 2.5,
            vx: 0.0,
            vy: 0.0,
            texture,
        }
    }

    pub fn update(&mut self, dt:f32) {
        // Вперед и назад
         let mut input_forward = 0.0;
         let mut input_right = 0.0;
        if is_key_down(KeyCode::Up) {
            input_forward -= 1.0;
        }
        if is_key_down(KeyCode::Down) {
            input_forward += 1.0;
        }

        // вправо и влево
        if is_key_down(KeyCode::Right) {
            input_right -= 1.0;
            self.angle += self.rotation_speed * dt;
        }
        if is_key_down(KeyCode::Left) {
            input_right += 1.0;
            self.angle -= self.rotation_speed * dt;
        }

        // расчет векторов направлений
        let forward = vec2(self.angle.sin(), self.angle.cos());
        let right = vec2(-self.angle.cos(), self.angle.sin());

        // расчет ускорений
        let acceleration_power: f32 = 500.0;

        let accel_x = (input_forward * forward.x + input_right * right.x) * acceleration_power;
        let accel_y = (input_forward * forward.y + input_right * right.y) * acceleration_power;

        // обновление скорости с трением
        let friction = 2.0;
        let max_speed = 300.0;

        self.vx += accel_x * dt;
        self.vy += accel_y * dt;
        self.vx *= 1.0 - friction * dt;
        self.vy *= 1.0 - friction * dt;


        // ограничение скорости
        let speed = (self.vx * self.vx + self.vy * self.vy).sqrt();
        if speed > max_speed {
            self.vx = (self.vx / speed) * max_speed;
            self.vy = (self.vy / speed) * max_speed;
        }

        // Обновление позиции
        self.x += self.vx * dt;
        self.y += self.vy * dt;

        // обновление угла
        //self.angle = (-self.vy).atan2(self.vx);

        // границы экрана
        let screen_width = screen_width();
        let screen_height = screen_height();
        
        // Простые границы - машинка не может выйти за экран
        self.x = self.x.clamp(20.0, screen_width - 20.0);
        self.y = self.y.clamp(20.0, screen_height - 20.0);
        
        // Если уперлись в границу - останавливаемся
        if self.x <= 20.0 || self.x >= screen_width - 20.0 {
            self.vx = 0.0;
        }
        if self.y <= 20.0 || self.y >= screen_height - 20.0 {
            self.vy = 0.0;
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
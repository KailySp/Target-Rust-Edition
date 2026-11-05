pub mod laser
{
    use std::f32::consts::PI;

    use sdl3::render::{Texture, TextureCreator};
    use sdl3::video::WindowContext;
    use sdl3::image::LoadTexture;

    use crate::SCREEN_WIDTH;
    use crate::SCREEN_HEIGHT;

    pub const LASER_WIDTH: f32 = 50.;
    pub const LASER_HEIGHT: f32 = 13.;

    const SPEED: f32 = 10.;

    pub struct Laser<'a>
    {
        // State variables
        pub x: f32, pub y: f32, pub angle: f64,
        pub to_destroy: bool,

        // Graphical Data
        pub sprite_texture: Texture<'a>,
    }

    impl<'a> Laser<'a>
    {
        pub fn new(texture_creator: &'a TextureCreator<WindowContext>, start_x: f32, start_y: f32, start_angle: f64) -> Self
        {
            Self
            {
                // State variables
                x: start_x, y: start_y, angle: start_angle,
                to_destroy: false,

                // Graphical Data
                sprite_texture: texture_creator.load_texture("data/images/laser.png").unwrap(),
            }
        }

        pub fn update(&mut self)
        {
            // Going forward
            self.x += ((self.angle as f32) * PI/180.).cos() * SPEED;
            self.y += ((self.angle as f32) * PI/180.).sin() * SPEED;

            //Going off screen
            if self.x < -LASER_WIDTH || self.x > SCREEN_WIDTH + LASER_WIDTH
                || self.y < -LASER_WIDTH || self.y > SCREEN_HEIGHT + LASER_WIDTH
            {self.to_destroy = true;}
        }
    }
}

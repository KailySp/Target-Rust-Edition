pub mod asteroid
{
    use sdl3::render::{Texture, TextureCreator, FRect};
    use sdl3::image::LoadTexture;
    use sdl3::video::{WindowContext};

    use crate::SCREEN_WIDTH;
    use crate::MAIN_SPEED;


    const SPEED_RATIO: f32 = 60.; // lucky number :3

    pub struct Asteroid<'tc>
    {
        // State variables
        pub x: f32, pub y: f32, pub angle: f64, pub size: f32,
        pub to_destroy: bool, pub rotation_speed: f64,

        // Graphical data
        pub sprite_texture: Texture<'tc>,
        pub sprite_rect: FRect,
    }

    impl<'tc> Asteroid<'tc>
    {
        pub fn new(texture_creator: &'tc TextureCreator<WindowContext>, start_x: f32, new_angle: f64, new_size: f32, new_type: i8, rd: i8) -> Self
        {
            Self
            {
                // State variables
                x: start_x, y: SCREEN_WIDTH, angle: new_angle, size: new_size,
                to_destroy: false, rotation_speed: if rd == 0 {1.} else {-1.} * ( SPEED_RATIO / (2.*new_size.exp2()).sqrt()/2.) as f64, // no comment

                // Graphical data
                sprite_texture: texture_creator.load_texture("data/images/asteroide.png").unwrap(),
                sprite_rect: match new_type
                {
                    0 => FRect{x:0., y:0., w:128., h:128.},
                    1 => FRect{x:128., y:0., w:128., h:128.},
                    2 => FRect{x:0., y:128., w:128., h:128.},
                    3 => FRect{x:128., y:128., w:128., h:128.},
                    _ => FRect { x: 0., y: 0., w: 0., h: 0.}
                }
            }
        }

        pub fn update(&mut self)
        {
            self.x -= MAIN_SPEED;

            self.angle += self.rotation_speed;
            if self.angle > 360. {self.angle -= 360.}
            if self.angle < 0. {self.angle += 360.}
        }
    }
}

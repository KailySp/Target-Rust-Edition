pub mod ship 
{
    use sdl3::render::{Texture, TextureCreator, FRect};
    use sdl3::image::LoadTexture;
    use sdl3::video::WindowContext;

use crate::SCREEN_WIDTH;


    // Constants parameters
    // ------------------------
    const MAX_SPEED: f32 = 5.;
    const ROTATION_SPEED: f32 = 7.;
    const JET_POWER: f32 = 0.1;

    const MAX_ENERGY: f32 = 30.; 
    const CHARGE_RATE: f32 = 1./5.;

    pub struct Ship<'a>
    {
        // State variables
        pub size: f32,
        pub x: f32, pub y: f32, pub angle: f32,
        velocity_x: f32, velocity_y: f32,
        pub energy: f32, can_recharge: bool,
        pub fire_on: bool,

        // Graphical Data
        pub sprite_texture: Texture<'a>,
        pub body_rect: FRect, pub fire_rect: FRect,

    }

    impl<'a> Ship<'a>
    {
        pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Self
        {
            Self
            {
                // State variables
                size: 64.0,
                x: 500.0, y: 250.0, angle: 0.0,
                velocity_x: 0.0, velocity_y: 0.0,
                energy: MAX_ENERGY, can_recharge: true,
                fire_on: false,


                // Graphical data 
                sprite_texture: texture_creator.load_texture("data/images/sprites.png").unwrap(),
                body_rect: FRect{x:2048.0, y:0.0, w:256.0, h:256.0}, 
                fire_rect: FRect{x:2048.0, y:256.0, w:256.0, h:256.0},

            }
        }

        pub fn update(&mut self)
        {
            // Manage fire_on
            if self.fire_on
            {
                self.velocity_x += (self.angle * std::f32::consts::PI/180.0).cos() * JET_POWER;
                self.velocity_y += (self.angle * std::f32::consts::PI/180.0).sin() * JET_POWER;

                // Manage max speed
                self.velocity_x = if self.velocity_x > MAX_SPEED {MAX_SPEED} else {self.velocity_x};
                self.velocity_y = if self.velocity_y > MAX_SPEED {MAX_SPEED} else {self.velocity_y};

            }

            // Manage Space physics
            self.x += self.velocity_x;
            self.y += self.velocity_y;

            // Manage Screen limit
            match (self.x, self.y)
            {
                (x, _) if x < 0.0 => {self.velocity_x = 0.0; self.x = 0.0;}
                (x, _) if x > crate::SCREEN_WIDTH - self.size => {self.velocity_x = 0.0; self.x = crate::SCREEN_WIDTH - self.size;}
                (_, y) if y < 0.0 => {self.velocity_y = 0.0; self.y = 0.0;}
                (_, y) if y > crate::SCREEN_HEIGHT - self.size => {self.velocity_y = 0.0; self.y = crate::SCREEN_HEIGHT - self.size;}
                (_, _) => {}
            }





        }
    }
}

pub mod ship
{
    use std::f32::consts::PI;

    use sdl3::render::{Texture, TextureCreator, FRect};
    use sdl3::image::LoadTexture;
    use sdl3::video::WindowContext;
    use sdl3::mixer::{Mixer, Audio};

    use crate::SCREEN_WIDTH;
    use crate::SCREEN_HEIGHT;

    use crate::laser::laser::Laser;
    use crate::laser::laser::LASER_WIDTH;
    use crate::laser::laser::LASER_HEIGHT;

    #[derive(PartialEq, Clone)]
    pub enum RotationDirection
    {
        None,
        Left,
        Right
    }


    // Constants parameters with a bunch of lucky numbers
    // ------------------------
    pub const SHIP_SIZE: f32 = 64.;

    const MAX_SPEED: f32 = 5.;
    const ROTATION_SPEED_STICK: f64 = 7.;
    const ROTATION_SPEED_BUTTON: f64 = 6.;
    const JET_POWER: f32 = 0.1;

    const MAX_ENERGY: f32 = 30.; 
    const CHARGE_RATE: f32 = 1./5.;

    pub struct Ship<'tc, 'm>
    {
        // State variables
        pub x: f32, pub y: f32, pub angle: f64,
        pub target_angle: Option<f64>, pub rotation_direction: RotationDirection,
        velocity_x: f32, velocity_y: f32,
        pub energy: f32, can_recharge: bool,
        pub fire_on: bool,

        // Graphical Data
        pub sprite_texture: Texture<'tc>,
        pub body_rect: FRect, pub fire_rect: FRect,

        // Sound Data
        mixer: &'m Mixer,
        laser_sfx: Audio,


    }

    impl<'tc, 'm> Ship<'tc, 'm>
    {
        pub fn new(texture_creator: &'tc TextureCreator<WindowContext>, new_mixer: &'m Mixer) -> Self
        {
            Self
            {
                // State variables
                x: 500.0, y: 250.0, angle: 0.0,
                target_angle: None, rotation_direction: RotationDirection::None,
                velocity_x: 0.0, velocity_y: 0.0,
                energy: MAX_ENERGY, can_recharge: true,
                fire_on: false,


                // Graphical data 
                sprite_texture: texture_creator.load_texture("data/images/sprites.png").unwrap(),
                body_rect: FRect{x:2048.0, y:0.0, w:256.0, h:256.0}, 
                fire_rect: FRect{x:2048.0, y:256.0, w:256.0, h:256.0},

                // Sound Data
                mixer: new_mixer,
                laser_sfx: new_mixer.load_audio("data/music/lazer.wav", true).unwrap(),

            }
        }

        pub fn update(&mut self)
        {
            // Manage fire_on
            if self.fire_on
            {
                self.velocity_x += ((self.angle as f32) * PI/180.).cos() * JET_POWER;
                self.velocity_y += ((self.angle as f32) * PI/180.).sin() * JET_POWER;

                // Manage max speed
                self.velocity_x = if self.velocity_x.abs() > MAX_SPEED {MAX_SPEED*self.velocity_x.signum()} else {self.velocity_x};
                self.velocity_y = if self.velocity_y.abs() > MAX_SPEED {MAX_SPEED*self.velocity_y.signum()} else {self.velocity_y};

            }

            // Manage Space physics
            self.x += self.velocity_x;
            self.y += self.velocity_y;

            // Manage Screen limit
            if self.x < 0. {self.velocity_x = 0.; self.x = 0.;}
            if self.x > SCREEN_WIDTH - SHIP_SIZE {self.velocity_x = 0.; self.x = crate::SCREEN_WIDTH - SHIP_SIZE;}
            if self.y < 0. {self.velocity_y = 0.; self.y = 0.;}
            if self.y > SCREEN_HEIGHT - SHIP_SIZE {self.velocity_y = 0.; self.y = crate::SCREEN_HEIGHT - SHIP_SIZE;}


            // Manage Rotation
            match self.rotation_direction
            {
                RotationDirection::None => {}
                RotationDirection::Left => {self.angle -= ROTATION_SPEED_BUTTON;}
                RotationDirection::Right => {self.angle += ROTATION_SPEED_BUTTON;}
            }

            if self.angle > 360. {self.angle -= 360.}
            if self.angle < 0. {self.angle += 360.}
        }

        pub fn fire(&mut self, texture_creator: &'tc TextureCreator<WindowContext>) -> Option<Laser<'tc>>
        {
            if self.energy <= 0. {None}
            else
            {
                self.mixer.play_audio(&self.laser_sfx).unwrap();
                Some(Laser::new(
                    texture_creator,
                    (self.x + SHIP_SIZE/2.) - LASER_WIDTH/2.,
                    (self.y + SHIP_SIZE/2.) - LASER_HEIGHT/2.,
                    self.angle))
            }
        }
    }
}





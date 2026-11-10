pub mod target
{
    use sdl3::render::{Texture, TextureCreator, FRect};
    use sdl3::image::LoadTexture;
    use sdl3::video::{WindowContext};

    use crate::{SCREEN_WIDTH, MAIN_SPEED, CircleCollider};

    pub struct Target<'tc>
    {
        // State variables
        pub x: f32, pub y: f32, pub size: f32,
        pub to_destroy: bool,

        // Graphical data
        pub sprite_texture: Texture<'tc>,
    }

    impl<'tc> Target<'tc>
    {
        pub fn new(texture_creator: &'tc TextureCreator<WindowContext>, start_y: f32, new_size: f32, new_type: i8) -> Self
        {
            Self
            {
                // State variables
                x: SCREEN_WIDTH + new_size, y: start_y, size: new_size,
                to_destroy: false,

                // Graphical data
                sprite_texture: match new_type
                {
                    0 => texture_creator.load_texture("data/images/redTarget.png").unwrap(),
                    1 => texture_creator.load_texture("data/images/greenTarget.png").unwrap(),
                    2 => texture_creator.load_texture("data/images/yellowTarget.png").unwrap(),
                    _ => panic!("Euuh nah")
                }
            }
        }

        pub fn update(&mut self)
        {
            self.x -= MAIN_SPEED;

            if self.x < -self.size {self.to_destroy = true}
        }

        pub fn get_circle_collider(&self) -> CircleCollider
        {
            CircleCollider { x: self.x + self.size/2., y: self.y + self.size/2., r: self.size/2.}
        }
    }
}

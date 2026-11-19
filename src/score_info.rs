pub mod score_info
{
    use sdl3::render::{FRect, Texture, TextureCreator, Canvas};
    use sdl3::video::{WindowContext};
    use sdl3::ttf::Font;
    use sdl3::pixels::Color;

    pub const SCORE_INFO_SIZE: f32 = 24.;
    const ALIVE_TIME: i32 = 120;
    const ORIGINAL_VELOCITY_Y: f32 = 10.;

    pub struct ScoreInfo<'tc>
    {
        // State variables
        pub x: f32, pub y: f32, velocity_y: f32,
        pub to_destroy: bool, alive_timer : i32,

        // Graphical data
        pub score_texture: Texture<'tc>,
    }

    impl<'tc> ScoreInfo<'tc>
    {
        pub fn new(texture_creator: &'tc TextureCreator<WindowContext>, pixel_font: &Font, start_x: f32, start_y: f32, value: i32) -> Self
        {
            Self
            {
                // State variables
                x: start_x, y: start_y, velocity_y: ORIGINAL_VELOCITY_Y,
                to_destroy: false, alive_timer: ALIVE_TIME,

                // Graphical data
                score_texture :
                {
                    let score_text = format!("{0}", value);
                    let score_surface = pixel_font.render(&score_text).solid(Color::RGBA(0, 255, 0, 255)).unwrap();
                    texture_creator.create_texture_from_surface(&score_surface).unwrap()
                }
            }
        }

        pub fn update(&mut self)
        {
            self.alive_timer -= 1;
            if self.alive_timer > 0
            {
                self.y = self.y - self.velocity_y;
                self.velocity_y = self.velocity_y / 1.5; // lucky number again....
            }
            else {self.to_destroy = true;}
        }
    }
}

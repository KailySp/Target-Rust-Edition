use sdl3::pixels::Color;
use sdl3::event::Event;
use sdl3::EventPump;
use sdl3::keyboard::Keycode;
use sdl3::surface::Surface;
use sdl3::render::{FRect, FPoint, Texture, Canvas};
use sdl3::video::Window;
use sdl3::mixer::Mixer;

use std::time::Duration;

struct GameEventState 
{
    game_stop: bool,
}

impl GameEventState 
{
    fn create() -> Self
    {
        Self { game_stop: false }
    }
}


fn draw (canvas: &mut Canvas<Window>, texture: &Texture, x: f32, y: f32, angle: Option<f64>, src_rect: Option<FRect>)
{
    let dst_rect = Some(FRect{x: x, y: y, w: texture.width() as f32, h: texture.height() as f32});

    if let Some(a) = angle 
    {
        let center = Some(FPoint{x: x + (texture.width() as f32)/2.0, y: y + (texture.height() as f32)/2.0});
        canvas.copy_ex(texture, src_rect, dst_rect, a, center, false, false).unwrap();
    }
    else 
    {
        canvas.copy(texture, src_rect, dst_rect).unwrap();
    }
}


fn poll_events(event_pump: &mut EventPump,  game_event_state: &mut GameEventState)
{
    for event in event_pump.poll_iter() 
    {
        match event
        {
            Event::Quit {..} => {game_event_state.game_stop = true;}
            Event::KeyDown{keycode: Some(Keycode::Q), ..} => {game_event_state.game_stop = true;}
            _ => {}
        }
    }
}


fn main() 
{
    // Init SDL
    // -----------
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Target Rust Edition", 1000, 500)
        .position_centered()
        .build()
        .unwrap();


    let mut canvas = window.into_canvas();
    let texture_creator = canvas.texture_creator();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut game_running: bool = true;


    // Mixer INIT 
    // ---------------
    let _ = sdl3::mixer::init().unwrap();
    let mixer = Mixer::open_device(None).unwrap();
    let soundtrack = mixer.create_track().unwrap();
    soundtrack.set_loops(-1).unwrap();


    // In game setup
    // ------------------
    
    // Events
    let mut game_event_state = GameEventState::create();

    // Background
    let background = Surface::load_surface("data/images/background.png").unwrap();
    let background = texture_creator.create_texture_from_surface(&background).unwrap();

    // Soundtrack
    let target_song = mixer.load_audio("data/music/TargetSong.wav", true).unwrap();
    soundtrack.set_audio(&target_song).unwrap();
    soundtrack.play().unwrap();


    
    // Main Loop
    // ---------------
    while game_running 
    {
        // Polls Events
        // --------------------
        poll_events(&mut event_pump, &mut game_event_state);


        // Update
        // ---------

        if game_event_state.game_stop {game_running = false};
        

        // Draw 
        // --------
        
        // Clear the board 
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        
        // Draw background
        draw(&mut canvas, &background, 0.0, 0.0, None, None);
        

        // Present and cap at 60fps
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

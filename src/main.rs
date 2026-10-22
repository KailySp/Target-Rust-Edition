use sdl3::pixels::Color;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use std::time::Duration;

fn main() {
    // Init SDL
    // -----------
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Target Rust Edition", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut game_running: bool = true;

    // Main Loop
    // ---------------
    while game_running {
        // Clear the board 
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        
        // Polls Events 
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {game_running = false;}
                Event::KeyDown{keycode: Some(Keycode::Q), ..} => {game_running = false;}
                _ => {}
            }
        }


        // Present and cap at 60fps
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    
}

use sdl3::pixels::Color;
use sdl3::event::Event;
use sdl3::EventPump;
use sdl3::keyboard::Keycode;
use sdl3::render::{FRect, Texture, TextureCreator, Canvas};
use sdl3::video::{Window, WindowContext};
use sdl3::mixer::Mixer;
use sdl3::image::LoadTexture;

use ship::ship::Ship;
use ship::ship::SHIP_SIZE;
use ship::ship::RotationDirection;
mod ship;

mod laser;
use laser::laser::Laser;
use laser::laser::LASER_WIDTH;
use laser::laser::LASER_HEIGHT;

mod asteroid;
use asteroid::asteroid::Asteroid;


use std::time::Duration;

pub static SCREEN_WIDTH: f32 = 1000.0;
pub static SCREEN_HEIGHT: f32 = 500.0;

pub static MAIN_SPEED: f32 = 3. ; // we love lucky number here :3
pub static DEBRI_FREQUENCY: f32 = 30.; // this value is in frame lol

struct GameEventState 
{
    game_stop: bool, ship_fire_on: bool, ship_rotation_direction: RotationDirection, ship_fire_laser: bool
}
impl GameEventState 
{
    fn create() -> Self
    {
        Self { game_stop: false, ship_fire_on: false, ship_rotation_direction: RotationDirection::None, ship_fire_laser: false}
    }
}


fn draw (canvas: &mut Canvas<Window>, texture: &Texture, dst_rect: FRect, angle: Option<f64>, src_rect: Option<FRect>)
{
    if let Some(a) = angle 
    {
        canvas.copy_ex(texture, src_rect, Some(dst_rect), a, None, false, false).unwrap();
    }
    else 
    {
        canvas.copy(texture, src_rect, Some(dst_rect)).unwrap();
    }
}


fn poll_events(event_pump: &mut EventPump,  ges: &mut GameEventState)
{
    for event in event_pump.poll_iter() 
    {
        match event
        {
            // General App keybinds
            Event::Quit {..} => {ges.game_stop = true;}
            Event::KeyDown{keycode: Some(Keycode::Escape), ..} => {ges.game_stop = true;}

            // Fire_On keys
            Event::KeyDown{keycode: Some(Keycode::Z), ..} => {ges.ship_fire_on = true;}
            Event::KeyUp{keycode: Some(Keycode::Z), ..} => {ges.ship_fire_on = false;}

            // Rotation buttons
            Event::KeyDown{keycode: Some(Keycode::Q), ..} => {ges.ship_rotation_direction = RotationDirection::Left;}
            Event::KeyDown{keycode: Some(Keycode::D), ..} => {ges.ship_rotation_direction = RotationDirection::Right;}

            Event::KeyUp{keycode: Some(Keycode::Q), ..} if ges.ship_rotation_direction == RotationDirection::Left =>
                {ges.ship_rotation_direction = RotationDirection::None;}
            Event::KeyUp{keycode: Some(Keycode::D), ..} if ges.ship_rotation_direction == RotationDirection::Right =>
                {ges.ship_rotation_direction = RotationDirection::None;}

            //Lasers
            Event::KeyDown{keycode: Some(Keycode::L), ..} => {ges.ship_fire_laser = true;}



            _ => {}
        }
    }
}



fn random_asteroid<'tc>(texture_creator: &'tc TextureCreator<WindowContext>) -> Asteroid<'tc>
{
    let new_size = rand::random_range(40..140);
    let start_y = rand::random_range(new_size..(500 - new_size));
    let new_type = rand::random_range(0..4);
    let new_angle = rand::random_range(0..360);
    let rd = rand::random_range(0..=1);
    Asteroid::new(texture_creator, start_y as f32, new_angle as f64, new_size as f32, new_type, rd)
}

fn update_level<'tc>(texture_creator: &'tc TextureCreator<WindowContext>, debri_timer: &mut f32, asteroids: &mut Vec<Asteroid<'tc>>)
{
    *debri_timer -= 1.;
    if *debri_timer <= 0.
    {
        let number = rand::random_range(0..10);
        if number == 1 {}
        else if number <= 7 {asteroids.push(random_asteroid(texture_creator));}

        *debri_timer = DEBRI_FREQUENCY;
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


    // In game setup
    // ------------------
    
    // Events
    let mut game_event_state = GameEventState::create();

    // Background
    let background = texture_creator.load_texture("data/images/background.png").unwrap();

    // Soundtrack
    let target_song = mixer.load_audio("data/music/TargetSong.wav", true).unwrap();
    soundtrack.set_audio(&target_song).unwrap();
    soundtrack.play().unwrap();
    soundtrack.set_loops(-1).unwrap();

    // Ship
    let mut ship = Ship::new(&texture_creator, &mixer);

    // Vectors
    let mut lasers = Vec::<Laser>::new();
    let mut asteroids = Vec::<Asteroid>::new();

    let mut debri_timer: f32 = DEBRI_FREQUENCY;

    
    // Main Loop
    // ---------------
    while game_running 
    {
        // Polls Events
        // --------------------
        poll_events(&mut event_pump, &mut game_event_state);


        // Updates
        // ------------------

        // Main Game
        if game_event_state.game_stop {game_running = false};

        // Ship
        ship.fire_on = game_event_state.ship_fire_on;
        ship.rotation_direction = game_event_state.ship_rotation_direction.clone();
        ship.update();
        if game_event_state.ship_fire_laser
        {
            match ship.fire(&texture_creator)
            {
                None => {}
                Some(laser) => {lasers.push(laser);}
            }
            game_event_state.ship_fire_laser = false;
        }

        // Lasers
        for laser in &mut lasers {laser.update();}
        lasers = lasers.into_iter().filter(|l| !l.to_destroy).collect();

        // Asteroids
        for asteroid in &mut asteroids {asteroid.update();}
        asteroids = asteroids.into_iter().filter(|a| !a.to_destroy).collect();

        // Level
        update_level(&texture_creator, &mut debri_timer, &mut asteroids);



        // Game logic
        // -------------------



        // Draw 
        // ------------------
        
        // Clear the board 
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        
        // Draw background
        draw(&mut canvas, &background, FRect{x: 0.0, y: 0.0, w: background.width() as f32, h: background.height() as f32}, None, None);

        // Draw Ship
        draw(&mut canvas, &ship.sprite_texture, FRect{x: ship.x, y: ship.y, w: SHIP_SIZE, h: SHIP_SIZE}, Some(ship.angle), Some(ship.body_rect));
        if ship.fire_on 
        {
            draw(&mut canvas, &ship.sprite_texture, FRect{x: ship.x, y: ship.y, w: SHIP_SIZE, h: SHIP_SIZE}, Some(ship.angle), Some(ship.fire_rect));
        }
        
        // Draw lasers
        for laser in &mut lasers
        {
            draw(&mut canvas, &laser.sprite_texture, FRect{x: laser.x, y: laser.y, w: LASER_WIDTH, h: LASER_HEIGHT}, Some(laser.angle), None);
        }

        // Draw asteroids
        for asteroid in &mut asteroids
        {
            draw(&mut canvas, &asteroid.sprite_texture, FRect{x: asteroid.x, y: asteroid.y, w: asteroid.size, h: asteroid.size},
                 Some(asteroid.angle),  Some(asteroid.sprite_rect));
        }

        // Present and cap at 60fps
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

use sdl3::pixels::Color;
use sdl3::event::Event;
use sdl3::EventPump;
use sdl3::keyboard::Keycode;
use sdl3::render::{FRect, Texture, TextureCreator, Canvas};
use sdl3::video::{Window, WindowContext};
use sdl3::mixer::Mixer;
use sdl3::image::LoadTexture;


mod ship;
use ship::ship::{Ship, SHIP_SIZE, RotationDirection};

mod laser;
use laser::laser::{Laser, LASER_WIDTH, LASER_HEIGHT};

mod asteroid;
use asteroid::asteroid::Asteroid;

mod target;
use target::target::Target;

mod score_info;
use score_info::score_info::{ScoreInfo, SCORE_INFO_SIZE};

use std::time::Duration;



pub static SCREEN_WIDTH: f32 = 1000.0;
pub static SCREEN_HEIGHT: f32 = 500.0;

pub static MAIN_SPEED: f32 = 3. ; // we love lucky number here :3
pub static DEBRI_FREQUENCY: f32 = 30.; // this value is in frame lol (one debri every 30 frames)

pub static HEALTH_SIZE: f32 = 64.; // who doesn't like lucky number ?



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

struct CircleCollider
{
    x: f32,
    y: f32,
    r: f32
}

fn collide_circle(c1: CircleCollider, c2: CircleCollider) -> bool
{
    ((c1.x - c2.x).powi(2) + (c1.y - c2.y).powi(2)).sqrt() < c1.r + c2.r
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

            // Rotation buttonsSCORE_INFO_SIZE
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

fn random_target<'tc>(texture_creator: &'tc TextureCreator<WindowContext>) -> Target<'tc>
{
    let new_size = rand::random_range(50..150);
    let start_y = rand::random_range(new_size..(500 - new_size));
    let new_type = rand::random_range(0..3);
    Target::new(texture_creator, start_y as f32, new_size as f32, new_type)
}

fn update_level<'tc>(texture_creator: &'tc TextureCreator<WindowContext>, debri_timer: &mut f32, asteroids: &mut Vec<Asteroid<'tc>>, targets: &mut Vec<Target<'tc>>)
{
    *debri_timer -= 1.;
    if *debri_timer <= 0.
    {
        let number = rand::random_range(0..10);
        if number == 1 {targets.push(random_target(texture_creator));}
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
        .resizable()
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

    // TTF INIT
    // ------------
    let ttf_context = sdl3::ttf::init().unwrap();
    let pixel_font = ttf_context.load_font("data/fonts/m5x7.ttf", 50.).unwrap();


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

    let target_break = mixer.load_audio("data/music/break.wav", true).unwrap();


    // Ship
    let mut ship = Ship::new(&texture_creator, &mixer);

    // Vectors
    let mut lasers = Vec::<Laser>::new();
    let mut asteroids = Vec::<Asteroid>::new();
    let mut targets = Vec::<Target>::new();
    let mut score_infos = Vec::<ScoreInfo>::new();

    // random variables
    let mut debri_timer: f32 = DEBRI_FREQUENCY;
    let mut score: Box<i32> = Box::new(0); // apparently score has to be contenerize because it makes copy everywhere else because it's a primitive

    // Health png
    let health_texture = texture_creator.load_texture("data/images/life.png").unwrap();

    
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
        // ---------
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

        // Game over
        if ship.to_destroy
        {
            lasers.clear();
            asteroids.clear();
            targets.clear();
            score_infos.clear();

            *score = 0;

            ship = Ship::new(&texture_creator, &mixer);

            soundtrack.play().unwrap();
        }


        // Lasers
        for laser in &mut lasers {laser.update();}
        lasers = lasers.into_iter().filter(|l| !l.to_destroy).collect();

        // Asteroids
        for asteroid in &mut asteroids {asteroid.update();}
        asteroids = asteroids.into_iter().filter(|a| !a.to_destroy).collect();

        // Targets
        for target in &mut targets {target.update();}
        targets = targets.into_iter().filter(|t| !t.to_destroy).collect();

        // ScoreInfos
        for score_info in &mut score_infos  {score_info.update();}
        score_infos = score_infos.into_iter().filter(|si| !si.to_destroy).collect();

        // Level
        update_level(&texture_creator, &mut debri_timer, &mut asteroids, &mut targets);



        // Collisions
        // -------------------

        // Lasers & Asteroids
        for laser in &mut lasers
        {
            for asteroid in &mut asteroids
            {
                if collide_circle(laser.get_circle_collider(), asteroid.get_circle_collider()) {laser.destroy();}
            }
        }

        // Laser & Targets
        for laser in &mut lasers
        {
            for target in &mut targets
            {
                if collide_circle(laser.get_circle_collider(), target.get_circle_collider())
                {
                    *score += target.get_value(); // score semble être une copie ici :/
                    laser.destroy();
                    target.destroy();
                    mixer.play_audio(&target_break).unwrap();

                    // Spawn score info
                    score_infos.push(ScoreInfo::new(&texture_creator, &pixel_font, target.x + target.size/2., target.y + target.size/2., target.get_value()));
                }
            }
        }

        // Ship & Asteroids
        for asteroid in & mut asteroids
        {
            if collide_circle(ship.get_circle_collider(), asteroid.get_circle_collider())
            {
                ship.damage();
                if ship.blowing_up {soundtrack.stop(0).unwrap();}
            }
        }


        // Draw 
        // ------------------
        
        // Clear the board 
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        
        // Draw background
        draw(&mut canvas, &background, FRect{x: 0.0, y: 0.0, w: background.width() as f32, h: background.height() as f32}, None, None);

        // Draw Ship
        if ship.visible
        {
            draw(&mut canvas, &ship.sprite_texture, FRect{x: ship.x, y: ship.y, w: SHIP_SIZE, h: SHIP_SIZE}, Some(ship.angle), Some(ship.body_rect));
            if ship.fire_on
            {
                draw(&mut canvas, &ship.sprite_texture, FRect{x: ship.x, y: ship.y, w: SHIP_SIZE, h: SHIP_SIZE}, Some(ship.angle), Some(ship.fire_rect));
            }
        }

        
        // Draw lasers
        for laser in &mut lasers
        {
            draw(&mut canvas, &laser.sprite_texture, FRect{x: laser.x, y: laser.y, w: LASER_WIDTH, h: LASER_HEIGHT}, Some(laser.angle), None);
        }

        // Draw targets
        for target in &mut targets
        {
            draw(&mut canvas, &target.sprite_texture, FRect { x: target.x, y: target.y, w: target.size, h: target.size}, None, None);
        }

        // Draw asteroids
        for asteroid in &mut asteroids
        {
            draw(&mut canvas, &asteroid.sprite_texture, FRect{x: asteroid.x, y: asteroid.y, w: asteroid.size, h: asteroid.size},
                 Some(asteroid.angle),  Some(asteroid.sprite_rect));
        }

        // UI
        // --------------

        // Score infos
        for score_info in &mut score_infos
        {
            draw(&mut canvas, &score_info.score_texture, FRect { x: score_info.x, y: score_info.y, w: SCORE_INFO_SIZE, h: SCORE_INFO_SIZE}, None, None);
        }


        // Score
        let score_text = format!("score : {0}", score);
        let score_surface = pixel_font.render(&score_text).solid(Color::RGBA(0, 255, 0, 255)).unwrap();
        let score_texture = texture_creator.create_texture_from_surface(&score_surface).unwrap();
        draw(&mut canvas, &score_texture, FRect { x: 450., y: 0., w: score_texture.width() as f32, h: score_texture.height() as f32}, None, None);


        // Health
        for i in 0..ship.health
        {
            let dst_rect = FRect { x: HEALTH_SIZE * i as f32, y: 1., w: HEALTH_SIZE, h: HEALTH_SIZE };
            draw(&mut canvas, &health_texture, dst_rect, None, None);

        }


        // Present and cap at 60fps
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

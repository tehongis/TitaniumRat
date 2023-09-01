
//  cargo run --release

use sfml::graphics::{Color, RenderTarget, RenderWindow,Sprite, Texture, Transformable, Font,Text};
use sfml::audio::{Music};

//use sfml::audio::{Sound, SoundBuffer, SoundStatus};
use sfml::system::{Clock, sleep, Time,Vector2f};
use sfml::window::{Event, Style};
//use std::ptr::null;
use std::f32::consts::PI;

fn main() {
    
    let mut window = RenderWindow::new(
        (800, 600),
        "SFML Example",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_framerate_limit(60);
    window.set_vertical_sync_enabled(true);

    // Load a music to play
    let mut music = Music::from_file("Roboxel - Space Music.ogg").unwrap();    
    
    // Lataa sprite kuva
    let texture = Texture::from_file("colored_packed.png").unwrap();
    
    // Luo sprite
    let mut sprite = Sprite::with_texture(&texture);
    
    // Aseta sprite sijainti
    // sprite.set_position(Vector2f::new(100.0, 100.0));
    
    let mut count_a = 0.0;
    let mut count_b = 0.5;
    
    music.play();

    let mut clock = Clock::start();
    let mut fps = 0;
    let mut fps_clock = Clock::start();

//    let font = Font::GetDefaultFont();
    let font = Font::from_file("/usr/share/fonts/truetype/freefont/FreeMono.ttf").unwrap();

    let mut fps_string = String::from("No FPS count yet.");

    while window.is_open() {

        let _elapsed_time = clock.restart();
        fps += 1;
        if fps_clock.elapsed_time().as_seconds() >= 1.0 {            
            fps_string = format!("FPS: {}", fps);
            fps = 0;
            fps_clock.restart();
        }

        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                Event::KeyPressed { code, .. } => {
                    match code {
                        sfml::window::Key::Escape => {
                            window.close();
                            println!("Exit!");
                        },
                        sfml::window::Key::Left => {
                            println!("Left!");
                        },
                        sfml::window::Key::Right => {
                            println!("Right!");
                        },
                        sfml::window::Key::Up => {
                            println!("Up!");
                        },
                        sfml::window::Key::Down => {
                            println!("Down!");
                        },
                        sfml::window::Key::Space => {
                            println!("Space!");
                        },
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        
        window.clear(Color::BLACK);
           
        // Päivitä lohikäärmeen sijainti
        //sprite.set_position(Vector2f::new(1.0, 1.0));
        
        count_a += 0.01;
        if count_a > PI*2.0 {
            count_a -= PI*2.0;
        };
        
        count_b += 0.011;
        if count_b > PI*2.0 {
            count_b -= PI*2.0;
        };
        
        let locx = 300.0 * count_a.sin();
        let locy = 300.0 * count_b.sin();
        
        let sprite_location = Vector2f::new(locx, locy);
        sprite.set_position(sprite_location);
        window.draw(&sprite);

        let mut fps_text = Text::new(&fps_string, &font, 20);
        fps_text.set_fill_color(Color::GREEN);
        
        fps_text.set_outline_color(Color::rgba(55,55,55,200));
        fps_text.set_outline_thickness(5.0);
        fps_text.set_position(Vector2f::new(20.0, 20.0));
        window.draw(&fps_text);

        window.display();
        
        sleep(Time::milliseconds(10));
        
    }
}

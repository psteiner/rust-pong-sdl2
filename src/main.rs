mod game;
mod ui;

mod prelude {
    pub use sdl2::event::Event;
    pub use sdl2::keyboard::Keycode;
    pub use sdl2::pixels::Color;
    pub use sdl2::rect::Point;
    pub use sdl2::rect::Rect;
    pub use sdl2::render::Canvas;
    pub use sdl2::video::Window;
    
    pub use std::time::Duration;

    pub use crate::game::Game;
    pub use crate::ui::*;

    pub const WINDOW_WIDTH: u32 = 800;
    pub const WINDOW_HEIGHT: u32 = 600;
    pub const SCREEN_MARGIN: i32 = 10;
    pub const COURT_HEIGHT: u32 = WINDOW_HEIGHT - SCREEN_MARGIN as u32 * 2;
    pub const COURT_WIDTH: u32 = WINDOW_WIDTH - SCREEN_MARGIN as u32 * 2;
    pub const RACKET_HEIGHT: u32 = WINDOW_HEIGHT / 8;
    pub const RACKET_WIDTH: u32 = 10;
    pub const RACKET_CENTRE: i32 =
        WINDOW_HEIGHT as i32 / 2 - RACKET_HEIGHT as i32 / 2;
    pub const BALL_SIZE: u32 = 10;
}

use prelude::*;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-pong", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.set_draw_color(Color::BLUE);
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    let mut game = Game::new();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    game.start();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => game.player.move_up(),
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => game.player.move_down(),
                _ => {}
            }
        }

        game.update();
        draw_game(&mut canvas, &game);
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
    Ok(())
}

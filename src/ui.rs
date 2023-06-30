use crate::prelude::*;

/* 
pub fn run() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-pong", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.set_draw_color(Color::BLACK);
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
        canvas.clear();
        draw_game();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
    Ok(())
}
*/

pub fn draw_game(canvas: &mut Canvas<Window>) {
    draw_net();
    draw_score();
    canvas.clear();
    canvas.present();
}

fn draw_score() {}

fn draw_net() {}

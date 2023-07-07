use crate::prelude::*;

pub fn draw_game<T>(canvas: &mut Canvas<Window>, game: &Game<T>) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    draw_net(canvas);
    draw_score(canvas, game);
    draw_ball(canvas, game);
    draw_player_paddle(canvas, game);
    draw_opponent_paddle(canvas, game);
    canvas.present();
}

fn draw_net(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::WHITE);
    const DASH_WIDTH: u32 = 8;
    let x = (WINDOW_WIDTH / 2) - (DASH_WIDTH / 2);
    const DASH_HEIGHT: u32 = 24;
    const DASH_COUNT: u32 = WINDOW_HEIGHT / DASH_HEIGHT;
    const DASH_GAP: u32 = DASH_HEIGHT / 2;

    for i in 0..DASH_COUNT {
        let y = (i * DASH_HEIGHT) as i32 + DASH_GAP as i32 / 2;
        canvas
            .fill_rect(Rect::new(
                x as i32,
                y,
                DASH_WIDTH,
                DASH_HEIGHT - DASH_GAP,
            ))
            .expect("Failed to draw net dash");
    }
}

fn draw_score<T>(canvas: &mut Canvas<Window>, game: &Game<T>) {
    canvas.set_draw_color(Color::WHITE);
}

fn draw_ball<T>(canvas: &mut Canvas<Window>, game: &Game<T>) {
    canvas.set_draw_color(Color::WHITE);
    canvas
        .fill_rect(Rect::new(
            game.ball.x,
            game.ball.y,
            game.ball.size,
            game.ball.size,
        ))
        .expect("Failed to draw ball");
}

fn draw_player_paddle<T>(canvas: &mut Canvas<Window>, game: &Game<T>) {}
fn draw_opponent_paddle<T>(canvas: &mut Canvas<Window>, game: &Game<T>) {}

use crate::prelude::*;

pub fn draw_game(canvas: &mut Canvas<Window>, game: &Game) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    draw_net(canvas);
    draw_score();
    canvas.present();
}

fn draw_net(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(32, 32, 32));
    canvas
        .fill_rect(Rect::new(SCREEN_MARGIN, SCREEN_MARGIN, COURT_WIDTH, COURT_HEIGHT))
        .expect("Failed to draw court");
    canvas.set_draw_color(Color::WHITE);
    const DASH_WIDTH: u32 = 8;
    let x = WINDOW_WIDTH / 2 - DASH_WIDTH / 2;
    const DASH_HEIGHT: u32 = 24;
    const DASH_COUNT: u32 = COURT_HEIGHT / DASH_HEIGHT;
    const DASH_GAP: u32 = DASH_HEIGHT / 2;

    for i in 0..DASH_COUNT {
        let y = (i * DASH_HEIGHT) as i32 + SCREEN_MARGIN + DASH_GAP as i32 / 2;
        canvas
            .fill_rect(Rect::new(x as i32, y, DASH_WIDTH, DASH_HEIGHT - DASH_GAP))
            .expect("Failed to draw net dash");
    }
}

fn draw_score() {}

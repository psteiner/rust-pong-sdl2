use crate::prelude::*;

pub fn draw_game(canvas: &mut Canvas<Window>, game: &Game) {
    draw_net();
    draw_score();
    canvas.clear();
    canvas.present();
}

fn draw_score() {}

fn draw_net() {}

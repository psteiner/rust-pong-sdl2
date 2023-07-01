use crate::prelude::*;

/*
Notes on the original Atari Pong cabinet game play from these videos :
https://youtu.be/_MxlPkD7mEw?t=909
https://youtu.be/ut6Rh-rmGAo?t=1204

    Idle:
        - display last score, ball bounces randomly around screen, no paddles or
        sound effects

    Playing:
        - game starts when a coin is inserted
        - After the coin is inserted, the screen 'flips' as the game initializes
        - the scores are reset to '0'
        - after two or three seconds, the ball is served from the centre line

    Over:
        - first player to 11

    Sounds:
        - low-pitched 'boop' when ball rebounds from top or bottom edge
        - higher-pitched 'bip' when paddle intercepts ball
        - longer, higher-pitched 'buuuzzz' when ball scores

        - volley speeds up the longer the ball is in play
        - three speed levels
*/
enum Mode {
    Idle,
    Playing,
    Over,
}

pub struct Paddle {
    pub x: i32,
    pub y: i32,
    pub height: u32,
    pub width: u32,
}

impl Paddle {
    pub fn new(x: i32, y: i32, height: u32, width: u32) -> Paddle {
        Paddle {
            x,
            y,
            height,
            width,
        }
    }

    pub fn move_up(&self) {}
    pub fn move_down(&self) {}
}

pub struct Ball {
    pub x: i32,
    pub y: i32,
    pub size: u32,
}

impl Ball {
    pub fn new(x: i32, y: i32, size: u32) -> Ball {
        Ball { x, y, size }
    }

    pub fn update() {}
}

pub struct Game {
    mode: Mode,
    pub player: Paddle,
    pub opponent: Paddle,
    pub ball: Ball,
}

impl Game {
    pub fn new() -> Game {
        Game {
            mode: Mode::Idle,
            player: Paddle::new(
                SCREEN_MARGIN,
                RACKET_CENTRE,
                RACKET_HEIGHT,
                RACKET_WIDTH,
            ),
            opponent: Paddle::new(
                WINDOW_WIDTH as i32 - SCREEN_MARGIN - RACKET_WIDTH as i32,
                RACKET_CENTRE,
                RACKET_HEIGHT,
                RACKET_WIDTH,
            ),
            ball: Ball::new(
                WINDOW_WIDTH as i32 / 2,
                WINDOW_HEIGHT as i32 / 2,
                BALL_SIZE,
            ),
        }
    }

    pub fn update(&mut self) {}

    pub fn start(&mut self) {
        self.mode = Mode::Playing;
    }
}

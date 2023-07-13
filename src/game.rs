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
        - after two or three seconds, the ball is served from a random location
            and angle on the net

    Over:
        - first player to 11

    Sounds:
        - low-pitched 'boop' when ball rebounds from top or bottom edge
        - higher-pitched 'bip' when paddle intercepts ball
        - longer, higher-pitched 'buzz' when ball scores

*/
#[derive(Debug, PartialEq)]
enum State {
    Idle,
    Playing,
    Over,
}

pub struct Game {
    state: State,
    pub player: Paddle,
    pub opponent: Paddle,
    pub ball: Ball,
}

impl Game {
    pub fn new() -> Self {
        Self {
            state: State::Idle,
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
            ball: Ball::new(),
        }
    }

    pub fn update(&mut self) {
        if self.state == State::Playing {
            self.player.update();
            self.opponent.update();
        }
        self.ball.update();
    }

    pub fn start(&mut self) {
        self.state = State::Playing;
        println!("State: {:?}", self.state);
    }
}

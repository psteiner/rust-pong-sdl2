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
        - longer, higher-pitched 'buuuzzz' when ball scores

*/

pub struct Paddle {
    pub x: i32,
    pub y: i32,
    pub height: u32,
    pub width: u32,
    pub score: u32,
}

impl Paddle {
    pub fn new(x: i32, y: i32, height: u32, width: u32) -> Paddle {
        Paddle {
            x,
            y,
            height,
            width,
            score: 0,
        }
    }

    pub fn move_up(&self) {}
    pub fn move_down(&self) {}
}

/*  Ball
   - new ball starts from random y position at random angle
   - winner serves
   - volley speeds up the longer the ball is in play
   - three speed levels
*/
pub struct Ball {
    pub x: i32,
    pub y: i32,
    pub size: u32,
}

impl Ball {
    pub fn new() -> Ball {
        Ball {
            x: WINDOW_WIDTH as i32 / 2 - BALL_SIZE as i32 / 2,
            y: WINDOW_HEIGHT as i32 / 2,
            size: BALL_SIZE,
        }
    }

    pub fn update() {}
}

pub struct Game<State> {
    state: State,
}

impl Game<Idle> {
    pub fn new() -> Self {
        Game { state: Idle::new() }
    }
    pub fn start(&self){}
    pub fn update(&self) {}
}

pub struct Idle {
    pub ball: Ball,
}

impl Idle {
    pub fn new() -> Self {
        Idle { ball: Ball::new() }
    }
}


impl From<Game<Over>> for Game<Idle> {
    fn from(game: Game<Over>) -> Game<Idle> {
        Game {
            state: Idle {
                ball: Ball::new()
            }
        }
    }
}

pub struct Playing {
    pub player: Paddle,
    pub opponent: Paddle,
    pub ball: Ball,
}

impl Game<Playing> {
    pub fn update(&self) {}
}

impl From<Game<Idle>> for Game<Playing> {
    fn from(game: Game<Idle>) -> Game<Playing> {
        Game {
            state: Playing {
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
            },
        }
    }
}

pub struct Over {
    pub player: Paddle,
    pub opponent: Paddle,
}

impl Game<Over> {
    pub fn update(&self) {}
}


impl From<Game<Playing>> for Game<Over> {
    fn from(game: Game<Playing>) -> Game<Over> {
        Game {
            state: Over {
                player: game.state.player,
                opponent: game.state.opponent
            }
        }
    }
}
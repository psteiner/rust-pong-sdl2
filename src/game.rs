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

pub trait Update {
    fn update(&mut self) {}
}

pub struct Player {
    score: u32,
    paddle: Paddle,
}

impl Update for Player {
    fn update(&mut self) {}
}

pub struct Opponent {
    score: u32,
    paddle: Paddle,
}

impl Update for Opponent {
    fn update(&mut self) {}
}

pub struct Game<State> {
    state: State,
}

impl Game<Idle> {
    pub fn new() -> Self {
        Game { state: Idle::new() }
    }
    pub fn start(self) -> Game<Playing> {
        Game::from(self)
    }
}

impl Update for Game<Idle> {
    fn update(&mut self) {
        
    }
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
            state: Idle { ball: Ball::new() },
        }
    }
}

pub struct Playing {
    pub player: Player,
    pub opponent: Opponent,
    pub ball: Ball,
}

impl Update for Game<Playing> {
    fn update(&mut self) {}
}

impl From<Game<Idle>> for Game<Playing> {
    fn from(game: Game<Idle>) -> Game<Playing> {
        Game {
            state: Playing {
                player: Player {
                    paddle: Paddle::new(
                        SCREEN_MARGIN,
                        RACKET_CENTRE,
                        RACKET_HEIGHT,
                        RACKET_WIDTH,
                    ),
                    score: 0,
                },

                opponent: Opponent {
                    paddle: Paddle::new(
                        WINDOW_WIDTH as i32
                            - SCREEN_MARGIN
                            - RACKET_WIDTH as i32,
                        RACKET_CENTRE,
                        RACKET_HEIGHT,
                        RACKET_WIDTH,
                    ),
                    score: 0,
                },
                ball: Ball::new(),
            },
        }
    }
}

pub struct Over {
    pub player: Player,
    pub opponent: Opponent,
}

impl Update for Game<Over> {
    fn update(&mut self) {}
}

impl From<Game<Playing>> for Game<Over> {
    fn from(game: Game<Playing>) -> Game<Over> {
        Game {
            state: Over {
                player: game.state.player,
                opponent: game.state.opponent,
            },
        }
    }
}

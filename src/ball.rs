use crate::prelude::*;

/*  Ball
   - new ball starts from random y position at random angle
   - winner serves
   - volley speeds up the longer the ball is in play
   - three speed levels
*/
pub struct Ball {
    pub x: i32,
    pub y: i32,
    pub dx: i32,
    pub dy: i32,
    pub size: u32,
}

impl Ball {
    pub fn new() -> Ball {
        Ball {
            x: WINDOW_WIDTH as i32 / 2 - BALL_SIZE as i32 / 2,
            y: WINDOW_HEIGHT as i32 / 2,
            dx: 3,
            dy: 3,
            size: BALL_SIZE,
        }
    }
}

impl Update for Ball {
    fn update(&mut self) {
        
    }
}
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
    pub fn new(x: i32, y: i32, size: u32) -> Ball {
        Ball { x, y, dx: 3, dy: 3, size }
    }

    pub fn update(&mut self) {
        self.x += self.dx;
        self.y += self.dy;

        if self.x < 0 {
            
        }
    }
}
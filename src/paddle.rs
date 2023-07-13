use crate::prelude::*;

pub struct Paddle {
    pub x: i32,
    pub y: i32,
    pub height: u32,
    pub width: u32,
    pub score: u32
}

impl Paddle {
    pub fn new(x: i32, y: i32, height: u32, width: u32) -> Paddle {
        Paddle {
            x,
            y,
            height,
            width,
            score: 0
        }
    }

    pub fn move_up(&self) {}
    pub fn move_down(&self) {}
    pub fn update(&self) {}
}
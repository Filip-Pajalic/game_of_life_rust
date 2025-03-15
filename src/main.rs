mod game;

use macroquad::prelude::*;
use crate::game::World;


const WINDOW_WIDTH: usize = 1920;
const WINDOW_HEIGHT: usize = 1080;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Game of life"),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {

    let mut world = World::new();

    loop {
        clear_background(BLACK);
        world.update();


        next_frame().await

    }
}

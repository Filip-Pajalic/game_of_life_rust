use std::cmp::PartialEq;
use std::time::{Duration, Instant};
use macroquad::color::GREEN;
use macroquad::shapes::{draw_rectangle, draw_rectangle_lines};
use rand::Rng;
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

const DEBUG: bool = false;

#[derive(PartialEq,Eq,Debug,Clone)]
pub struct World{
    width: usize,
    height: usize,
    grid_size: usize,
    cells: Vec<Cell>,
    update_timer: Instant,
    update_time: Duration
}

#[derive(PartialEq,Eq,Debug,Clone)]
enum Cell {
    ALIVE,
    DEAD
}

impl World{
    pub fn new() -> Self{
        let mut world =World{
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            grid_size: 10,
            cells: vec!{},
            update_timer: Instant::now(),
            update_time: Duration::from_millis(0)
        };
        let grid_y = world.height/world.grid_size;
        let grid_x= world.width/world.grid_size;
        for _ in 0..grid_x {
            for _ in 0..grid_y {
                let mut rng = rand::rng();
                let mut cell = Cell::DEAD;
                if rng.random_bool(0.5){
                    cell = Cell::ALIVE;
                }
                world.cells.push(cell);
            }
        }
        world
    }

    pub fn update(&mut self){
        if DEBUG{
            self.draw_debug_grid()
        }
        if self.update_timer.elapsed() >= self.update_time{
            self.update_cell_state();
            self.update_timer = Instant::now();
        }
        self.draw_live_cells();
    }
    /*Every cell interacts with its eight neighbours, which are the cells that are horizontally, vertically, or diagonally adjacent. At each step in time, the following transitions occur:

    Any live cell with fewer than two live neighbours dies, as if by underpopulation.
    Any live cell with two or three live neighbours lives on to the next generation.
    Any live cell with more than three live neighbours dies, as if by overpopulation.
    Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
    The initial pattern constitutes the seed of the system.*/
    fn update_cell_state(&mut self){
        let mut next_iteration = self.cells.clone();
        let grid_y = self.height/self.grid_size;
        let grid_x= self.width/self.grid_size;
        for x in 0..grid_x {
            for y in 0..grid_y {
                let index = y * grid_x + x;
                let cell = &self.cells[index];
                let count = self.get_neighbour_count(x,y);
                let next_cell = match (count, &cell) {
                    (2, Cell::ALIVE) | (3, Cell::ALIVE) => Cell::ALIVE,
                    (3, Cell::DEAD) => Cell::ALIVE,
                    _ => Cell::DEAD,
                };
               next_iteration[index] = next_cell;
            }
        }
        self.cells = next_iteration;
    }

     fn get_neighbour_count(&self, x :usize, y: usize) -> usize {
         let grid_y = self.height/self.grid_size;
         let grid_x= self.width/self.grid_size;
         let mut count = 0;
            for delta_x  in [grid_x -1, 0, 1].iter().cloned() {
                for delta_y in [grid_y -1 , 0 , 1].iter().cloned(){

                    if delta_x == 0 && delta_y == 0 {
                        continue;
                    }
                    let neighbour_x = (x + delta_x ) % grid_x;
                    let neighbour_y = (y + delta_y ) % grid_y;

                    let index = neighbour_y * grid_x + neighbour_x;
                    if self.cells[index] == Cell::ALIVE{
                        count +=1;
                    }
                }
         }
         count
     }

    fn draw_debug_grid(&self){

        let grid_y = self.height/self.grid_size;
        let grid_x= self.width/self.grid_size;
        for x in 0..grid_x {
            for y in 0..grid_y {
                draw_rectangle_lines((x * self.grid_size) as f32, (y * self.grid_size) as f32, self.grid_size as f32, self.grid_size as f32, 1.0, GREEN);
            }
        }
    }



    fn draw_live_cells(&self){
        let grid_y = self.height/self.grid_size;
        let grid_x= self.width/self.grid_size;
        for x in 0..grid_x {
            for y in 0..grid_y {
                let index = y * grid_x + x;
                if self.cells[index] == Cell::ALIVE{
                    draw_rectangle((x * self.grid_size) as f32, (y * self.grid_size) as f32, self.grid_size as f32, self.grid_size as f32,  GREEN);
                }
            }
        }
    }
}
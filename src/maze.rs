/*
* my implementation of the hunt & kill algorythm
blog that I based things off of:
http://weblog.jamisbuck.org/2011/1/24/maze-generation-hunt-and-kill-algorithm

! A lot of this is going to be terrible. I need to fix it.
! I just wanted something that would get the job done, so I could continue getting everything to work.

* steps for algorythm:
    1. Initialize the grid of cells (all walls initially).
    2. Randomly select a starting cell.
    3. Set the starting cell as visited and carve a path.
    4. While there are unvisited cells:
        a. If there are unvisited neighbors for the current cell:
            i. Randomly select a neighbor.
            ii. Remove the wall between the current cell and the neighbor.
            iii. Move to the neighbor and continue.
        b. If there are no unvisited neighbors for the current cell:
            i. Hunt for an unvisited cell.
            ii. Move to the unvisited cell and start a new killing phase.
    5. End when all cells are visited.
*/

#[allow(unused_imports)]
use std::option;
use rand::random;
use crate::{MAZE_HEIGHT, MAZE_WIDTH};

pub fn make_maze() -> Vec<Tile> {
    let mut maze: Vec<Tile> = Vec::new();
    for x in 0..MAZE_WIDTH {
        for z in 0..MAZE_HEIGHT {
            maze.push(Tile::new_tile(x as f32, z as f32, random()))
        }
    }
    maze
}

pub struct Tile {
    pub x: f32,
    pub z: f32,
    pub is_wall: bool,
    pub is_path: bool,
    touched: bool,
    neigbors: Vec<Tile>
}

impl Tile {
    fn new_tile(x: f32, z: f32, is_wall: bool) -> Self {
        Tile { 
            x, 
            z, 
            is_wall, 
            is_path: true, 
            touched: false, 
            neigbors: Vec::new()
        }
    }
}
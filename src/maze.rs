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

use bevy::math::Vec2;
#[allow(unused_imports)]
use rand::{random, thread_rng, seq::SliceRandom};
use crate::{MAZE_HEIGHT, MAZE_WIDTH};


#[allow(dead_code)]
pub struct Maze {
    pub tiles: Vec<Tile>,
    pub width: u64,
    pub height: u64,
    pub start_position: Vec2,
    pub algorythm: String,
    illumination_percent: f32,
}

pub enum MazeAlgorithm {
    HuntAndKill,
    Random,
    RandomIllumination,
}

impl Maze{
    pub fn make_maze (maze_type: MazeAlgorithm) -> Self {
        match maze_type {
            MazeAlgorithm::HuntAndKill => Maze::hunt_and_kill(MAZE_WIDTH, MAZE_HEIGHT),
            MazeAlgorithm::Random => Maze::random(MAZE_WIDTH, MAZE_HEIGHT),
            MazeAlgorithm::RandomIllumination => Maze::random_illumination(MAZE_WIDTH, MAZE_HEIGHT),
        }
    }

    fn find_neighbors (&mut self) {
        let directions: [Vec2; 4] = [
            Vec2{ x: -1.0, y: 0.0 },    // Left
            Vec2{ x: 1.0, y: 0.0},      // Right
            Vec2{ x: 0.0, y: 1.0},      // Up
            Vec2{ x: 0.0, y: -1.0},     // Down
        ];
        for tile in self.tiles.iter_mut() {
            for direction in directions {
                let spot = tile.position + direction;
                if !(spot.x > self.width as f32) && !(spot.x < 0.0) && !(spot.y > self.height as f32) && !(spot.y < 0.0) {
                    // if the thingymabobber is valid, put it into the other thingymabobber
                    tile.neighbors.push(tile.position + direction)
                }
            }
        }
    }

    /* 
    * making this function feels kinda like a fever dream, I don't know what, or why it is.
    pub fn return_wall_places(&self) -> Vec<Vec2> {
        let mut walls: Vec<Vec2> = Vec::new();
        for tile in &self.tiles {
            if tile.is_wall {
                walls.push(tile.position);
            }
        }
        walls
    }
    */

    fn new_maze (width: u64, height: u64, algorythm: String) -> Self {
        let mut tiles = Vec::new();
        for x in 0..height {
            for z in 0..width {
                tiles.push(Tile::new_tile(x as f32, z as f32, true, false))
            }
        }
        let mut maze: Maze = Maze { 
            tiles, 
            width, 
            height, 
            start_position: Vec2 { x: 0.5, y: 0.5 }, 
            algorythm, 
            illumination_percent: 1.0 
        };
        maze.find_neighbors();
        maze
    }

    fn hunt_and_kill (width: u64, height: u64) -> Maze {
        let mut maze = Maze::new_maze(width, height, "Hunt and Kill".to_string());
        for mut tile in &mut maze.tiles {
            //tile.toggle_wall();
        }
        maze
    }

    fn random (width: u64, height: u64) -> Maze {
        let mut maze = Maze::new_maze(width, height, "random".to_string());
        for tile in &mut maze.tiles {
            if random() {
                tile.toggle_wall();
            }
        }
        for tile in &maze.tiles {
            if !tile.is_wall {
                maze.start_position = tile.position;
                break
            }
        }
        maze
    }

    fn random_illumination (width: u64, height: u64) -> Maze {
        let mut maze = Maze::new_maze(width, height, "Random w/ illumination".to_string());
        for tile in &mut maze.tiles {
            if random() {
                tile.toggle_wall();
            }
            if random() {
                tile.toggle_illuminated();
            }
        }
        for tile in &maze.tiles {
            if !tile.is_wall {
                maze.start_position = tile.position;
                break
            }
        }
        maze
    }

}

#[allow(dead_code)]
pub struct Tile {
    pub position: Vec2,
    pub is_wall: bool,
    is_path: bool,
    pub illuminated: bool,
    num_adjacent_paths: u8,
    touched: bool,
    neighbors: Vec<Vec2>
}

impl Tile {
    fn new_tile (x: f32, y: f32, is_wall: bool, is_path: bool) -> Self {
        Tile { 
            position: Vec2 { x, y },
            is_wall, 
            is_path,
            illuminated: false,
            num_adjacent_paths: 0,
            touched: false, 
            neighbors: Vec::new()
        }
    }

    fn toggle_touched(&mut self) {
        self.touched = !self.touched
    }

    fn toggle_wall(&mut self) {
        self.is_wall = !self.is_wall
    }

    fn toggle_illuminated(&mut self) {
        self.illuminated = !self.illuminated
    }
}

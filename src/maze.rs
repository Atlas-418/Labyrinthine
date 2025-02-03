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
#[allow(unused_imports)]
use crate::{MAX_QUESTIONS, MAZE_HEIGHT, MAZE_WIDTH};


// * struct containing all of the data for a maze
#[derive(Clone)]
pub struct Maze {
    pub tiles: Vec<Tile>,
    pub width: u64,
    pub height: u64,
    pub start_position: Vec2,
    pub algorythm: String,
    pub questions: usize,
    pub choices_made: usize,
    pub morality: f64,
}

// * list of algorythms I have implemented. Used in Maze::make_maze()
pub enum MazeAlgorithm {
    HuntAndKill,
    Random,
    RandomIllumination,
}

impl Maze {

    // * chooses between algorythms using an enumerate, and then returns the created maze
    pub fn make_maze (maze_type: MazeAlgorithm) -> Self {
        match maze_type {
            MazeAlgorithm::HuntAndKill => Maze::hunt_and_kill(MAZE_WIDTH, MAZE_HEIGHT),
            MazeAlgorithm::Random => Maze::random(MAZE_WIDTH, MAZE_HEIGHT, false),
            MazeAlgorithm::RandomIllumination => Maze::random(MAZE_WIDTH, MAZE_HEIGHT, true),
        }
    }

    // * Makes a new, empty maze, with default values
    fn new (width: u64, height: u64, algorythm: String) -> Self {
        let mut tiles = Vec::new();
        for x in 0..height {
            for z in 0..width {
                tiles.push(Tile::new(x as f32, z as f32, true, false))
            }
        }
        let mut maze: Maze = Maze { 
            tiles, 
            width, 
            height, 
            start_position: Vec2 { x: 0.0, y: 0.0 }, 
            questions: 0,
            algorythm, 
            morality: 0.5,
            choices_made: 0
        };

        // * populates tile.adjacent cells with the adjacent cells' positions
        let tile_directions: [Vec2; 4] = [
            Vec2{ x: -1.0, y: 0.0 },    // Left
            Vec2{ x: 1.0, y: 0.0},      // Right
            Vec2{ x: 0.0, y: 1.0},      // Up
            Vec2{ x: 0.0, y: -1.0},     // Down
        ];
        let cell_directions: [Vec2; 4] = [
            Vec2{ x: -2.0, y: 0.0 },    // Left
            Vec2{ x: 2.0, y: 0.0},      // Right
            Vec2{ x: 0.0, y: 2.0},      // Up
            Vec2{ x: 0.0, y: -2.0},     // Down
        ];

        for tile in maze.tiles.iter_mut() {
            for direction_idx in 0..4 {
                let next_tile = tile.position + tile_directions[direction_idx];
                let next_cell = tile.position + cell_directions[direction_idx];

                if !(next_cell.x > maze.width as f32) && !(next_cell.x < 0.0) && !(next_cell.y > maze.height as f32) && !(next_cell.y < 0.0) {
                    // if the thingymabobber is valid, put it into the other thingymabobber
                    tile.adjacent.push([next_tile, next_cell])
                }
            }
        }

        maze
    }

    // * takes in a position, and a maze, and returns a mutable reference to the tile with that position
    // this entire function is reference hell. To future me, who wants to refactor this, I am sorry.
    fn get_tile_at_position (&mut self, pos: Vec2) -> &mut Tile {

        // * takes in a vec2, and returns the index at which it can be found in maze.tiles
        fn get_index_of_tile_at_position (tiles: &Vec<Tile>, pos: Vec2) -> Option<usize> {
            let mut index = None;

            for (i, tile) in tiles.iter().enumerate() {
                if tile.position == pos {
                    index = Some(i);
                }
            }
            if index != None {
                index
            } else {
                panic!("Maze::get_index_of_tile_at_position did not find a tile with the position ({x}, {y})", x = {pos.x}, y = {pos.y})
            }
        }

        // get the starting tile, through pain and suffering.
        match get_index_of_tile_at_position(&self.tiles, pos) {
            Some(idx) if idx < self.tiles.len() => {

                // this next line pretty much tries to return a tile, and if it won;t work, panics.
                // this panic *shouldn't* ever trigger, because of the match statement this is inside
                // but I don't trust my own code, and need to know when & where something goes bad.
                self.tiles.get_mut(idx).unwrap_or_else(|| {panic!("Maze::get_tile_at_position got no tile at the index {}", idx)})
            }
            _ => panic!("Maze::get_index_of_tile_at_postion gave an invalid index when given ({x}, {y})", x = {pos.x}, y = {pos.y})
        }
    }

    fn get_num_neighbors (&mut self) {

    }

    // dear jod, these function names are getting out of hand.
    fn is_tile_at_position_open (tiles: &Vec<Tile>, tile_position: Vec2) -> bool {
        let mut is_open: bool = false;
        for tile in tiles {
            if tile.position == tile_position && !tile.touched {
                is_open = true;
            }

        }
        is_open
    }

    fn toggle_tile_at_position (&mut self, pos: Vec2, attributes: Vec<&str>) {
        let tile = self.get_tile_at_position(pos);
        for attribute in attributes {
            match attribute {
                "wall" => tile.is_wall = false,//tile.toggle_wall(),
                "touched" => tile.touched = true,//tile.toggle_touched(),
                "question" => tile.toggle_touched(),
                "illuminated" => tile.toggle_illuminated(),
                _ => {}
            }
        }
    }

    //? Oh, hey, look, the rest of this impl is maze generation algorythms, and nothing else. keep it that way.

    fn hunt_and_kill (width: u64, height: u64) -> Maze {

        // * performs a random walk from a starting position, to when it can no longer continue.
        fn random_walk (maze: &mut Maze, start_pos: [Vec2; 2]) {

            let mut next_pos: [Vec2; 2] = [start_pos[0], start_pos[1]];

            loop {
                let all_tiles = maze.tiles.clone();

                // use mutable reference of maze to toggle attributes of target tiles
                maze.toggle_tile_at_position(next_pos[0], vec!["wall", "touched"]);
                maze.toggle_tile_at_position(next_pos[1], vec!["wall", "touched"]);

                // grab the next cell, use that for the rest of the loop
                let tile: &mut Tile = maze.get_tile_at_position(next_pos[1]);

                // make a list to put all of the open possibilities into, and populate it
                let mut possible_next_positions: Vec<[Vec2; 2]> = Vec::new();
                for position in tile.adjacent.clone() {
                    let coords_valid: bool = (position[1].x < maze.width as f32) && (position[1].x > 0.0) && (position[1].y < maze.height as f32) && (position[1].y > 0.0);
                    if Maze::is_tile_at_position_open(&all_tiles, position[1]) && coords_valid {    // use position[1] because we only care if the next cell is open
                        possible_next_positions.push(position);
                    }
                }

                if !possible_next_positions.is_empty() {
                    next_pos = possible_next_positions.choose(&mut thread_rng()).unwrap().clone();
                } else {
                    break
                }
            }
        }

        // * Find a new starting point for the random walk
        // if this returns None, there are no open cells
        fn get_open_cell (maze: &Maze) -> Option<[Vec2; 2]> {
            let directions: [Vec2; 4] = [
                Vec2{ x: -1.0, y: 0.0 },    // Left
                Vec2{ x: 1.0, y: 0.0},      // Right
                Vec2{ x: 0.0, y: 1.0},      // Up
                Vec2{ x: 0.0, y: -1.0}      // Down
            ];
            let mut new_start: Option<[Vec2; 2]> = None;

            let mut cells: Vec<Tile> = Vec::new();
            for tile in maze.tiles.clone() {
                if (tile.position.x - 1.0) % 2.0 == 0.0 && (tile.position.y - 1.0) % 2.0 == 0.0 {
                    cells.push(tile);
                }
            }

            for cell in cells {
                let mut touched_neigbors: Vec<[Vec2; 2]> = Vec::new();
                if Maze::is_tile_at_position_open(&maze.tiles, cell.position) {
                    for direction in directions {
                        let new_cell_postition: Vec2 = cell.position + (direction + direction);
                        let coords_valid: bool = (new_cell_postition.x < maze.width as f32) && (new_cell_postition.x > 0.0) && (new_cell_postition.y < maze.height as f32) && (new_cell_postition.y > 0.0);

                        if !Maze::is_tile_at_position_open(&maze.tiles, new_cell_postition) && coords_valid {
                            touched_neigbors.push([cell.position + direction, new_cell_postition]);
                        }
                    }
                }
                if !touched_neigbors.is_empty() {
                    new_start = Some(touched_neigbors[0]);
                }
            }
            new_start
        }

        let mut maze: Maze = Maze::new(width, height, "Hunt and Kill".to_string());

        let mut walk_start: Option<[Vec2; 2]> = Some([Vec2{x: 0.0, y: 1.0}, Vec2{x: 1.0, y: 1.0}]);

        while walk_start != None {
            random_walk(&mut maze, walk_start.unwrap());

            walk_start = get_open_cell(&maze);
        }
        
        maze.start_position = Vec2{x: 0.5, y: 1.5};

        maze
    }

    fn random (width: u64, height: u64, illuminated: bool) -> Maze {
        let mut maze = Maze::new(width, height, "random".to_string());
        for tile in &mut maze.tiles {
            if random() {
                tile.toggle_wall();
            }
            if illuminated && random() {
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

// * contains all of the data for an individual tile
#[allow(dead_code)]
#[derive(Clone)]
pub struct Tile {
    pub position: Vec2,
    pub is_wall: bool,
    is_path: bool,
    pub illuminated: bool,
    pub is_question: bool,
    num_adjacent_paths: u8,
    touched: bool,
    adjacent: Vec<[Vec2; 2]> // first entry is the neigbor in that direction, second entry is the cell in that direction
}

impl Tile {

    // * basically just does default values
    fn new (x: f32, y: f32, is_wall: bool, is_path: bool) -> Self {
        Tile { 
            position: Vec2 { x, y },
            is_wall, 
            is_path,
            illuminated: false,
            is_question: false,
            num_adjacent_paths: 0,
            touched: false, 
            adjacent: Vec::new()
        }
    }

    // * all of the toggle functions are sad, and simple, and stupid, and do exactly what they say they do. I hate them.
    fn toggle_touched(&mut self) {
        self.touched = !self.touched
    }
    
    fn toggle_wall(&mut self) {
        self.is_wall = !self.is_wall
    }
    
    fn toggle_illuminated(&mut self) {
        self.illuminated = !self.illuminated
    }

    fn toggle_question (&mut self) {
        self.is_question = !self.is_question
    }

}
  
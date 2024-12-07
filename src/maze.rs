//use rand::random;
use crate::{MAZE_HEIGHT, MAZE_WIDTH};

pub fn make_maze() -> Vec<Vec<bool>> {
    let mut maze = Maze::initialize_maze();

    maze.maze
}

/*
* my implementation of the hunt & kill algorythm
blog that I based things off of:
http://weblog.jamisbuck.org/2011/1/24/maze-generation-hunt-and-kill-algorithm

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

struct Maze {
    maze: Vec<Vec<bool>>

}

enum State {
    Touched,
    Untouched,
}

impl Maze{
    // make a vector of vectors, filled with 'true
    fn initialize_maze() -> Self {
        let mut maze: Vec<Vec<bool>> = Vec::new();
        for _ in 0..MAZE_HEIGHT {
            let mut new_row = Vec::new();
            for _ in 0..MAZE_WIDTH {
                new_row.push(true);
            }
            maze.push(new_row);
        }
    Self { maze }
    }

    fn random_cell() {
        
    }
}
use rand::{random, thread_rng, Rng};
use crate::{MAZE_HEIGHT, MAZE_WIDTH};

pub fn make_maze() -> Vec<Vec<Option<bool>>> {
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
    maze: Vec<Vec<Option<bool>>>,
}

impl Maze{
    //! Fix this! Figure out how to put state and bool into one Vec
    fn initialize_maze() -> Self {
        let mut maze = Vec::new();
        for _ in 0..MAZE_HEIGHT {
            //let mut new_row = Vec::new();
            //for _ in 0..MAZE_WIDTH {
            //    new_row.push(None);
            //}
            maze.push(vec![None; MAZE_WIDTH]);
        }
        Self { 
            maze, 
        }
    }

    fn random_untouched() -> (bool, usize, usize) {
        for i in 0..(MAZE_HEIGHT*MAZE_WIDTH) {
            let y = thread_rng().gen_range(0..MAZE_HEIGHT);
            let x = thread_rng().gen_range(0..MAZE_HEIGHT);
            if true {
                return (true, y, x);
            }
        }
        (false, 0, 0)
    }

    fn random_neighbor(y: usize, x: usize) -> (bool, usize, usize) {
        
    }
}
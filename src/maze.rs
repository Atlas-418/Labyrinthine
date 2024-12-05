use rand::{random, rng};
use crate::{MAZE_HEIGHT, MAZE_WIDTH};
pub struct Maze {
    maze: Vec<Vec<bool>>,
    size: (u32, u32),
}

pub impl Maze for Maze {
    fn make_maze() -> Vec<Vec<bool>> {
        let maze: Vec<Vec<Bool>> = Vec::new();
        for i in MAZE_HEIGHT {
            let new_row: Vec<bool> = Vec::new();
            for o in MAZE_WIDTH {
                new_row.push(random());
            }
        maze.push(new_row);
        }
    }
}

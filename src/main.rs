mod camera;
mod maze;
mod cubemesh{pub mod gen_cubemesh;}
mod questions;
mod user_interface;

use bevy::prelude::*;
use crate::user_interface::UiPlugin;
use crate::questions::{Question, give_questions};
use crate::maze::*;
use crate::camera::CameraPlugin;
use crate::cubemesh::gen_cubemesh::GenWorldPlugin;


const MAZE_WIDTH:   u64 = 35; // These both need to be odd numbers
const MAZE_HEIGHT:  u64 = 35;
#[allow(dead_code)]
const MAX_QUESTIONS: usize = 3;


#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref MAZE: Maze = Maze::make_maze(MazeAlgorithm::HuntAndKill);
    pub static ref QUESTIONS: Vec<Question> = give_questions(MAZE.questions);
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum GameState {
    #[default]
    Menu,
    World
}

fn main() {
    println!("Starting");
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_plugins(UiPlugin)
        .add_plugins(GenWorldPlugin)
        .add_plugins(CameraPlugin)
        //.add_systems(Startup, ) 
        .run();
}

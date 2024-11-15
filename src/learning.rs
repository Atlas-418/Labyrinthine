use bevy::prelude::*;

#[derive(Component)]
struct Position {   // make a struct for position. will be used in ECS
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}

pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people);
        app.add_systems(Update, greet_people);
    }
}

fn hello_world() {
    println!("Hello, world!")
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Alex".to_string())));
    commands.spawn((Person, Name("Frank".to_string())));
    commands.spawn((Person, Name("Gary".to_string())));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("Hello, {}!", name.0);
        }
    }
}

fn update_alex(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Alex" {
            name.0 = "Alex 2".to_string();
            break;
        }
    }
}

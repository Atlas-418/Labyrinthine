use bevy::prelude::*;
use crate::GameState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu)
           .add_systems(
               Update,
               button_interaction.run_if(in_state(GameState::Menu))
           )
           .add_systems(OnExit(GameState::Menu), cleanup_menu);
    }
}

fn setup_menu (mut cmd: Commands) {
    cmd.spawn((Camera2d, IsDefaultUiCamera, UiBoxShadowSamples(6)));

    cmd.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            padding: UiRect::all(Val::Percent(5.0)),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        BackgroundColor(Color::srgb(0.53, 0.0, 0.65))
    ))
    .insert(PickingBehavior::IGNORE)
    .with_children(|parent| {

        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.)),
                row_gap: Val::Px(5.),
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
        ))

        .with_children(|parent| {
            parent.spawn((
                Text::new("Intro to Labyrinthine"),
                TextFont {
                    font_size: 25.0,
                    ..default()
                },
                TextLayout::new_with_justify(JustifyText::Center),
            ));
            parent.spawn((
                Text::new("Labyrinthine is A small project made for the FBLA 2024-25 game design & simulation challenge. \nWhen playing the game, the player is put into a randomly generated maze, in which they will find themselves prompted with multiple moral dillemmas. The player's choices for each dilemma will affect their score, which is a percentage. The score percentage is directly related to how visible the path to the end of the maze is, encoraging the player to make the \"good\" decision, letting them escape the maze faster."),
            ));

            parent.spawn((
                Text::new("\nPress enter to continue"),
                TextFont {
                    font_size: 24.5,
                    ..default()
                },
                TextLayout::new_with_justify(JustifyText::Right),
            ));
        });
    });

}

fn button_interaction (keys: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keys.just_pressed(KeyCode::Enter) {
        next_state.set(GameState::World)
    }
}

fn cleanup_menu(
    mut commands: Commands,
    camera_query: Query<Entity, With<Camera2d>>,
    node_query: Query<Entity, With<Node>>,
) {
    // Despawn the camera entity
    for camera_entity in camera_query.iter() {
        commands.entity(camera_entity).despawn();
    }

    // Despawn all displayed nodes
    for node_entity in node_query.iter() {
        commands.entity(node_entity).despawn();
    }
}
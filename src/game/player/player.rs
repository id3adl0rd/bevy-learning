use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_player);
    }
}

#[derive(Component)]
pub struct Player;

fn init_player(
    mut commands: Commands
) {
    let fov = 103.0_f32;

    commands.spawn((
        Player{},
        Camera3dBundle {
            transform : Transform::from_translation(Vec3::new(0., 10., 0.)),
            projection : Projection::Perspective(PerspectiveProjection{
                fov : fov,
                ..default()
            }),
            ..default()
        }
    ));
}
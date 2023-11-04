use bevy::prelude::*;
use crate::constants::BACKGROUND_COLOR;

pub(crate) struct ShuttlePlugin;

#[derive(Resource)]
pub(crate) struct ShuttleSpeed(pub(crate) f32);
#[derive(Component)]
pub(crate) struct Shuttle;

impl Plugin for ShuttlePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BACKGROUND_COLOR)
            .insert_resource(ShuttleSpeed(0.25))
            .add_systems(Startup, spawn_shuttle)
            .add_systems(Update, shuttle_movements);
    }
}

fn spawn_shuttle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<StandardMaterial>>
)
{
    commands.spawn(
        PbrBundle {
            mesh: meshes.add(shape::Cube::new(1.).into()),
            material: material.add(Color::GRAY.into()),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        }
    ).insert(Shuttle);
}

fn shuttle_movements(
    mut shuttles: Query<(Entity, &mut Transform), With<Shuttle>>,
    keys: Res<Input<KeyCode>>,
    speed: Res<ShuttleSpeed>
)
{
    for (_, mut transform) in shuttles.iter_mut() {
        let mut m = Vec3::ZERO;
        // horizontal
        if keys.pressed(KeyCode::Q) && transform.translation.y >= -5. {
            m.y -= speed.0;
        }
        if keys.pressed(KeyCode::D) && transform.translation.y <= 5. {
            m.y += speed.0;
        }

        // vertical
        if keys.pressed(KeyCode::Z) && transform.translation.x <= 8. {
            m.x -= speed.0;
        }
        if keys.pressed(KeyCode::S) && transform.translation.x >= -8. {
            m.x += speed.0;
        }

        transform.translation += m;

        // collisions on the window :)
        if transform.translation.y < -5. {
            transform.translation.y = -5.;
        }
        if transform.translation.y > 5. {
            transform.translation.y = 5.;
        }

        if transform.translation.x < -8. {
            transform.translation.x = -8.;
        }
        if transform.translation.x > 8. {
            transform.translation.x = 8.;
        }
    }
}
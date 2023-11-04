use std::time::{Duration, Instant};
use bevy::prelude::*;
use crate::shuttle::Shuttle;

pub(crate) struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (shoot, move_projectiles));
    }
}

#[derive(Resource, Clone, Debug)]
pub(crate) struct LastShot(pub(crate) Instant);

#[derive(Component)]
pub(crate) struct Projectile;

fn shoot(
    mut commands: Commands,
    mut last_shot: Option<ResMut<LastShot>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<StandardMaterial>>,
    keys: Res<Input<KeyCode>>,
    shuttles: Query<&Transform, With<Shuttle>>
)
{
    // check if the last shot was fired less than 50ms before
    if let Some(last_shot) = last_shot.as_mut() {
        dbg!(last_shot.0.elapsed());

        if last_shot.0.elapsed() < Duration::from_millis(50) {
            return;
        }
    }

    // if the space bar is not pressed, we don't fire :)
    if !keys.pressed(KeyCode::Space) {
        return;
    }

    if let Some(last_host) = last_shot.as_mut() {
        *last_host.as_mut() = LastShot(Instant::now());
    } else {
        commands.insert_resource(LastShot(Instant::now()));
    };

    let pos = shuttles.get_single().cloned().unwrap_or(Transform::default());

    commands.spawn(
        PbrBundle {
            mesh: meshes.add(shape::Box::new(0.2, 0.1 ,0.1).into()),
            material: material.add(StandardMaterial {
                emissive: Color::rgb_u8(255, 190, 51),
                ..default()
            }),
            transform: Transform::from_translation(pos.translation),
            ..default()
        }
    ).insert(Projectile);
}

fn move_projectiles(
    mut commands: Commands,
    mut projectiles: Query<(Entity, &mut Transform), With<Projectile>>
)
{
    for (entity, mut p) in projectiles.iter_mut() {
        if p.translation.x < 9.0 {
            p.translation.x -= 0.5;
        } else {
            commands.entity(entity).remove_parent();
        }
    }
}
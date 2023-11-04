use bevy::prelude::*;
use rand::Rng;use std::time::Duration;
use crate::constants::background::STAR_COLOR;
use crate::constants::{HORIZONTAL_AXIS, MIN_MAX_VERTICAL};
use crate::shuttle::ShuttleSpeed;


pub(crate) struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StarSpawnTimer(Timer::new(Duration::from_millis(50), TimerMode::Repeating)))
            .insert_resource(StarsSpeed(1.0))
            .add_systems(Update, (spawn_stars, update_stars));
    }
}

#[derive(Resource)]
pub struct StarSpawnTimer(pub Timer);

#[derive(Resource)]
pub(crate) struct StarsSpeed(pub(crate) f32);

#[derive(Component)]
pub struct Star;

fn spawn_stars(
    mut commands: Commands,
    time: Res<Time>,
    mut star_timer: ResMut<StarSpawnTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<StandardMaterial>>
)
{
    star_timer.0.tick(time.delta());

    if !star_timer.0.finished() {
        return;
    }

    star_timer.0.reset();

    // run the logic here
    let stars_spawned = rand::thread_rng().gen_range(5..40);

    for _ in 0..stars_spawned {
        let random_horizontal_translation = (Vec3::NEG_X * Vec3::splat(10.))
            + (HORIZONTAL_AXIS * Vec3::splat(rand::thread_rng().gen_range(-50.0..50.0)));

        commands.spawn(
            PbrBundle {
                mesh: meshes.add(shape::Circle::new(0.08).into()),
                material: material.add(StandardMaterial {
                    emissive: STAR_COLOR.into(),
                    // base_color: STAR_COLOR,
                    ..default()
                }),
                transform: Transform::from_translation(random_horizontal_translation),
                ..default()
            }
        ).insert(Star)
            .insert(PointLight {
                intensity: 500.0,
                range: 4.0,
                ..Default::default()
            });
    }
}

fn update_stars(
    mut commands: Commands,
    mut stars: Query<(Entity, &mut Transform), With<Star>>,
    speed: Res<StarsSpeed>
)
{
    for (star, mut transform) in stars.iter_mut() {
        if transform.translation.z > MIN_MAX_VERTICAL.0 {
            transform.translation.x += speed.0;
        } else {
            commands.entity(star).remove_parent();
        }
    }
}
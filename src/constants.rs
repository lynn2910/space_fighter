use bevy::math::Vec3;
use bevy::prelude::{ClearColor, Color};

pub(crate) const BACKGROUND_COLOR: ClearColor = ClearColor(Color::Rgba { red: 0., green: 0., blue: 0., alpha: 1. });

pub(crate) const WINDOW_TITLE: &str = "Space Fighter";
pub(crate) const LOGICAL_WIDTH: f32 = 400.;
pub(crate) const LOGICAL_HEIGHT: f32 = 600.;

pub(crate) const HORIZONTAL_AXIS: Vec3 = Vec3::Y;
pub(crate) const VERTICAL_AXIS: Vec3 = Vec3::Z;
pub(crate) const DEPTH_AXIS: Vec3 = Vec3::X;

pub(crate) const CAMERA_DEPTH: Vec3 = Vec3 { x: 0., y: 0., z: 20. };
pub(crate) const MIN_MAX_VERTICAL: (f32, f32) = (-10., 10.);
pub(crate) const MIN_MAX_HORIZONTAL: (f32, f32) = (-2., 2.);

pub(crate) mod background {
    use bevy::prelude::Color;

    pub(crate) const STAR_COLOR: Color = Color::Rgba { red: 1., green: 1., blue: 1., alpha: 1. };
}
use std::ops::RangeInclusive;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

#[derive(Component)]
pub struct ArcBall {
    pub x: f32,
    pub y: f32,
    pub pitch_range: RangeInclusive<f32>,
    pub distance: f32,
    pub center: Vec3,
    pub rotate_sensitivity: f32,
    pub pan_sensitivity: f32,
    pub rotate_button: MouseButton,
    pub pan_button:MouseButton,
    pub enabled: bool,
}

impl Default for ArcBall {
    fn default() -> Self {
        ArcBall {
            x: 0.0,
            y: 2.0,
            pitch_range: 0.01..=3.0,
            distance: 3.0,
            center: Vec3::ZERO,
            rotate_sensitivity: 3.0,
            pan_sensitivity: 3.0,
            rotate_button: MouseButton::Middle,
            pan_button: MouseButton::Right,
            enabled: true,
        }
    }
}

pub struct ArcBallPlugin;

impl ArcBallPlugin {
    fn update_transform(
        mut query: Query<(&ArcBall, &mut Transform), (Changed<ArcBall>,  With<Camera>)>
    ) {
        for (camera, mut transform) in query.iter_mut() {
            let rot = Quat::from_axis_angle(Vec3::Y, camera.x)
                * Quat::from_axis_angle(-Vec3::X, camera.y);

            transform.translation = (rot * Vec3::Y) * camera.distance + camera.center;
            transform.look_at(camera.center, Vec3::Y);
        }
    }

    fn mouse_motion(
        time: Res<Time>,
        mut mouse_events: EventReader<MouseMotion>,
        mouse_button_input: Res<Input<MouseButton>>,
        mut query: Query<(&mut ArcBall, &mut Transform, &mut Camera)>
    ) {
        let mut delta = Vec2::ZERO;

        for event in mouse_events.iter() {
            delta += event.delta;
        }

        for (mut camera, transform, _) in query.iter_mut {
            todo!()
        }
    }
}
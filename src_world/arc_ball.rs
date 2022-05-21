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
            pitch_range: 0.01..=7.0,
            distance: 8.0,
            center: Vec3::ZERO,
            rotate_sensitivity: 0.3,
            pan_sensitivity: 1.0,
            rotate_button: MouseButton::Middle,
            pan_button: MouseButton::Right,
            enabled: true,
        }
    }
}


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

        for (mut camera, transform, _) in query.iter_mut() {
            if !camera.enabled {
                continue;
            }

            if mouse_button_input.pressed(camera.rotate_button) {
                camera.x -= delta.x * camera.rotate_sensitivity * time.delta_seconds();
                camera.y -= delta.y * camera.rotate_sensitivity * time.delta_seconds();

                camera.y = camera.y
                    .max(*camera.pitch_range.start())
                    .min(*camera.pitch_range.end());
            }

            if mouse_button_input.pressed(camera.pan_button) {
                let right = transform.rotation * -Vec3::X;
                let up = transform.rotation * Vec3::Y;
                let pan_dir = (delta.x * right + delta.y * up) * camera.pan_sensitivity
                    * time.delta_seconds();

                camera.center += pan_dir;
            }
        }
    }
}

pub struct ArcBallPlugin;

impl Plugin for ArcBallPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::mouse_motion)
            .add_system(Self::update_transform);
    }
}
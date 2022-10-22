pub use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

pub use super::components::*;

pub fn camera_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut ms_wheel_rdr: EventReader<MouseWheel>,
    mut query: Query<&mut Transform, With<MainCamera>>,
) {
    let mut transf = query.single_mut();
    let mut direction = Vec3::ZERO;
    let scale: f32 = transf.scale.x;
    if keyboard_input.pressed(KeyCode::A) {
        direction -= Vec3::new(2.0, 0.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::D) {
        direction += Vec3::new(2.0, 0.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::W) {
        direction += Vec3::new(0.0, 2.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::S) {
        direction -= Vec3::new(0.0, 2.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::Z) {
        let scale = scale + 0.5;
        transf.scale = Vec3::splat(scale);
    }

    if keyboard_input.pressed(KeyCode::X) {
        let scale = scale - 0.5;
        transf.scale = Vec3::splat(scale);
    }
    for e in ms_wheel_rdr.iter() {
        let scale = scale - e.y * 0.1;
        transf.scale = Vec3::splat(scale);
    }
    if transf.scale.x < 1.0 {
        transf.scale = Vec3::splat(1.)
    }
    transf.translation += time.delta_seconds() * direction * 100.;
}

pub fn mouse_to_world_pos(
    windows: Res<Windows>,
    camera_query: Query<(&Transform, &Camera), With<MainCamera>>,
    mut mouse_pos: ResMut<MouseWorldPos>,
) {
    let window = windows.get_primary().unwrap();
    if let Some(cursor) = window.cursor_position() {
        mouse_pos.0 = cursor_to_world(cursor, &camera_query, window)
    }
}

fn cursor_to_world(
    cursor: Vec2,
    camera_query: &Query<(&Transform, &Camera), With<MainCamera>>,
    window: &Window,
) -> Vec2 {
    let (transform, camera) = camera_query.single();

    let screen_size = Vec2::new(window.width() as f32, window.height() as f32);
    let camera_position = transform.compute_matrix();
    let projection_matrix = camera.projection_matrix();

    // Normalized device coordinate cursor position from (-1, -1, -1) to (1, 1, 1)
    let cursor_ndc = (cursor / screen_size) * 2.0 - Vec2::from([1.0, 1.0]);
    // let cursor_pos_ndc_near = cursor_ndc.extend(-1.0);
    let cursor_pos_ndc_far = cursor_ndc.extend(1.0);

    let ndc_to_world = camera_position * projection_matrix.inverse();
    // let cursor_pos_near = ndc_to_world.project_point3(cursor_pos_ndc_near);
    let cursor_pos_far = ndc_to_world.project_point3(cursor_pos_ndc_far);
    // let ray_direction = cursor_pos_far - cursor_pos_near;

    cursor_pos_far.truncate()
}

pub fn draw_ray() {}

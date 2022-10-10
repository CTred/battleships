mod components;
mod systems;

pub use bevy::prelude::*;
pub use bevy::render::camera::ScalingMode;
pub use components::*;
pub use systems::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MouseWorldPos::default())
            .add_startup_system(camera_setup)
            .add_system(camera_system)
            .add_system(mouse_to_world_pos);
    }
}

fn camera_setup(mut commands: Commands) {
    commands
        .spawn_bundle(Camera3dBundle {
            projection: OrthographicProjection {
                scale: 3.0,
                scaling_mode: ScalingMode::FixedVertical(5.0),
                ..default()
            }
            .into(),
            // camera_3d: {
            //     Camera3d {
            //         clear_color: ClearColorConfig::Custom(Color::FUCHSIA),
            //         ..default()
            //     }
            // },
            transform: { Transform::from_xyz(0., 0., 50.) },
            ..default()
        })
        .insert(MainCamera);
}

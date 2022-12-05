use renet::RenetClient;
use std::f32::consts::PI;

use bevy::{input::mouse::MouseWheel, prelude::*};

use crate::{
    map::{
        self,
        components::{HexMapEntities, Hexagon, MouseCubePos},
        HEX_CONFIG_PADDING, HEX_CONFIG_SIZE,
    },
    GameEvent,
};

use super::{AngularRot, GameObject, GridMaxRotation, MouseFollow, ObjectHover};

pub fn object_mouse_follow(
    mut query: Query<&mut Transform, With<MouseFollow>>,
    ms_pos: Res<MouseCubePos>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        let hex = Hexagon::new(HEX_CONFIG_SIZE, HEX_CONFIG_PADDING, Some(ms_pos.0), 2.0);
        let world_pos = hex.world_pos();
        let curr_ship_pos = transform.translation;
        if (curr_ship_pos.x != world_pos.x) | (curr_ship_pos.y != world_pos.y) {
            transform.translation.x = world_pos.x;
            transform.translation.y = world_pos.y;
        }
    }
}

/// listens to mousewheel events and rotate the mousefollow mesh accordingly.
pub fn object_mouse_rotate(
    mut query: Query<(&GridMaxRotation, &mut AngularRot, &mut Transform), With<MouseFollow>>,
    mut scroll_ev: EventReader<MouseWheel>,
) {
    if let Ok((grid_max_rot, mut angular_rot, mut transform)) = query.get_single_mut() {
        for ev in scroll_ev.iter() {
            let direction = ev.y.signum() as i32;
            angular_rot.0 = (angular_rot.0 + direction) % grid_max_rot.0 as i32;
            transform.rotate_local_z(direction as f32 * PI * 2.0 / grid_max_rot.0 as f32);
        }
    }
}

pub fn object_mouse_place(
    mut query: Query<(&GameObject, &AngularRot), With<MouseFollow>>,
    ms_input: Res<Input<MouseButton>>,
    ms_pos: Res<MouseCubePos>,
    mut client: ResMut<RenetClient>,
) {
    if ms_input.just_pressed(MouseButton::Left) {
        if let Ok((game_object, rotation)) = query.get_single_mut() {
            let event = GameEvent::ShipPlaced {
                player_id: client.client_id(),
                at: ms_pos.0,
                rotation: rotation.0,
                ship_type: game_object.clone(),
            };
            client.send_message(0, bincode::serialize(&event).unwrap());
        }
    }
}

pub fn object_mouse_hover(
    hex_board: Res<HexMapEntities>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ms_coord: Res<MouseCubePos>,
    object: Query<(&GameObject, &AngularRot, &Transform), With<MouseFollow>>,
    hex_query: Query<(Entity, &Transform), With<ObjectHover>>,
    kb_input: Res<Input<KeyCode>>,
) {
    if let Ok((object, angular_rot, _)) = object.get_single() {
        // the updated object position in all hexes coord
        let coords = super::get_object_all_coords(object, angular_rot.0, &ms_coord.0);
        if kb_input.just_pressed(KeyCode::Space) {
            dbg!(&coords.last().unwrap());
        }

        // check if there is any spawned hex that no longer matches the curr position and despawn
        let mut prev_coords = Vec::new();
        for (e, transf) in &hex_query {
            let coord = map::components::world_pos_to_coordinates(
                HEX_CONFIG_SIZE + HEX_CONFIG_PADDING,
                Vec2 {
                    x: transf.translation.x,
                    y: transf.translation.y,
                },
            );
            prev_coords.push(coord);
            if !coords.iter().any(|c| c == &coord) {
                commands.entity(e).despawn_recursive();
            }
        }

        // check if there is any coordinate missing a hex hover to spawn
        for coord in coords.iter() {
            // TODO: implement proper layering system;
            if !prev_coords.iter().any(|c| c == coord) {
                let is_entity = match hex_board.0.get(&coord) {
                    Some(_) => true,
                    None => false,
                };
                let hex = Hexagon::new(HEX_CONFIG_SIZE, HEX_CONFIG_PADDING, Some(*coord), 1.1);
                let hex_pos = hex.world_pos();
                commands
                    .spawn(MaterialMeshBundle {
                        mesh: meshes.add(hex.to_mesh()),
                        material: materials.add(StandardMaterial {
                            base_color: Color::rgba(0.87, 0.57, 0.57, 0.8),
                            unlit: true,
                            alpha_mode: AlphaMode::Blend,
                            ..default()
                        }),
                        transform: Transform::from_xyz(hex_pos.x, hex_pos.y, hex_pos.z),
                        visibility: Visibility {
                            is_visible: is_entity,
                        },
                        ..default()
                    })
                    .insert(ObjectHover);
            }
        }
    }
}
